//! 微服务架构示例 - 展示如何使用Rust构建微服务系统
//! 
//! 本示例展示了如何构建一个完整的微服务系统，包括：
//! - 服务发现与注册
//! - API网关
//! - 负载均衡
//! - 分布式配置管理
//! - 服务间通信
//! - 容错与重试机制

use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};
use serde::{Deserialize, Serialize};
use tokio::sync::{mpsc, oneshot};
use tokio::time::{sleep, timeout};
use anyhow::{Result, Context, bail};
use uuid::Uuid;

/// 服务注册中心
/// 
/// 负责管理所有微服务的注册、发现和健康检查
#[derive(Debug, Clone)]
pub struct ServiceRegistry {
    services: Arc<RwLock<HashMap<String, Vec<ServiceInstance>>>>,
    health_checkers: Arc<RwLock<HashMap<String, Box<dyn HealthChecker + Send + Sync>>>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceInstance {
    pub id: String,
    pub name: String,
    pub host: String,
    pub port: u16,
    pub metadata: HashMap<String, String>,
    pub health_status: HealthStatus,
    pub last_heartbeat: Instant,
    pub version: String,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum HealthStatus {
    Healthy,
    Unhealthy,
    Unknown,
}

/// 健康检查器trait
#[async_trait::async_trait]
pub trait HealthChecker: Send + Sync {
    async fn check(&self, instance: &ServiceInstance) -> HealthStatus;
}

/// HTTP健康检查器
pub struct HttpHealthChecker {
    timeout: Duration,
}

impl HttpHealthChecker {
    pub fn new(timeout: Duration) -> Self {
        Self { timeout }
    }
}

#[async_trait::async_trait]
impl HealthChecker for HttpHealthChecker {
    async fn check(&self, instance: &ServiceInstance) -> HealthStatus {
        let url = format!("http://{}:{}/health", instance.host, instance.port);
        
        match timeout(self.timeout, reqwest::get(&url)).await {
            Ok(Ok(response)) => {
                if response.status().is_success() {
                    HealthStatus::Healthy
                } else {
                    HealthStatus::Unhealthy
                }
            }
            _ => HealthStatus::Unhealthy,
        }
    }
}

impl ServiceRegistry {
    pub fn new() -> Self {
        Self {
            services: Arc::new(RwLock::new(HashMap::new())),
            health_checkers: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    /// 注册服务实例
    pub async fn register_service(&self, instance: ServiceInstance) -> Result<()> {
        let mut services = self.services.write().unwrap();
        let service_name = instance.name.clone();
        
        services
            .entry(service_name)
            .or_insert_with(Vec::new)
            .push(instance);
        
        println!("✅ 服务实例已注册: {}", service_name);
        Ok(())
    }
    
    /// 注销服务实例
    pub async fn unregister_service(&self, service_name: &str, instance_id: &str) -> Result<()> {
        let mut services = self.services.write().unwrap();
        
        if let Some(instances) = services.get_mut(service_name) {
            instances.retain(|instance| instance.id != instance_id);
            
            if instances.is_empty() {
                services.remove(service_name);
            }
        }
        
        println!("❌ 服务实例已注销: {} - {}", service_name, instance_id);
        Ok(())
    }
    
    /// 发现服务实例
    pub fn discover_services(&self, service_name: &str) -> Vec<ServiceInstance> {
        let services = self.services.read().unwrap();
        
        services
            .get(service_name)
            .map(|instances| {
                instances
                    .iter()
                    .filter(|instance| instance.health_status == HealthStatus::Healthy)
                    .cloned()
                    .collect()
            })
            .unwrap_or_default()
    }
    
    /// 获取单个健康服务实例
    pub fn get_service_instance(&self, service_name: &str) -> Option<ServiceInstance> {
        let services = self.services.read().unwrap();
        
        services
            .get(service_name)?
            .iter()
            .find(|instance| instance.health_status == HealthStatus::Healthy)
            .cloned()
    }
    
    /// 设置健康检查器
    pub fn set_health_checker(&self, service_name: String, checker: Box<dyn HealthChecker + Send + Sync>) {
        let mut checkers = self.health_checkers.write().unwrap();
        checkers.insert(service_name, checker);
    }
    
    /// 执行健康检查
    pub async fn perform_health_check(&self, service_name: &str) -> Result<()> {
        let checkers = self.health_checkers.read().unwrap();
        let checker = checkers.get(service_name);
        
        if let Some(checker) = checker {
            let mut services = self.services.write().unwrap();
            
            if let Some(instances) = services.get_mut(service_name) {
                for instance in instances.iter_mut() {
                    let health_status = checker.check(instance).await;
                    instance.health_status = health_status;
                    instance.last_heartbeat = Instant::now();
                }
            }
        }
        
        Ok(())
    }
    
    /// 启动定期健康检查
    pub async fn start_health_checking(&self, interval: Duration) {
        let registry = self.clone();
        
        tokio::spawn(async move {
            let mut interval_timer = tokio::time::interval(interval);
            
            loop {
                interval_timer.tick().await;
                
                let service_names: Vec<String> = {
                    let services = registry.services.read().unwrap();
                    services.keys().cloned().collect()
                };
                
                for service_name in service_names {
                    if let Err(e) = registry.perform_health_check(&service_name).await {
                        eprintln!("健康检查失败 {}: {}", service_name, e);
                    }
                }
            }
        });
    }
}

/// 负载均衡器
/// 
/// 提供多种负载均衡策略
#[derive(Debug, Clone)]
pub struct LoadBalancer {
    strategy: LoadBalancingStrategy,
    service_registry: Arc<ServiceRegistry>,
}

#[derive(Debug, Clone)]
pub enum LoadBalancingStrategy {
    RoundRobin,
    LeastConnections,
    Random,
    WeightedRoundRobin,
}

impl LoadBalancer {
    pub fn new(strategy: LoadBalancingStrategy, service_registry: Arc<ServiceRegistry>) -> Self {
        Self {
            strategy,
            service_registry,
        }
    }
    
    /// 选择服务实例
    pub fn select_instance(&self, service_name: &str) -> Option<ServiceInstance> {
        let instances = self.service_registry.discover_services(service_name);
        
        if instances.is_empty() {
            return None;
        }
        
        match self.strategy {
            LoadBalancingStrategy::RoundRobin => self.round_robin_selection(instances),
            LoadBalancingStrategy::LeastConnections => self.least_connections_selection(instances),
            LoadBalancingStrategy::Random => self.random_selection(instances),
            LoadBalancingStrategy::WeightedRoundRobin => self.weighted_round_robin_selection(instances),
        }
    }
    
    fn round_robin_selection(&self, instances: Vec<ServiceInstance>) -> Option<ServiceInstance> {
        // 简化实现，实际中应该维护状态
        instances.into_iter().next()
    }
    
    fn least_connections_selection(&self, instances: Vec<ServiceInstance>) -> Option<ServiceInstance> {
        // 简化实现，实际中应该跟踪连接数
        instances.into_iter().next()
    }
    
    fn random_selection(&self, instances: Vec<ServiceInstance>) -> Option<ServiceInstance> {
        use rand::seq::SliceRandom;
        let mut rng = rand::thread_rng();
        instances.into_iter().choose(&mut rng)
    }
    
    fn weighted_round_robin_selection(&self, instances: Vec<ServiceInstance>) -> Option<ServiceInstance> {
        // 简化实现，实际中应该考虑权重
        instances.into_iter().next()
    }
}

/// API网关
/// 
/// 作为微服务系统的入口点，负责路由、认证、限流等
#[derive(Debug)]
pub struct ApiGateway {
    routes: Arc<RwLock<HashMap<String, Route>>>,
    load_balancer: Arc<LoadBalancer>,
    rate_limiter: Arc<RateLimiter>,
    auth_service: Arc<AuthService>,
}

#[derive(Debug, Clone)]
pub struct Route {
    pub path: String,
    pub method: HttpMethod,
    pub service_name: String,
    pub timeout: Duration,
    pub retry_count: u32,
    pub circuit_breaker: Option<CircuitBreaker>,
}

#[derive(Debug, Clone)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
}

#[derive(Debug, Clone)]
pub struct CircuitBreaker {
    pub failure_threshold: u32,
    pub recovery_timeout: Duration,
    pub failure_count: u32,
    pub last_failure_time: Option<Instant>,
    pub state: CircuitBreakerState,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CircuitBreakerState {
    Closed,
    Open,
    HalfOpen,
}

impl CircuitBreaker {
    pub fn new(failure_threshold: u32, recovery_timeout: Duration) -> Self {
        Self {
            failure_threshold,
            recovery_timeout,
            failure_count: 0,
            last_failure_time: None,
            state: CircuitBreakerState::Closed,
        }
    }
    
    pub fn can_execute(&mut self) -> bool {
        match self.state {
            CircuitBreakerState::Closed => true,
            CircuitBreakerState::Open => {
                if let Some(last_failure) = self.last_failure_time {
                    if Instant::now().duration_since(last_failure) >= self.recovery_timeout {
                        self.state = CircuitBreakerState::HalfOpen;
                        true
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
            CircuitBreakerState::HalfOpen => true,
        }
    }
    
    pub fn record_success(&mut self) {
        self.failure_count = 0;
        self.state = CircuitBreakerState::Closed;
    }
    
    pub fn record_failure(&mut self) {
        self.failure_count += 1;
        self.last_failure_time = Some(Instant::now());
        
        if self.failure_count >= self.failure_threshold {
            self.state = CircuitBreakerState::Open;
        }
    }
}

impl ApiGateway {
    pub fn new(
        load_balancer: Arc<LoadBalancer>,
        rate_limiter: Arc<RateLimiter>,
        auth_service: Arc<AuthService>,
    ) -> Self {
        Self {
            routes: Arc::new(RwLock::new(HashMap::new())),
            load_balancer,
            rate_limiter,
            auth_service,
        }
    }
    
    /// 添加路由
    pub fn add_route(&self, route: Route) {
        let key = format!("{}:{}", route.method.clone(), route.path.clone());
        let mut routes = self.routes.write().unwrap();
        routes.insert(key, route);
    }
    
    /// 处理请求
    pub async fn handle_request(&self, request: HttpRequest) -> Result<HttpResponse> {
        // 1. 限流检查
        if !self.rate_limiter.allow_request(&request.client_id).await? {
            return Ok(HttpResponse {
                status: 429,
                headers: HashMap::new(),
                body: "Rate limit exceeded".to_string(),
            });
        }
        
        // 2. 认证检查
        if !self.auth_service.authenticate(&request).await? {
            return Ok(HttpResponse {
                status: 401,
                headers: HashMap::new(),
                body: "Unauthorized".to_string(),
            });
        }
        
        // 3. 查找路由
        let route_key = format!("{}:{}", request.method, request.path);
        let route = {
            let routes = self.routes.read().unwrap();
            routes.get(&route_key).cloned()
        };
        
        let route = match route {
            Some(route) => route,
            None => {
                return Ok(HttpResponse {
                    status: 404,
                    headers: HashMap::new(),
                    body: "Route not found".to_string(),
                });
            }
        };
        
        // 4. 熔断器检查
        if let Some(ref mut circuit_breaker) = route.circuit_breaker.as_ref() {
            if !circuit_breaker.can_execute() {
                return Ok(HttpResponse {
                    status: 503,
                    headers: HashMap::new(),
                    body: "Service unavailable".to_string(),
                });
            }
        }
        
        // 5. 选择服务实例
        let instance = match self.load_balancer.select_instance(&route.service_name) {
            Some(instance) => instance,
            None => {
                return Ok(HttpResponse {
                    status: 503,
                    headers: HashMap::new(),
                    body: "No healthy instances available".to_string(),
                });
            }
        };
        
        // 6. 转发请求
        let mut retry_count = 0;
        loop {
            match self.forward_request(&instance, &request, route.timeout).await {
                Ok(response) => {
                    if let Some(ref mut circuit_breaker) = route.circuit_breaker.as_ref() {
                        circuit_breaker.record_success();
                    }
                    return Ok(response);
                }
                Err(e) => {
                    if let Some(ref mut circuit_breaker) = route.circuit_breaker.as_ref() {
                        circuit_breaker.record_failure();
                    }
                    
                    retry_count += 1;
                    if retry_count >= route.retry_count {
                        return Err(e);
                    }
                    
                    sleep(Duration::from_millis(100 * retry_count)).await;
                }
            }
        }
    }
    
    async fn forward_request(
        &self,
        instance: &ServiceInstance,
        request: &HttpRequest,
        timeout: Duration,
    ) -> Result<HttpResponse> {
        let url = format!("http://{}:{}{}", instance.host, instance.port, request.path);
        
        let client = reqwest::Client::new();
        let mut req_builder = match request.method {
            HttpMethod::GET => client.get(&url),
            HttpMethod::POST => client.post(&url),
            HttpMethod::PUT => client.put(&url),
            HttpMethod::DELETE => client.delete(&url),
            HttpMethod::PATCH => client.patch(&url),
        };
        
        // 添加请求头
        for (key, value) in &request.headers {
            req_builder = req_builder.header(key, value);
        }
        
        // 添加请求体
        if !request.body.is_empty() {
            req_builder = req_builder.body(request.body.clone());
        }
        
        let response = timeout(timeout, req_builder.send()).await
            .context("Request timeout")?
            .context("Request failed")?;
        
        let status = response.status().as_u16();
        let headers = response
            .headers()
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_str().unwrap_or("").to_string()))
            .collect();
        
        let body = response.text().await?;
        
        Ok(HttpResponse {
            status,
            headers,
            body,
        })
    }
}

/// 限流器
#[derive(Debug)]
pub struct RateLimiter {
    limits: Arc<RwLock<HashMap<String, RateLimit>>>,
}

#[derive(Debug, Clone)]
pub struct RateLimit {
    pub requests_per_minute: u32,
    pub requests: Vec<Instant>,
}

impl RateLimiter {
    pub fn new() -> Self {
        Self {
            limits: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    pub async fn allow_request(&self, client_id: &str) -> Result<bool> {
        let mut limits = self.limits.write().unwrap();
        let now = Instant::now();
        let one_minute_ago = now - Duration::from_secs(60);
        
        let limit = limits.entry(client_id.to_string()).or_insert(RateLimit {
            requests_per_minute: 100, // 默认限制
            requests: Vec::new(),
        });
        
        // 清理过期请求
        limit.requests.retain(|&time| time > one_minute_ago);
        
        // 检查是否超过限制
        if limit.requests.len() >= limit.requests_per_minute as usize {
            Ok(false)
        } else {
            limit.requests.push(now);
            Ok(true)
        }
    }
}

/// 认证服务
#[derive(Debug)]
pub struct AuthService {
    jwt_secret: String,
    user_store: Arc<RwLock<HashMap<String, User>>>,
}

#[derive(Debug, Clone)]
pub struct User {
    pub id: String,
    pub username: String,
    pub roles: Vec<String>,
    pub permissions: Vec<String>,
}

impl AuthService {
    pub fn new(jwt_secret: String) -> Self {
        Self {
            jwt_secret,
            user_store: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    pub async fn authenticate(&self, request: &HttpRequest) -> Result<bool> {
        // 简化实现，实际中应该验证JWT token
        if let Some(auth_header) = request.headers.get("Authorization") {
            if auth_header.starts_with("Bearer ") {
                return Ok(true);
            }
        }
        
        Ok(false)
    }
    
    pub async fn login(&self, username: &str, password: &str) -> Result<String> {
        // 简化实现，实际中应该验证密码
        let user = User {
            id: Uuid::new_v4().to_string(),
            username: username.to_string(),
            roles: vec!["user".to_string()],
            permissions: vec!["read".to_string(), "write".to_string()],
        };
        
        let mut user_store = self.user_store.write().unwrap();
        user_store.insert(user.id.clone(), user);
        
        // 生成JWT token（简化实现）
        let token = format!("token_{}", Uuid::new_v4());
        Ok(token)
    }
}

/// HTTP请求和响应结构
#[derive(Debug, Clone)]
pub struct HttpRequest {
    pub method: HttpMethod,
    pub path: String,
    pub headers: HashMap<String, String>,
    pub body: String,
    pub client_id: String,
}

#[derive(Debug, Clone)]
pub struct HttpResponse {
    pub status: u16,
    pub headers: HashMap<String, String>,
    pub body: String,
}

/// 微服务示例
#[derive(Debug)]
pub struct UserService {
    pub id: String,
    pub registry: Arc<ServiceRegistry>,
    pub port: u16,
}

impl UserService {
    pub fn new(registry: Arc<ServiceRegistry>, port: u16) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            registry,
            port,
        }
    }
    
    pub async fn start(&self) -> Result<()> {
        // 注册服务
        let instance = ServiceInstance {
            id: self.id.clone(),
            name: "user-service".to_string(),
            host: "localhost".to_string(),
            port: self.port,
            metadata: HashMap::new(),
            health_status: HealthStatus::Healthy,
            last_heartbeat: Instant::now(),
            version: "1.0.0".to_string(),
            tags: vec!["user".to_string(), "api".to_string()],
        };
        
        self.registry.register_service(instance).await?;
        
        // 启动HTTP服务器（简化实现）
        println!("🚀 用户服务启动在端口 {}", self.port);
        
        // 启动健康检查端点
        self.start_health_endpoint().await?;
        
        Ok(())
    }
    
    async fn start_health_endpoint(&self) -> Result<()> {
        // 简化实现，实际中应该启动真正的HTTP服务器
        println!("🏥 健康检查端点已启动");
        Ok(())
    }
}

/// 微服务系统
#[derive(Debug)]
pub struct MicroservicesSystem {
    pub registry: Arc<ServiceRegistry>,
    pub gateway: Arc<ApiGateway>,
    pub services: Vec<Arc<UserService>>,
}

impl MicroservicesSystem {
    pub fn new() -> Self {
        let registry = Arc::new(ServiceRegistry::new());
        let load_balancer = Arc::new(LoadBalancer::new(
            LoadBalancingStrategy::RoundRobin,
            registry.clone(),
        ));
        let rate_limiter = Arc::new(RateLimiter::new());
        let auth_service = Arc::new(AuthService::new("secret-key".to_string()));
        let gateway = Arc::new(ApiGateway::new(load_balancer, rate_limiter, auth_service));
        
        Self {
            registry,
            gateway,
            services: Vec::new(),
        }
    }
    
    pub async fn start(&self) -> Result<()> {
        println!("🚀 启动微服务系统...");
        
        // 启动服务注册中心
        self.registry.start_health_checking(Duration::from_secs(30)).await;
        
        // 启动用户服务
        let user_service = Arc::new(UserService::new(self.registry.clone(), 8080));
        user_service.start().await?;
        self.services.push(user_service);
        
        // 配置API网关路由
        self.gateway.add_route(Route {
            path: "/api/users".to_string(),
            method: HttpMethod::GET,
            service_name: "user-service".to_string(),
            timeout: Duration::from_secs(30),
            retry_count: 3,
            circuit_breaker: Some(CircuitBreaker::new(5, Duration::from_secs(60))),
        });
        
        println!("✅ 微服务系统启动完成");
        Ok(())
    }
    
    pub async fn handle_request(&self, request: HttpRequest) -> Result<HttpResponse> {
        self.gateway.handle_request(request).await
    }
}

/// 主函数 - 演示微服务系统的使用
#[tokio::main]
async fn main() -> Result<()> {
    // 初始化日志
    env_logger::init();
    
    // 创建微服务系统
    let system = Arc::new(MicroservicesSystem::new());
    
    // 启动系统
    system.start().await?;
    
    // 模拟请求
    let request = HttpRequest {
        method: HttpMethod::GET,
        path: "/api/users".to_string(),
        headers: {
            let mut headers = HashMap::new();
            headers.insert("Authorization".to_string(), "Bearer token123".to_string());
            headers
        },
        body: String::new(),
        client_id: "client1".to_string(),
    };
    
    let response = system.handle_request(request).await?;
    println!("📨 响应: {:?}", response);
    
    // 保持系统运行
    tokio::signal::ctrl_c().await?;
    println!("🛑 系统已停止");
    
    Ok(())
}
