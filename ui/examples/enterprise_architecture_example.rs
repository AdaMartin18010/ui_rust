//! 企业级应用架构示例 - 展示大型应用的架构设计
//! 
//! 本示例展示了如何使用Rust 1.90和跨平台UI框架构建企业级应用，
//! 包括微服务架构、状态管理、错误处理、性能监控等企业级特性。

use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};
use std::time::{Duration, Instant};
use serde::{Deserialize, Serialize};
use tokio::sync::{broadcast, mpsc};
use anyhow::{Result, Context};

/// 应用配置结构体
/// 
/// 企业级应用通常需要复杂的配置管理
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    /// 应用基本信息
    pub app_info: AppInfo,
    
    /// 数据库配置
    pub database: DatabaseConfig,
    
    /// 缓存配置
    pub cache: CacheConfig,
    
    /// 日志配置
    pub logging: LoggingConfig,
    
    /// 性能监控配置
    pub monitoring: MonitoringConfig,
    
    /// 安全配置
    pub security: SecurityConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppInfo {
    pub name: String,
    pub version: String,
    pub environment: Environment,
    pub debug_mode: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Environment {
    Development,
    Staging,
    Production,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
    pub connection_timeout: Duration,
    pub pool_size: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheConfig {
    pub redis_url: String,
    pub ttl: Duration,
    pub max_size: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    pub level: LogLevel,
    pub format: LogFormat,
    pub output: LogOutput,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LogFormat {
    Json,
    Text,
    Compact,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LogOutput {
    Console,
    File(String),
    Remote(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringConfig {
    pub metrics_endpoint: String,
    pub health_check_interval: Duration,
    pub performance_thresholds: PerformanceThresholds,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceThresholds {
    pub max_response_time: Duration,
    pub max_memory_usage: f64,
    pub max_cpu_usage: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    pub jwt_secret: String,
    pub session_timeout: Duration,
    pub max_login_attempts: u32,
    pub password_policy: PasswordPolicy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasswordPolicy {
    pub min_length: usize,
    pub require_uppercase: bool,
    pub require_lowercase: bool,
    pub require_numbers: bool,
    pub require_special_chars: bool,
}

/// 应用状态管理器
/// 
/// 企业级应用需要复杂的状态管理，包括：
/// - 用户会话管理
/// - 业务数据缓存
/// - 系统状态监控
/// - 错误状态跟踪
#[derive(Debug)]
pub struct AppStateManager {
    /// 用户会话存储
    sessions: Arc<RwLock<HashMap<String, UserSession>>>,
    
    /// 业务数据缓存
    cache: Arc<RwLock<HashMap<String, CachedData>>>,
    
    /// 系统状态
    system_state: Arc<RwLock<SystemState>>,
    
    /// 性能指标
    metrics: Arc<Mutex<PerformanceMetrics>>,
    
    /// 错误日志
    error_log: Arc<Mutex<Vec<AppError>>>,
    
    /// 配置信息
    config: Arc<AppConfig>,
    
    /// 事件广播器
    event_broadcaster: broadcast::Sender<AppEvent>,
}

#[derive(Debug, Clone)]
pub struct UserSession {
    pub user_id: String,
    pub username: String,
    pub roles: Vec<String>,
    pub permissions: Vec<String>,
    pub created_at: Instant,
    pub last_activity: Instant,
    pub is_active: bool,
}

#[derive(Debug, Clone)]
pub struct CachedData {
    pub data: serde_json::Value,
    pub expires_at: Instant,
    pub access_count: u64,
}

#[derive(Debug, Clone)]
pub struct SystemState {
    pub status: SystemStatus,
    pub health_score: f64,
    pub active_connections: u32,
    pub memory_usage: f64,
    pub cpu_usage: f64,
    pub last_updated: Instant,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SystemStatus {
    Healthy,
    Warning,
    Critical,
    Maintenance,
}

#[derive(Debug, Default)]
pub struct PerformanceMetrics {
    pub request_count: u64,
    pub response_time_avg: Duration,
    pub error_count: u64,
    pub cache_hit_rate: f64,
    pub memory_usage_peak: f64,
    pub cpu_usage_peak: f64,
}

#[derive(Debug, Clone)]
pub struct AppError {
    pub error_type: ErrorType,
    pub message: String,
    pub timestamp: Instant,
    pub context: HashMap<String, String>,
    pub stack_trace: Option<String>,
}

#[derive(Debug, Clone)]
pub enum ErrorType {
    Authentication,
    Authorization,
    Validation,
    Database,
    Network,
    Internal,
    External,
}

#[derive(Debug, Clone)]
pub enum AppEvent {
    UserLogin { user_id: String },
    UserLogout { user_id: String },
    DataUpdated { key: String, value: serde_json::Value },
    SystemAlert { level: AlertLevel, message: String },
    PerformanceThreshold { metric: String, value: f64 },
}

#[derive(Debug, Clone)]
pub enum AlertLevel {
    Info,
    Warning,
    Error,
    Critical,
}

impl AppStateManager {
    /// 创建新的应用状态管理器
    pub fn new(config: AppConfig) -> Self {
        let (event_broadcaster, _) = broadcast::channel(1000);
        
        Self {
            sessions: Arc::new(RwLock::new(HashMap::new())),
            cache: Arc::new(RwLock::new(HashMap::new())),
            system_state: Arc::new(RwLock::new(SystemState {
                status: SystemStatus::Healthy,
                health_score: 100.0,
                active_connections: 0,
                memory_usage: 0.0,
                cpu_usage: 0.0,
                last_updated: Instant::now(),
            })),
            metrics: Arc::new(Mutex::new(PerformanceMetrics::default())),
            error_log: Arc::new(Mutex::new(Vec::new())),
            config: Arc::new(config),
            event_broadcaster,
        }
    }
    
    /// 创建用户会话
    pub async fn create_session(
        &self,
        user_id: String,
        username: String,
        roles: Vec<String>,
        permissions: Vec<String>,
    ) -> Result<String> {
        let session_id = uuid::Uuid::new_v4().to_string();
        let session = UserSession {
            user_id: user_id.clone(),
            username,
            roles,
            permissions,
            created_at: Instant::now(),
            last_activity: Instant::now(),
            is_active: true,
        };
        
        // 存储会话
        {
            let mut sessions = self.sessions.write().unwrap();
            sessions.insert(session_id.clone(), session);
        }
        
        // 发送事件
        let _ = self.event_broadcaster.send(AppEvent::UserLogin { user_id });
        
        Ok(session_id)
    }
    
    /// 验证会话
    pub fn validate_session(&self, session_id: &str) -> Result<UserSession> {
        let sessions = self.sessions.read().unwrap();
        let session = sessions.get(session_id)
            .ok_or_else(|| anyhow::anyhow!("会话不存在"))?;
        
        // 检查会话是否过期
        let now = Instant::now();
        let session_timeout = self.config.security.session_timeout;
        
        if now.duration_since(session.last_activity) > session_timeout {
            return Err(anyhow::anyhow!("会话已过期"));
        }
        
        if !session.is_active {
            return Err(anyhow::anyhow!("会话已失效"));
        }
        
        Ok(session.clone())
    }
    
    /// 缓存数据
    pub fn cache_data(&self, key: String, data: serde_json::Value, ttl: Duration) {
        let cached_data = CachedData {
            data,
            expires_at: Instant::now() + ttl,
            access_count: 0,
        };
        
        let mut cache = self.cache.write().unwrap();
        cache.insert(key, cached_data);
    }
    
    /// 获取缓存数据
    pub fn get_cached_data(&self, key: &str) -> Option<serde_json::Value> {
        let mut cache = self.cache.write().unwrap();
        
        if let Some(cached_data) = cache.get_mut(key) {
            // 检查是否过期
            if Instant::now() > cached_data.expires_at {
                cache.remove(key);
                return None;
            }
            
            // 增加访问计数
            cached_data.access_count += 1;
            
            // 更新缓存命中率
            self.update_cache_hit_rate(true);
            
            return Some(cached_data.data.clone());
        }
        
        self.update_cache_hit_rate(false);
        None
    }
    
    /// 更新系统状态
    pub async fn update_system_state(&self) -> Result<()> {
        let memory_usage = self.get_memory_usage().await?;
        let cpu_usage = self.get_cpu_usage().await?;
        let active_connections = self.get_active_connections().await?;
        
        let health_score = self.calculate_health_score(memory_usage, cpu_usage);
        let status = self.determine_system_status(health_score);
        
        let mut system_state = self.system_state.write().unwrap();
        system_state.memory_usage = memory_usage;
        system_state.cpu_usage = cpu_usage;
        system_state.active_connections = active_connections;
        system_state.health_score = health_score;
        system_state.status = status.clone();
        system_state.last_updated = Instant::now();
        
        // 检查性能阈值
        self.check_performance_thresholds(memory_usage, cpu_usage).await;
        
        // 如果状态变化，发送事件
        if status != SystemStatus::Healthy {
            let _ = self.event_broadcaster.send(AppEvent::SystemAlert {
                level: match status {
                    SystemStatus::Warning => AlertLevel::Warning,
                    SystemStatus::Critical => AlertLevel::Error,
                    SystemStatus::Maintenance => AlertLevel::Info,
                    _ => AlertLevel::Info,
                },
                message: format!("系统状态: {:?}", status),
            });
        }
        
        Ok(())
    }
    
    /// 记录错误
    pub fn log_error(&self, error_type: ErrorType, message: String, context: HashMap<String, String>) {
        let error = AppError {
            error_type,
            message,
            timestamp: Instant::now(),
            context,
            stack_trace: None, // 在实际应用中，这里会包含堆栈跟踪
        };
        
        let mut error_log = self.error_log.lock().unwrap();
        error_log.push(error);
        
        // 限制错误日志大小
        if error_log.len() > 10000 {
            error_log.drain(0..1000);
        }
    }
    
    /// 获取性能指标
    pub fn get_metrics(&self) -> PerformanceMetrics {
        self.metrics.lock().unwrap().clone()
    }
    
    /// 获取系统状态
    pub fn get_system_state(&self) -> SystemState {
        self.system_state.read().unwrap().clone()
    }
    
    /// 订阅应用事件
    pub fn subscribe_events(&self) -> broadcast::Receiver<AppEvent> {
        self.event_broadcaster.subscribe()
    }
    
    /// 清理过期数据
    pub async fn cleanup_expired_data(&self) {
        let now = Instant::now();
        
        // 清理过期会话
        {
            let mut sessions = self.sessions.write().unwrap();
            sessions.retain(|_, session| {
                now.duration_since(session.last_activity) <= self.config.security.session_timeout
            });
        }
        
        // 清理过期缓存
        {
            let mut cache = self.cache.write().unwrap();
            cache.retain(|_, cached_data| now <= cached_data.expires_at);
        }
    }
    
    // 私有辅助方法
    async fn get_memory_usage(&self) -> Result<f64> {
        // 在实际应用中，这里会调用系统API获取内存使用情况
        Ok(45.0) // 模拟数据
    }
    
    async fn get_cpu_usage(&self) -> Result<f64> {
        // 在实际应用中，这里会调用系统API获取CPU使用情况
        Ok(25.0) // 模拟数据
    }
    
    async fn get_active_connections(&self) -> Result<u32> {
        let sessions = self.sessions.read().unwrap();
        Ok(sessions.len() as u32)
    }
    
    fn calculate_health_score(&self, memory_usage: f64, cpu_usage: f64) -> f64 {
        let memory_score = (100.0 - memory_usage).max(0.0);
        let cpu_score = (100.0 - cpu_usage).max(0.0);
        (memory_score + cpu_score) / 2.0
    }
    
    fn determine_system_status(&self, health_score: f64) -> SystemStatus {
        match health_score {
            score if score >= 90.0 => SystemStatus::Healthy,
            score if score >= 70.0 => SystemStatus::Warning,
            score if score >= 50.0 => SystemStatus::Critical,
            _ => SystemStatus::Maintenance,
        }
    }
    
    async fn check_performance_thresholds(&self, memory_usage: f64, cpu_usage: f64) {
        let thresholds = &self.config.monitoring.performance_thresholds;
        
        if memory_usage > thresholds.max_memory_usage {
            let _ = self.event_broadcaster.send(AppEvent::PerformanceThreshold {
                metric: "memory_usage".to_string(),
                value: memory_usage,
            });
        }
        
        if cpu_usage > thresholds.max_cpu_usage {
            let _ = self.event_broadcaster.send(AppEvent::PerformanceThreshold {
                metric: "cpu_usage".to_string(),
                value: cpu_usage,
            });
        }
    }
    
    fn update_cache_hit_rate(&self, hit: bool) {
        let mut metrics = self.metrics.lock().unwrap();
        // 这里应该实现更复杂的缓存命中率计算
        // 简化版本仅用于演示
    }
}

/// 企业级应用主结构体
/// 
/// 展示如何组织大型应用的核心组件
pub struct EnterpriseApp {
    /// 配置管理器
    config_manager: Arc<ConfigManager>,
    
    /// 状态管理器
    state_manager: Arc<AppStateManager>,
    
    /// 服务容器
    service_container: Arc<ServiceContainer>,
    
    /// 事件处理器
    event_processor: Arc<EventProcessor>,
    
    /// 性能监控器
    performance_monitor: Arc<PerformanceMonitor>,
    
    /// 健康检查器
    health_checker: Arc<HealthChecker>,
    
    /// 应用生命周期管理器
    lifecycle_manager: Arc<LifecycleManager>,
}

/// 配置管理器
#[derive(Debug)]
pub struct ConfigManager {
    config: Arc<RwLock<AppConfig>>,
    watchers: Vec<Box<dyn ConfigWatcher>>,
}

impl ConfigManager {
    pub fn new(config: AppConfig) -> Self {
        Self {
            config: Arc::new(RwLock::new(config)),
            watchers: Vec::new(),
        }
    }
    
    pub fn get_config(&self) -> AppConfig {
        self.config.read().unwrap().clone()
    }
    
    pub fn update_config<F>(&self, updater: F) -> Result<()>
    where
        F: FnOnce(&mut AppConfig),
    {
        let mut config = self.config.write().unwrap();
        updater(&mut config);
        self.notify_watchers();
        Ok(())
    }
    
    fn notify_watchers(&self) {
        for watcher in &self.watchers {
            watcher.on_config_changed();
        }
    }
}

/// 配置监听器trait
pub trait ConfigWatcher: Send + Sync {
    fn on_config_changed(&self);
}

/// 服务容器
#[derive(Debug)]
pub struct ServiceContainer {
    services: Arc<RwLock<HashMap<String, Box<dyn Any + Send + Sync>>>>,
}

impl ServiceContainer {
    pub fn new() -> Self {
        Self {
            services: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    pub fn register<T: 'static + Send + Sync>(&self, name: String, service: T) {
        let mut services = self.services.write().unwrap();
        services.insert(name, Box::new(service));
    }
    
    pub fn get<T: 'static>(&self, name: &str) -> Option<&T> {
        let services = self.services.read().unwrap();
        services.get(name)?.downcast_ref::<T>()
    }
}

/// 事件处理器
#[derive(Debug)]
pub struct EventProcessor {
    handlers: Arc<RwLock<HashMap<String, Vec<Box<dyn EventHandler>>>>>,
}

impl EventProcessor {
    pub fn new() -> Self {
        Self {
            handlers: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    pub fn register_handler(&self, event_type: String, handler: Box<dyn EventHandler>) {
        let mut handlers = self.handlers.write().unwrap();
        handlers.entry(event_type).or_insert_with(Vec::new).push(handler);
    }
    
    pub async fn process_event(&self, event: AppEvent) -> Result<()> {
        let event_type = std::any::type_name::<AppEvent>();
        let handlers = {
            let handlers = self.handlers.read().unwrap();
            handlers.get(event_type).cloned()
        };
        
        if let Some(handlers) = handlers {
            for handler in handlers {
                handler.handle(event.clone()).await?;
            }
        }
        
        Ok(())
    }
}

/// 事件处理器trait
#[async_trait::async_trait]
pub trait EventHandler: Send + Sync {
    async fn handle(&self, event: AppEvent) -> Result<()>;
}

/// 性能监控器
#[derive(Debug)]
pub struct PerformanceMonitor {
    metrics: Arc<Mutex<HashMap<String, MetricValue>>>,
    collectors: Vec<Box<dyn MetricCollector>>,
}

impl PerformanceMonitor {
    pub fn new() -> Self {
        Self {
            metrics: Arc::new(Mutex::new(HashMap::new())),
            collectors: Vec::new(),
        }
    }
    
    pub fn record_metric(&self, name: String, value: MetricValue) {
        let mut metrics = self.metrics.lock().unwrap();
        metrics.insert(name, value);
    }
    
    pub fn get_metrics(&self) -> HashMap<String, MetricValue> {
        self.metrics.lock().unwrap().clone()
    }
}

#[derive(Debug, Clone)]
pub enum MetricValue {
    Counter(u64),
    Gauge(f64),
    Histogram(Vec<f64>),
    Timer(Duration),
}

/// 指标收集器trait
pub trait MetricCollector: Send + Sync {
    fn collect(&self) -> HashMap<String, MetricValue>;
}

/// 健康检查器
#[derive(Debug)]
pub struct HealthChecker {
    checks: Vec<Box<dyn HealthCheck>>,
}

impl HealthChecker {
    pub fn new() -> Self {
        Self {
            checks: Vec::new(),
        }
    }
    
    pub fn add_check(&mut self, check: Box<dyn HealthCheck>) {
        self.checks.push(check);
    }
    
    pub async fn run_checks(&self) -> Vec<HealthCheckResult> {
        let mut results = Vec::new();
        
        for check in &self.checks {
            let result = check.check().await;
            results.push(result);
        }
        
        results
    }
}

/// 健康检查trait
#[async_trait::async_trait]
pub trait HealthCheck: Send + Sync {
    async fn check(&self) -> HealthCheckResult;
}

#[derive(Debug, Clone)]
pub struct HealthCheckResult {
    pub name: String,
    pub status: HealthStatus,
    pub message: String,
    pub duration: Duration,
}

#[derive(Debug, Clone, PartialEq)]
pub enum HealthStatus {
    Healthy,
    Unhealthy,
    Degraded,
}

/// 应用生命周期管理器
#[derive(Debug)]
pub struct LifecycleManager {
    phases: Vec<Box<dyn LifecyclePhase>>,
    current_phase: Arc<RwLock<LifecyclePhase>>,
}

impl LifecycleManager {
    pub fn new() -> Self {
        Self {
            phases: Vec::new(),
            current_phase: Arc::new(RwLock::new(LifecyclePhase::Initializing)),
        }
    }
    
    pub async fn start(&self) -> Result<()> {
        self.transition_to(LifecyclePhase::Starting).await?;
        self.transition_to(LifecyclePhase::Running).await?;
        Ok(())
    }
    
    pub async fn stop(&self) -> Result<()> {
        self.transition_to(LifecyclePhase::Stopping).await?;
        self.transition_to(LifecyclePhase::Stopped).await?;
        Ok(())
    }
    
    async fn transition_to(&self, phase: LifecyclePhase) -> Result<()> {
        let mut current_phase = self.current_phase.write().unwrap();
        *current_phase = phase;
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LifecyclePhase {
    Initializing,
    Starting,
    Running,
    Stopping,
    Stopped,
}

/// 主应用入口点
impl EnterpriseApp {
    /// 创建新的企业级应用
    pub async fn new(config: AppConfig) -> Result<Self> {
        let config_manager = Arc::new(ConfigManager::new(config.clone()));
        let state_manager = Arc::new(AppStateManager::new(config));
        let service_container = Arc::new(ServiceContainer::new());
        let event_processor = Arc::new(EventProcessor::new());
        let performance_monitor = Arc::new(PerformanceMonitor::new());
        let health_checker = Arc::new(HealthChecker::new());
        let lifecycle_manager = Arc::new(LifecycleManager::new());
        
        Ok(Self {
            config_manager,
            state_manager,
            service_container,
            event_processor,
            performance_monitor,
            health_checker,
            lifecycle_manager,
        })
    }
    
    /// 启动应用
    pub async fn start(&self) -> Result<()> {
        println!("🚀 启动企业级应用...");
        
        // 启动生命周期管理器
        self.lifecycle_manager.start().await?;
        
        // 初始化服务
        self.initialize_services().await?;
        
        // 启动事件处理
        self.start_event_processing().await?;
        
        // 启动性能监控
        self.start_performance_monitoring().await?;
        
        // 启动健康检查
        self.start_health_checking().await?;
        
        // 启动定期清理任务
        self.start_cleanup_tasks().await?;
        
        println!("✅ 企业级应用启动完成");
        Ok(())
    }
    
    /// 停止应用
    pub async fn stop(&self) -> Result<()> {
        println!("🛑 停止企业级应用...");
        
        // 停止生命周期管理器
        self.lifecycle_manager.stop().await?;
        
        println!("✅ 企业级应用已停止");
        Ok(())
    }
    
    /// 获取应用状态
    pub fn get_status(&self) -> AppStatus {
        AppStatus {
            system_state: self.state_manager.get_system_state(),
            metrics: self.state_manager.get_metrics(),
            config: self.config_manager.get_config(),
        }
    }
    
    // 私有方法
    async fn initialize_services(&self) -> Result<()> {
        println!("🔧 初始化服务...");
        // 在这里初始化各种服务
        Ok(())
    }
    
    async fn start_event_processing(&self) -> Result<()> {
        println!("📡 启动事件处理...");
        // 启动事件处理循环
        Ok(())
    }
    
    async fn start_performance_monitoring(&self) -> Result<()> {
        println!("📊 启动性能监控...");
        // 启动性能监控任务
        Ok(())
    }
    
    async fn start_health_checking(&self) -> Result<()> {
        println!("🏥 启动健康检查...");
        // 启动健康检查任务
        Ok(())
    }
    
    async fn start_cleanup_tasks(&self) -> Result<()> {
        println!("🧹 启动清理任务...");
        // 启动定期清理任务
        Ok(())
    }
}

#[derive(Debug)]
pub struct AppStatus {
    pub system_state: SystemState,
    pub metrics: PerformanceMetrics,
    pub config: AppConfig,
}

/// 主函数 - 演示企业级应用的使用
#[tokio::main]
async fn main() -> Result<()> {
    // 初始化日志
    env_logger::init();
    
    // 加载配置
    let config = load_config().await?;
    
    // 创建应用
    let app = EnterpriseApp::new(config).await?;
    
    // 启动应用
    app.start().await?;
    
    // 模拟应用运行
    tokio::time::sleep(Duration::from_secs(10)).await;
    
    // 停止应用
    app.stop().await?;
    
    Ok(())
}

/// 加载应用配置
async fn load_config() -> Result<AppConfig> {
    // 在实际应用中，这里会从文件或环境变量加载配置
    Ok(AppConfig {
        app_info: AppInfo {
            name: "企业级Rust应用".to_string(),
            version: "1.0.0".to_string(),
            environment: Environment::Development,
            debug_mode: true,
        },
        database: DatabaseConfig {
            url: "postgresql://localhost:5432/enterprise_app".to_string(),
            max_connections: 100,
            connection_timeout: Duration::from_secs(30),
            pool_size: 10,
        },
        cache: CacheConfig {
            redis_url: "redis://localhost:6379".to_string(),
            ttl: Duration::from_secs(3600),
            max_size: 1000,
        },
        logging: LoggingConfig {
            level: LogLevel::Info,
            format: LogFormat::Json,
            output: LogOutput::Console,
        },
        monitoring: MonitoringConfig {
            metrics_endpoint: "http://localhost:9090/metrics".to_string(),
            health_check_interval: Duration::from_secs(30),
            performance_thresholds: PerformanceThresholds {
                max_response_time: Duration::from_millis(500),
                max_memory_usage: 80.0,
                max_cpu_usage: 80.0,
            },
        },
        security: SecurityConfig {
            jwt_secret: "your-secret-key".to_string(),
            session_timeout: Duration::from_secs(3600),
            max_login_attempts: 5,
            password_policy: PasswordPolicy {
                min_length: 8,
                require_uppercase: true,
                require_lowercase: true,
                require_numbers: true,
                require_special_chars: true,
            },
        },
    })
}
