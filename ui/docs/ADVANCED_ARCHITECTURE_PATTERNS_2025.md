# Rust 1.90 高级架构设计模式

## 📋 目录

## 🏗️ 架构模式概述

### 设计原则

1. **单一职责原则 (SRP)**: 每个模块只负责一个功能
2. **开闭原则 (OCP)**: 对扩展开放，对修改关闭
3. **里氏替换原则 (LSP)**: 子类可以替换父类
4. **接口隔离原则 (ISP)**: 使用多个专门的接口
5. **依赖倒置原则 (DIP)**: 依赖抽象而不是具体实现

### 架构层次

```text
┌─────────────────────────────────────────────────────────────┐
│                   表现层 (Presentation Layer)                │
│  ┌─────────────┐ ┌─────────────┐ ┌─────────────┐ ┌─────────┐ │
│  │   Web UI    │ │  Desktop UI │ │  Mobile UI  │ │   API   │ │
│  │  (Dioxus)   │ │   (Tauri)   │ │  (Dioxus)   │ │(Axum)   │ │
│  └─────────────┘ └─────────────┘ └─────────────┘ └─────────┘ │
├─────────────────────────────────────────────────────────────┤
│                    应用层 (Application Layer)                │
│  ┌─────────────┐ ┌─────────────┐ ┌─────────────┐ ┌─────────┐ │
│  │   用例服务  │ │   命令处理   │ │   查询处理   │ │ 事件处理 │ │
│  │   (Use Case)│ │  (Command)  │ │  (Query)    │ │(Event)  │ │
│  └─────────────┘ └─────────────┘ └─────────────┘ └─────────┘ │
├─────────────────────────────────────────────────────────────┤
│                    领域层 (Domain Layer)                     │
│  ┌─────────────┐ ┌─────────────┐ ┌─────────────┐ ┌─────────┐ │
│  │   实体     │ │   值对象     │ │   聚合根     │ │ 领域服务 │ │
│  │  (Entity)   │ │ (Value Obj) │ │ (Aggregate) │ │(Service)│ │
│  └─────────────┘ └─────────────┘ └─────────────┘ └─────────┘ │
├─────────────────────────────────────────────────────────────┤
│                   基础设施层 (Infrastructure Layer)           │
│  ┌─────────────┐ ┌─────────────┐ ┌─────────────┐ ┌─────────┐ │
│  │   数据库     │ │   缓存      │ │   消息队列   │ │ 外部API │ │
│  │ (Database)  │ │  (Cache)    │ │  (Message)  │ │ (API)   │ │
│  └─────────────┘ └─────────────┘ └─────────────┘ └─────────┘ │
└─────────────────────────────────────────────────────────────┘
```

## 🆕 Rust 1.90新特性集成

### 1. 改进的异步编程支持

#### 优化的Future trait

```rust
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

// Rust 1.90 优化的异步组合器
pub struct OptimizedFuture<T> {
    inner: Pin<Box<dyn Future<Output = T> + Send + 'static>>,
}

impl<T> Future for OptimizedFuture<T> {
    type Output = T;
    
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // Rust 1.90 改进的轮询机制
        self.inner.as_mut().poll(cx)
    }
}

// 使用新的异步特性
pub async fn enhanced_async_operation() -> Result<String, Box<dyn std::error::Error>> {
    // 改进的错误处理
    let result = tokio::time::timeout(
        Duration::from_secs(5),
        fetch_data_with_retry().await
    ).await??; // 双重问号操作符优化
    
    Ok(result)
}
```

#### 增强的模式匹配

```rust
// Rust 1.90 增强的模式匹配在架构中的应用
pub enum ArchitectureEvent {
    UserCreated { user_id: String, username: String },
    UserUpdated { user_id: String, changes: HashMap<String, String> },
    UserDeleted { user_id: String },
    SystemShutdown { reason: String },
}

impl ArchitectureEvent {
    pub fn handle_with_enhanced_patterns(&self) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            // 使用守卫条件进行复杂匹配
            ArchitectureEvent::UserCreated { user_id, username } 
                if user_id.len() > 0 && username.len() > 0 => {
                self.process_user_creation(user_id, username).await?;
            }
            
            // 使用范围匹配
            ArchitectureEvent::UserUpdated { user_id, changes } 
                if changes.len() > 0 && changes.len() < 100 => {
                self.process_user_update(user_id, changes).await?;
            }
            
            // 使用通配符和条件
            ArchitectureEvent::UserDeleted { user_id } => {
                self.process_user_deletion(user_id).await?;
            }
            
            // 默认处理
            _ => {
                log::warn!("未处理的事件类型: {:?}", self);
            }
        }
        Ok(())
    }
}
```

### 2. 新API稳定化应用

#### Cell::update 在状态管理中的应用

```rust
use std::cell::Cell;
use std::sync::Arc;

// 使用Cell::update进行原子状态更新
pub struct AtomicStateManager {
    state: Cell<ApplicationState>,
    version: Cell<u64>,
}

impl AtomicStateManager {
    pub fn new() -> Self {
        Self {
            state: Cell::new(ApplicationState::default()),
            version: Cell::new(0),
        }
    }
    
    // 使用Rust 1.90的Cell::update进行原子更新
    pub fn update_state<F>(&self, updater: F) -> Result<ApplicationState, StateError>
    where
        F: FnOnce(&ApplicationState) -> Result<ApplicationState, StateError>,
    {
        let new_state = self.state.update(|current_state| {
            updater(current_state)
        })?;
        
        // 原子更新版本号
        self.version.update(|v| v + 1);
        
        Ok(new_state)
    }
    
    pub fn get_state(&self) -> ApplicationState {
        self.state.get()
    }
    
    pub fn get_version(&self) -> u64 {
        self.version.get()
    }
}
```

#### HashMap::extract_if 在缓存管理中的应用

```rust
use std::collections::HashMap;
use std::time::{Duration, Instant};

// 使用HashMap::extract_if进行高效的缓存清理
pub struct SmartCache<K, V> {
    data: HashMap<K, CacheEntry<V>>,
    max_size: usize,
    ttl: Duration,
}

struct CacheEntry<V> {
    value: V,
    created_at: Instant,
    access_count: u64,
}

impl<K: Eq + Hash + Clone, V> SmartCache<K, V> {
    pub fn new(max_size: usize, ttl: Duration) -> Self {
        Self {
            data: HashMap::new(),
            max_size,
            ttl,
        }
    }
    
    // 使用Rust 1.90的extract_if进行高效清理
    pub fn cleanup_expired(&mut self) -> usize {
        let now = Instant::now();
        let expired_keys: Vec<K> = self.data
            .extract_if(|_, entry| now.duration_since(entry.created_at) > self.ttl)
            .map(|(key, _)| key)
            .collect();
        
        expired_keys.len()
    }
    
    // 使用extract_if进行LRU清理
    pub fn cleanup_lru(&mut self) -> usize {
        if self.data.len() <= self.max_size {
            return 0;
        }
        
        let mut entries: Vec<_> = self.data.iter().collect();
        entries.sort_by_key(|(_, entry)| entry.access_count);
        
        let to_remove = entries.len() - self.max_size;
        let keys_to_remove: Vec<K> = entries
            .into_iter()
            .take(to_remove)
            .map(|(key, _)| key.clone())
            .collect();
        
        for key in keys_to_remove {
            self.data.remove(&key);
        }
        
        to_remove
    }
}
```

### 3. 性能优化特性

#### 改进的编译器优化

```rust
// Rust 1.90 编译器优化在架构中的应用
#[inline(always)] // 更积极的内联优化
pub fn hot_path_operation(data: &[u8]) -> Result<ProcessedData, ProcessingError> {
    // 编译器会自动优化这个热路径
    data.iter()
        .map(|&byte| process_byte(byte))
        .collect::<Result<Vec<_>, _>>()
        .map(ProcessedData::new)
}

// 使用新的优化提示
#[cold] // 标记冷路径
pub fn error_handling_path(error: ProcessingError) -> ErrorResponse {
    ErrorResponse::new(error)
}

// 使用likely/unlikely提示
pub fn branch_prediction_optimized(value: u32) -> String {
    if std::intrinsics::likely(value > 1000) {
        "large_value".to_string()
    } else {
        "small_value".to_string()
    }
}
```

## 🏢 企业级架构模式

### 1. 分层架构 (Layered Architecture)

```rust
//! 分层架构示例 - 展示如何组织企业级应用的代码结构

use std::sync::Arc;
use serde::{Deserialize, Serialize};
use anyhow::Result;

/// 表现层 - 处理用户界面和API请求
pub mod presentation {
    use super::*;
    
    /// Web控制器
    pub struct WebController {
        user_service: Arc<dyn application::UserService>,
    }
    
    impl WebController {
        pub fn new(user_service: Arc<dyn application::UserService>) -> Self {
            Self { user_service }
        }
        
        pub async fn create_user(&self, request: CreateUserRequest) -> Result<CreateUserResponse> {
            let command = application::CreateUserCommand {
                username: request.username,
                email: request.email,
                password: request.password,
            };
            
            let user = self.user_service.create_user(command).await?;
            
            Ok(CreateUserResponse {
                id: user.id,
                username: user.username,
                email: user.email,
            })
        }
    }
    
    #[derive(Deserialize)]
    pub struct CreateUserRequest {
        pub username: String,
        pub email: String,
        pub password: String,
    }
    
    #[derive(Serialize)]
    pub struct CreateUserResponse {
        pub id: String,
        pub username: String,
        pub email: String,
    }
}

/// 应用层 - 处理业务用例
pub mod application {
    use super::*;
    
    /// 用户服务接口
    #[async_trait::async_trait]
    pub trait UserService: Send + Sync {
        async fn create_user(&self, command: CreateUserCommand) -> Result<UserDto>;
        async fn get_user(&self, id: &str) -> Result<Option<UserDto>>;
        async fn update_user(&self, id: &str, command: UpdateUserCommand) -> Result<UserDto>;
        async fn delete_user(&self, id: &str) -> Result<()>;
    }
    
    /// 创建用户命令
    #[derive(Debug)]
    pub struct CreateUserCommand {
        pub username: String,
        pub email: String,
        pub password: String,
    }
    
    /// 更新用户命令
    #[derive(Debug)]
    pub struct UpdateUserCommand {
        pub username: Option<String>,
        pub email: Option<String>,
    }
    
    /// 用户数据传输对象
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct UserDto {
        pub id: String,
        pub username: String,
        pub email: String,
        pub created_at: chrono::DateTime<chrono::Utc>,
    }
    
    /// 用户服务实现
    pub struct UserServiceImpl {
        user_repository: Arc<dyn domain::UserRepository>,
        event_publisher: Arc<dyn domain::EventPublisher>,
    }
    
    impl UserServiceImpl {
        pub fn new(
            user_repository: Arc<dyn domain::UserRepository>,
            event_publisher: Arc<dyn domain::EventPublisher>,
        ) -> Self {
            Self {
                user_repository,
                event_publisher,
            }
        }
    }
    
    #[async_trait::async_trait]
    impl UserService for UserServiceImpl {
        async fn create_user(&self, command: CreateUserCommand) -> Result<UserDto> {
            // 创建领域实体
            let user = domain::User::new(
                command.username.clone(),
                command.email.clone(),
                command.password,
            )?;
            
            // 保存到仓库
            let saved_user = self.user_repository.save(user).await?;
            
            // 发布事件
            let event = domain::UserCreatedEvent {
                user_id: saved_user.id().clone(),
                username: saved_user.username().clone(),
                email: saved_user.email().clone(),
            };
            self.event_publisher.publish(event).await?;
            
            // 转换为DTO
            Ok(UserDto {
                id: saved_user.id().clone(),
                username: saved_user.username().clone(),
                email: saved_user.email().clone(),
                created_at: saved_user.created_at(),
            })
        }
        
        async fn get_user(&self, id: &str) -> Result<Option<UserDto>> {
            let user = self.user_repository.find_by_id(id).await?;
            
            Ok(user.map(|u| UserDto {
                id: u.id().clone(),
                username: u.username().clone(),
                email: u.email().clone(),
                created_at: u.created_at(),
            }))
        }
        
        async fn update_user(&self, id: &str, command: UpdateUserCommand) -> Result<UserDto> {
            let mut user = self.user_repository.find_by_id(id).await?
                .ok_or_else(|| anyhow::anyhow!("用户不存在"))?;
            
            if let Some(username) = command.username {
                user.change_username(username)?;
            }
            
            if let Some(email) = command.email {
                user.change_email(email)?;
            }
            
            let saved_user = self.user_repository.save(user).await?;
            
            Ok(UserDto {
                id: saved_user.id().clone(),
                username: saved_user.username().clone(),
                email: saved_user.email().clone(),
                created_at: saved_user.created_at(),
            })
        }
        
        async fn delete_user(&self, id: &str) -> Result<()> {
            self.user_repository.delete_by_id(id).await?;
            Ok(())
        }
    }
}

/// 领域层 - 核心业务逻辑
pub mod domain {
    use super::*;
    
    /// 用户实体
    #[derive(Debug, Clone)]
    pub struct User {
        id: String,
        username: String,
        email: String,
        password_hash: String,
        created_at: chrono::DateTime<chrono::Utc>,
        updated_at: chrono::DateTime<chrono::Utc>,
    }
    
    impl User {
        pub fn new(username: String, email: String, password: String) -> Result<Self> {
            // 验证输入
            if username.is_empty() {
                return Err(anyhow::anyhow!("用户名不能为空"));
            }
            
            if !email.contains('@') {
                return Err(anyhow::anyhow!("邮箱格式不正确"));
            }
            
            // 密码哈希
            let password_hash = hash_password(&password)?;
            
            let now = chrono::Utc::now();
            Ok(Self {
                id: uuid::Uuid::new_v4().to_string(),
                username,
                email,
                password_hash,
                created_at: now,
                updated_at: now,
            })
        }
        
        pub fn id(&self) -> &str {
            &self.id
        }
        
        pub fn username(&self) -> &str {
            &self.username
        }
        
        pub fn email(&self) -> &str {
            &self.email
        }
        
        pub fn created_at(&self) -> chrono::DateTime<chrono::Utc> {
            self.created_at
        }
        
        pub fn change_username(&mut self, new_username: String) -> Result<()> {
            if new_username.is_empty() {
                return Err(anyhow::anyhow!("用户名不能为空"));
            }
            
            self.username = new_username;
            self.updated_at = chrono::Utc::now();
            Ok(())
        }
        
        pub fn change_email(&mut self, new_email: String) -> Result<()> {
            if !new_email.contains('@') {
                return Err(anyhow::anyhow!("邮箱格式不正确"));
            }
            
            self.email = new_email;
            self.updated_at = chrono::Utc::now();
            Ok(())
        }
    }
    
    /// 用户仓库接口
    #[async_trait::async_trait]
    pub trait UserRepository: Send + Sync {
        async fn save(&self, user: User) -> Result<User>;
        async fn find_by_id(&self, id: &str) -> Result<Option<User>>;
        async fn find_by_username(&self, username: &str) -> Result<Option<User>>;
        async fn find_by_email(&self, email: &str) -> Result<Option<User>>;
        async fn delete_by_id(&self, id: &str) -> Result<()>;
    }
    
    /// 事件发布器接口
    #[async_trait::async_trait]
    pub trait EventPublisher: Send + Sync {
        async fn publish<T>(&self, event: T) -> Result<()>
        where
            T: DomainEvent + Send + Sync;
    }
    
    /// 领域事件trait
    pub trait DomainEvent: Send + Sync {
        fn event_type(&self) -> &str;
        fn occurred_at(&self) -> chrono::DateTime<chrono::Utc>;
    }
    
    /// 用户创建事件
    #[derive(Debug, Clone)]
    pub struct UserCreatedEvent {
        pub user_id: String,
        pub username: String,
        pub email: String,
    }
    
    impl DomainEvent for UserCreatedEvent {
        fn event_type(&self) -> &str {
            "user.created"
        }
        
        fn occurred_at(&self) -> chrono::DateTime<chrono::Utc> {
            chrono::Utc::now()
        }
    }
}

/// 基础设施层 - 外部依赖实现
pub mod infrastructure {
    use super::*;
    
    /// 数据库用户仓库实现
    pub struct DatabaseUserRepository {
        pool: sqlx::PgPool,
    }
    
    impl DatabaseUserRepository {
        pub fn new(pool: sqlx::PgPool) -> Self {
            Self { pool }
        }
    }
    
    #[async_trait::async_trait]
    impl domain::UserRepository for DatabaseUserRepository {
        async fn save(&self, user: domain::User) -> Result<domain::User> {
            // 数据库保存逻辑
            // 这里简化实现
            Ok(user)
        }
        
        async fn find_by_id(&self, id: &str) -> Result<Option<domain::User>> {
            // 数据库查询逻辑
            // 这里简化实现
            Ok(None)
        }
        
        async fn find_by_username(&self, username: &str) -> Result<Option<domain::User>> {
            // 数据库查询逻辑
            Ok(None)
        }
        
        async fn find_by_email(&self, email: &str) -> Result<Option<domain::User>> {
            // 数据库查询逻辑
            Ok(None)
        }
        
        async fn delete_by_id(&self, id: &str) -> Result<()> {
            // 数据库删除逻辑
            Ok(())
        }
    }
    
    /// 消息队列事件发布器实现
    pub struct MessageQueueEventPublisher {
        sender: tokio::sync::mpsc::UnboundedSender<String>,
    }
    
    impl MessageQueueEventPublisher {
        pub fn new(sender: tokio::sync::mpsc::UnboundedSender<String>) -> Self {
            Self { sender }
        }
    }
    
    #[async_trait::async_trait]
    impl domain::EventPublisher for MessageQueueEventPublisher {
        async fn publish<T>(&self, event: T) -> Result<()>
        where
            T: domain::DomainEvent + Send + Sync,
        {
            let event_json = serde_json::to_string(&event)?;
            self.sender.send(event_json)?;
            Ok(())
        }
    }
}

/// 辅助函数
fn hash_password(password: &str) -> Result<String> {
    // 实际应用中应该使用bcrypt或argon2
    Ok(format!("hashed_{}", password))
}
```

### 2. 依赖注入容器

```rust
//! 依赖注入容器 - 管理应用中的依赖关系

use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::Arc;
use anyhow::Result;

/// 服务生命周期
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ServiceLifetime {
    /// 单例 - 整个应用生命周期内只有一个实例
    Singleton,
    /// 瞬态 - 每次请求都创建新实例
    Transient,
    /// 作用域 - 在同一作用域内共享实例
    Scoped,
}

/// 服务描述符
#[derive(Debug)]
pub struct ServiceDescriptor {
    pub service_type: TypeId,
    pub implementation_type: TypeId,
    pub lifetime: ServiceLifetime,
    pub factory: Box<dyn ServiceFactory>,
}

/// 服务工厂trait
pub trait ServiceFactory: Send + Sync {
    fn create_service(&self, provider: &ServiceProvider) -> Result<Box<dyn Any + Send + Sync>>;
}

/// 服务提供者
pub struct ServiceProvider {
    services: HashMap<TypeId, ServiceDescriptor>,
    singletons: HashMap<TypeId, Box<dyn Any + Send + Sync>>,
}

impl ServiceProvider {
    pub fn new() -> Self {
        Self {
            services: HashMap::new(),
            singletons: HashMap::new(),
        }
    }
    
    /// 注册服务
    pub fn register_singleton<TInterface, TImplementation>(&mut self) -> Result<()>
    where
        TInterface: 'static + Send + Sync,
        TImplementation: 'static + Send + Sync + TInterface,
    {
        let service_type = TypeId::of::<TInterface>();
        let implementation_type = TypeId::of::<TImplementation>();
        
        let factory = Box::new(ConcreteServiceFactory::<TImplementation>::new());
        
        let descriptor = ServiceDescriptor {
            service_type,
            implementation_type,
            lifetime: ServiceLifetime::Singleton,
            factory,
        };
        
        self.services.insert(service_type, descriptor);
        Ok(())
    }
    
    /// 注册瞬态服务
    pub fn register_transient<TInterface, TImplementation>(&mut self) -> Result<()>
    where
        TInterface: 'static + Send + Sync,
        TImplementation: 'static + Send + Sync + TInterface,
    {
        let service_type = TypeId::of::<TInterface>();
        let implementation_type = TypeId::of::<TImplementation>();
        
        let factory = Box::new(ConcreteServiceFactory::<TImplementation>::new());
        
        let descriptor = ServiceDescriptor {
            service_type,
            implementation_type,
            lifetime: ServiceLifetime::Transient,
            factory,
        };
        
        self.services.insert(service_type, descriptor);
        Ok(())
    }
    
    /// 获取服务
    pub fn get_service<T>(&self) -> Result<Arc<T>>
    where
        T: 'static + Send + Sync,
    {
        let service_type = TypeId::of::<T>();
        
        let descriptor = self.services.get(&service_type)
            .ok_or_else(|| anyhow::anyhow!("服务未注册: {:?}", service_type))?;
        
        match descriptor.lifetime {
            ServiceLifetime::Singleton => {
                if let Some(singleton) = self.singletons.get(&service_type) {
                    Ok(singleton.downcast_ref::<T>()
                        .ok_or_else(|| anyhow::anyhow!("类型转换失败"))?
                        .clone())
                } else {
                    Err(anyhow::anyhow!("单例服务未初始化"))
                }
            }
            ServiceLifetime::Transient => {
                let instance = descriptor.factory.create_service(self)?;
                let instance = instance.downcast::<T>()
                    .map_err(|_| anyhow::anyhow!("类型转换失败"))?;
                Ok(Arc::new(*instance))
            }
            ServiceLifetime::Scoped => {
                // 作用域服务的实现需要更复杂的逻辑
                Err(anyhow::anyhow!("作用域服务暂未实现"))
            }
        }
    }
}

/// 具体服务工厂
struct ConcreteServiceFactory<T> {
    _phantom: std::marker::PhantomData<T>,
}

impl<T> ConcreteServiceFactory<T> {
    fn new() -> Self {
        Self {
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<T> ServiceFactory for ConcreteServiceFactory<T>
where
    T: 'static + Send + Sync + Default,
{
    fn create_service(&self, _provider: &ServiceProvider) -> Result<Box<dyn Any + Send + Sync>> {
        Ok(Box::new(T::default()))
    }
}

/// 服务注册扩展trait
pub trait ServiceCollection {
    fn add_singleton<TInterface, TImplementation>(&mut self) -> Result<()>
    where
        TInterface: 'static + Send + Sync,
        TImplementation: 'static + Send + Sync + TInterface;
    
    fn add_transient<TInterface, TImplementation>(&mut self) -> Result<()>
    where
        TInterface: 'static + Send + Sync,
        TImplementation: 'static + Send + Sync + TInterface;
}

impl ServiceCollection for ServiceProvider {
    fn add_singleton<TInterface, TImplementation>(&mut self) -> Result<()>
    where
        TInterface: 'static + Send + Sync,
        TImplementation: 'static + Send + Sync + TInterface,
    {
        self.register_singleton::<TInterface, TImplementation>()
    }
    
    fn add_transient<TInterface, TImplementation>(&mut self) -> Result<()>
    where
        TInterface: 'static + Send + Sync,
        TImplementation: 'static + Send + Sync + TInterface,
    {
        self.register_transient::<TInterface, TImplementation>()
    }
}
```

## 🔄 事件驱动架构

### 事件总线实现

```rust
//! 事件驱动架构 - 实现松耦合的事件通信

use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use tokio::sync::mpsc;
use serde::{Deserialize, Serialize};
use anyhow::Result;

/// 事件总线
#[derive(Debug)]
pub struct EventBus {
    handlers: Arc<RwLock<HashMap<String, Vec<Box<dyn EventHandler + Send + Sync>>>>>,
    event_queue: mpsc::UnboundedSender<Box<dyn Event + Send + Sync>>,
}

/// 事件trait
pub trait Event: Send + Sync {
    fn event_type(&self) -> &str;
    fn occurred_at(&self) -> chrono::DateTime<chrono::Utc>;
    fn aggregate_id(&self) -> &str;
}

/// 事件处理器trait
#[async_trait::async_trait]
pub trait EventHandler: Send + Sync {
    async fn handle(&self, event: &dyn Event) -> Result<()>;
    fn can_handle(&self, event_type: &str) -> bool;
}

impl EventBus {
    pub fn new() -> Self {
        let (sender, mut receiver) = mpsc::unbounded_channel::<Box<dyn Event + Send + Sync>>();
        
        let handlers = Arc::new(RwLock::new(HashMap::new()));
        let handlers_clone = handlers.clone();
        
        // 启动事件处理循环
        tokio::spawn(async move {
            while let Some(event) = receiver.recv().await {
                let event_type = event.event_type().to_string();
                let handlers = {
                    let handlers = handlers_clone.read().unwrap();
                    handlers.get(&event_type).cloned()
                };
                
                if let Some(handlers) = handlers {
                    for handler in handlers {
                        if let Err(e) = handler.handle(event.as_ref()).await {
                            eprintln!("事件处理失败: {}", e);
                        }
                    }
                }
            }
        });
        
        Self {
            handlers,
            event_queue: sender,
        }
    }
    
    /// 注册事件处理器
    pub fn register_handler(&self, event_type: String, handler: Box<dyn EventHandler + Send + Sync>) {
        let mut handlers = self.handlers.write().unwrap();
        handlers.entry(event_type).or_insert_with(Vec::new).push(handler);
    }
    
    /// 发布事件
    pub fn publish(&self, event: Box<dyn Event + Send + Sync>) -> Result<()> {
        self.event_queue.send(event)?;
        Ok(())
    }
}

/// 用户事件示例
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserCreatedEvent {
    pub user_id: String,
    pub username: String,
    pub email: String,
    pub occurred_at: chrono::DateTime<chrono::Utc>,
}

impl Event for UserCreatedEvent {
    fn event_type(&self) -> &str {
        "user.created"
    }
    
    fn occurred_at(&self) -> chrono::DateTime<chrono::Utc> {
        self.occurred_at
    }
    
    fn aggregate_id(&self) -> &str {
        &self.user_id
    }
}

/// 用户事件处理器
pub struct UserEventHandler;

#[async_trait::async_trait]
impl EventHandler for UserEventHandler {
    async fn handle(&self, event: &dyn Event) -> Result<()> {
        if let Some(user_event) = event.as_any().downcast_ref::<UserCreatedEvent>() {
            println!("处理用户创建事件: {:?}", user_event);
            // 发送欢迎邮件、创建用户档案等
        }
        Ok(())
    }
    
    fn can_handle(&self, event_type: &str) -> bool {
        event_type == "user.created"
    }
}

/// 事件扩展trait
pub trait EventExt {
    fn as_any(&self) -> &dyn std::any::Any;
}

impl<T: Event + 'static> EventExt for T {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
```

## 📊 CQRS模式

### 命令查询职责分离

```rust
//! CQRS模式实现 - 分离命令和查询的职责

use std::sync::Arc;
use serde::{Deserialize, Serialize};
use anyhow::Result;

/// 命令接口
#[async_trait::async_trait]
pub trait Command: Send + Sync {
    type Result;
    async fn execute(self) -> Result<Self::Result>;
}

/// 查询接口
#[async_trait::async_trait]
pub trait Query: Send + Sync {
    type Result;
    async fn execute(self) -> Result<Self::Result>;
}

/// 命令处理器
#[async_trait::async_trait]
pub trait CommandHandler<C>: Send + Sync
where
    C: Command,
{
    async fn handle(&self, command: C) -> Result<C::Result>;
}

/// 查询处理器
#[async_trait::async_trait]
pub trait QueryHandler<Q>: Send + Sync
where
    Q: Query,
{
    async fn handle(&self, query: Q) -> Result<Q::Result>;
}

/// 命令总线
pub struct CommandBus {
    handlers: std::collections::HashMap<std::any::TypeId, Box<dyn CommandHandler<dyn Command<Result = ()>> + Send + Sync>>,
}

impl CommandBus {
    pub fn new() -> Self {
        Self {
            handlers: std::collections::HashMap::new(),
        }
    }
    
    pub fn register_handler<C, H>(&mut self, handler: H)
    where
        C: Command + 'static,
        H: CommandHandler<C> + 'static,
    {
        let type_id = std::any::TypeId::of::<C>();
        // 这里需要类型擦除，实际实现会更复杂
    }
}

/// 查询总线
pub struct QueryBus {
    handlers: std::collections::HashMap<std::any::TypeId, Box<dyn QueryHandler<dyn Query<Result = ()>> + Send + Sync>>,
}

impl QueryBus {
    pub fn new() -> Self {
        Self {
            handlers: std::collections::HashMap::new(),
        }
    }
}

/// 用户命令示例
#[derive(Debug)]
pub struct CreateUserCommand {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct CreateUserResult {
    pub user_id: String,
}

#[async_trait::async_trait]
impl Command for CreateUserCommand {
    type Result = CreateUserResult;
    
    async fn execute(self) -> Result<Self::Result> {
        // 命令执行逻辑
        Ok(CreateUserResult {
            user_id: uuid::Uuid::new_v4().to_string(),
        })
    }
}

/// 用户查询示例
#[derive(Debug)]
pub struct GetUserQuery {
    pub user_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetUserResult {
    pub user_id: String,
    pub username: String,
    pub email: String,
}

#[async_trait::async_trait]
impl Query for GetUserQuery {
    type Result = GetUserResult;
    
    async fn execute(self) -> Result<Self::Result> {
        // 查询执行逻辑
        Ok(GetUserResult {
            user_id: self.user_id,
            username: "test_user".to_string(),
            email: "test@example.com".to_string(),
        })
    }
}
```

## 🏛️ 六边形架构

### 端口和适配器模式

```rust
//! 六边形架构 - 端口和适配器模式

use std::sync::Arc;
use anyhow::Result;

/// 端口定义 - 定义业务逻辑的接口
pub mod ports {
    use super::*;
    
    /// 用户仓库端口
    #[async_trait::async_trait]
    pub trait UserRepository: Send + Sync {
        async fn save(&self, user: User) -> Result<User>;
        async fn find_by_id(&self, id: &str) -> Result<Option<User>>;
    }
    
    /// 通知服务端口
    #[async_trait::async_trait]
    pub trait NotificationService: Send + Sync {
        async fn send_email(&self, to: &str, subject: &str, body: &str) -> Result<()>;
    }
    
    /// 用户实体
    #[derive(Debug, Clone)]
    pub struct User {
        pub id: String,
        pub username: String,
        pub email: String,
    }
}

/// 适配器实现 - 实现端口的具体逻辑
pub mod adapters {
    use super::*;
    
    /// 数据库适配器
    pub struct DatabaseUserRepository {
        // 数据库连接等
    }
    
    #[async_trait::async_trait]
    impl ports::UserRepository for DatabaseUserRepository {
        async fn save(&self, user: ports::User) -> Result<ports::User> {
            // 数据库保存逻辑
            Ok(user)
        }
        
        async fn find_by_id(&self, id: &str) -> Result<Option<ports::User>> {
            // 数据库查询逻辑
            Ok(None)
        }
    }
    
    /// 邮件服务适配器
    pub struct EmailNotificationService {
        // 邮件服务配置等
    }
    
    #[async_trait::async_trait]
    impl ports::NotificationService for EmailNotificationService {
        async fn send_email(&self, to: &str, subject: &str, body: &str) -> Result<()> {
            // 发送邮件逻辑
            println!("发送邮件到 {}: {} - {}", to, subject, body);
            Ok(())
        }
    }
}

/// 应用服务 - 业务逻辑的核心
pub struct UserApplicationService {
    user_repository: Arc<dyn ports::UserRepository>,
    notification_service: Arc<dyn ports::NotificationService>,
}

impl UserApplicationService {
    pub fn new(
        user_repository: Arc<dyn ports::UserRepository>,
        notification_service: Arc<dyn ports::NotificationService>,
    ) -> Self {
        Self {
            user_repository,
            notification_service,
        }
    }
    
    pub async fn create_user(&self, username: String, email: String) -> Result<ports::User> {
        // 业务逻辑
        let user = ports::User {
            id: uuid::Uuid::new_v4().to_string(),
            username,
            email: email.clone(),
        };
        
        // 保存用户
        let saved_user = self.user_repository.save(user).await?;
        
        // 发送欢迎邮件
        self.notification_service
            .send_email(&saved_user.email, "欢迎", "欢迎使用我们的服务")
            .await?;
        
        Ok(saved_user)
    }
}
```

## 🔒 安全架构模式

### 认证和授权

```rust
//! 安全架构模式 - 实现认证和授权

use std::collections::HashMap;
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use anyhow::Result;
use chrono::{Duration, Utc};

/// JWT令牌
#[derive(Debug, Serialize, Deserialize)]
pub struct JwtToken {
    pub sub: String,        // 用户ID
    pub username: String,   // 用户名
    pub roles: Vec<String>, // 角色列表
    pub exp: i64,          // 过期时间
    pub iat: i64,          // 签发时间
}

/// 认证服务
pub struct AuthenticationService {
    jwt_secret: String,
    user_store: Arc<UserStore>,
}

impl AuthenticationService {
    pub fn new(jwt_secret: String, user_store: Arc<UserStore>) -> Self {
        Self {
            jwt_secret,
            user_store,
        }
    }
    
    /// 用户登录
    pub async fn login(&self, username: &str, password: &str) -> Result<String> {
        let user = self.user_store.find_by_username(username).await?
            .ok_or_else(|| anyhow::anyhow!("用户不存在"))?;
        
        if !self.verify_password(password, &user.password_hash)? {
            return Err(anyhow::anyhow!("密码错误"));
        }
        
        let token = self.generate_token(&user)?;
        Ok(token)
    }
    
    /// 验证令牌
    pub fn verify_token(&self, token: &str) -> Result<JwtToken> {
        // JWT验证逻辑
        let claims: JwtToken = jsonwebtoken::decode(
            token,
            &jsonwebtoken::DecodingKey::from_secret(self.jwt_secret.as_ref()),
            &jsonwebtoken::Validation::default(),
        )?.claims;
        
        if claims.exp < Utc::now().timestamp() {
            return Err(anyhow::anyhow!("令牌已过期"));
        }
        
        Ok(claims)
    }
    
    fn generate_token(&self, user: &User) -> Result<String> {
        let now = Utc::now();
        let claims = JwtToken {
            sub: user.id.clone(),
            username: user.username.clone(),
            roles: user.roles.clone(),
            exp: (now + Duration::hours(24)).timestamp(),
            iat: now.timestamp(),
        };
        
        let token = jsonwebtoken::encode(
            &jsonwebtoken::Header::default(),
            &claims,
            &jsonwebtoken::EncodingKey::from_secret(self.jwt_secret.as_ref()),
        )?;
        
        Ok(token)
    }
    
    fn verify_password(&self, password: &str, hash: &str) -> Result<bool> {
        // 密码验证逻辑
        Ok(password == hash) // 简化实现
    }
}

/// 授权服务
pub struct AuthorizationService {
    permissions: HashMap<String, Vec<String>>, // 角色 -> 权限列表
}

impl AuthorizationService {
    pub fn new() -> Self {
        let mut permissions = HashMap::new();
        permissions.insert("admin".to_string(), vec!["user.read".to_string(), "user.write".to_string()]);
        permissions.insert("user".to_string(), vec!["user.read".to_string()]);
        
        Self { permissions }
    }
    
    /// 检查权限
    pub fn has_permission(&self, roles: &[String], permission: &str) -> bool {
        for role in roles {
            if let Some(role_permissions) = self.permissions.get(role) {
                if role_permissions.contains(&permission.to_string()) {
                    return true;
                }
            }
        }
        false
    }
}

/// 用户存储
pub struct UserStore {
    users: HashMap<String, User>,
}

impl UserStore {
    pub fn new() -> Self {
        let mut users = HashMap::new();
        users.insert("admin".to_string(), User {
            id: "1".to_string(),
            username: "admin".to_string(),
            password_hash: "admin".to_string(),
            roles: vec!["admin".to_string()],
        });
        
        Self { users }
    }
    
    pub async fn find_by_username(&self, username: &str) -> Result<Option<User>> {
        Ok(self.users.get(username).cloned())
    }
}

#[derive(Debug, Clone)]
pub struct User {
    pub id: String,
    pub username: String,
    pub password_hash: String,
    pub roles: Vec<String>,
}
```

## 🏗️ 微服务架构

### 服务发现和注册

```rust
//! 微服务架构 - 服务发现和注册中心

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceInstance {
    pub id: String,
    pub name: String,
    pub host: String,
    pub port: u16,
    pub health_check_url: String,
    pub metadata: HashMap<String, String>,
    pub last_heartbeat: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone)]
pub struct ServiceRegistry {
    services: Arc<RwLock<HashMap<String, Vec<ServiceInstance>>>>,
    health_check_interval: Duration,
}

impl ServiceRegistry {
    pub fn new(health_check_interval: Duration) -> Self {
        Self {
            services: Arc::new(RwLock::new(HashMap::new())),
            health_check_interval,
        }
    }
    
    pub async fn register_service(&self, instance: ServiceInstance) -> Result<()> {
        let mut services = self.services.write().await;
        services
            .entry(instance.name.clone())
            .or_insert_with(Vec::new)
            .push(instance);
        Ok(())
    }
    
    pub async fn discover_services(&self, service_name: &str) -> Result<Vec<ServiceInstance>> {
        let services = self.services.read().await;
        Ok(services
            .get(service_name)
            .cloned()
            .unwrap_or_default())
    }
    
    pub async fn health_check(&self) -> Result<()> {
        let mut services = self.services.write().await;
        let now = chrono::Utc::now();
        
        for (_, instances) in services.iter_mut() {
            instances.retain(|instance| {
                now.signed_duration_since(instance.last_heartbeat) < chrono::Duration::seconds(30)
            });
        }
        
        Ok(())
    }
}
```

### API网关

```rust
//! API网关 - 请求路由和负载均衡

use axum::{
    extract::{Path, Query},
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use std::collections::HashMap;
use serde_json::Value;

pub struct ApiGateway {
    service_registry: Arc<ServiceRegistry>,
    load_balancer: Arc<LoadBalancer>,
}

impl ApiGateway {
    pub fn new(service_registry: Arc<ServiceRegistry>) -> Self {
        Self {
            service_registry,
            load_balancer: Arc::new(LoadBalancer::new()),
        }
    }
    
    pub fn create_router(self) -> Router {
        Router::new()
            .route("/api/:service/*path", get(self.proxy_get))
            .route("/api/:service/*path", post(self.proxy_post))
    }
    
    async fn proxy_get(
        &self,
        Path((service, path)): Path<(String, String)>,
        Query(params): Query<HashMap<String, String>>,
    ) -> Result<Json<Value>, StatusCode> {
        let instances = self.service_registry
            .discover_services(&service)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        
        if instances.is_empty() {
            return Err(StatusCode::SERVICE_UNAVAILABLE);
        }
        
        let selected_instance = self.load_balancer.select_instance(&instances);
        let url = format!("http://{}:{}/{}", 
            selected_instance.host, 
            selected_instance.port, 
            path
        );
        
        // 转发请求到选中的服务实例
        let response = reqwest::get(&url)
            .await
            .map_err(|_| StatusCode::BAD_GATEWAY)?;
        
        let body: Value = response
            .json()
            .await
            .map_err(|_| StatusCode::BAD_GATEWAY)?;
        
        Ok(Json(body))
    }
    
    async fn proxy_post(
        &self,
        Path((service, path)): Path<(String, String)>,
        body: String,
    ) -> Result<Json<Value>, StatusCode> {
        // 类似的POST请求处理逻辑
        Ok(Json(serde_json::json!({"status": "ok"})))
    }
}

pub struct LoadBalancer;

impl LoadBalancer {
    pub fn new() -> Self {
        Self
    }
    
    pub fn select_instance(&self, instances: &[ServiceInstance]) -> &ServiceInstance {
        // 简单的轮询负载均衡
        let index = rand::random::<usize>() % instances.len();
        &instances[index]
    }
}
```

## 🔄 响应式架构

### 响应式流处理

```rust
//! 响应式架构 - 流处理和背压控制

use tokio_stream::{Stream, StreamExt};
use tokio::sync::mpsc;
use std::pin::Pin;
use std::task::{Context, Poll};

pub struct ReactiveProcessor<T> {
    input_stream: Pin<Box<dyn Stream<Item = T> + Send>>,
    output_sender: mpsc::UnboundedSender<T>,
    buffer_size: usize,
    buffer: Vec<T>,
}

impl<T: Clone + Send + 'static> ReactiveProcessor<T> {
    pub fn new(
        input_stream: impl Stream<Item = T> + Send + 'static,
        buffer_size: usize,
    ) -> (Self, mpsc::UnboundedReceiver<T>) {
        let (sender, receiver) = mpsc::unbounded_channel();
        
        let processor = Self {
            input_stream: Box::pin(input_stream),
            output_sender: sender,
            buffer_size,
            buffer: Vec::new(),
        };
        
        (processor, receiver)
    }
    
    pub async fn process(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        while let Some(item) = self.input_stream.next().await {
            // 背压控制
            if self.buffer.len() >= self.buffer_size {
                // 处理缓冲区中的数据
                self.flush_buffer().await?;
            }
            
            self.buffer.push(item);
        }
        
        // 处理剩余数据
        self.flush_buffer().await?;
        Ok(())
    }
    
    async fn flush_buffer(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        for item in self.buffer.drain(..) {
            self.output_sender.send(item)?;
        }
        Ok(())
    }
}

// 响应式事件处理
pub struct ReactiveEventHandler {
    event_stream: mpsc::UnboundedReceiver<AppEvent>,
    processors: Vec<Box<dyn EventProcessor>>,
}

impl ReactiveEventHandler {
    pub fn new(event_stream: mpsc::UnboundedReceiver<AppEvent>) -> Self {
        Self {
            event_stream,
            processors: Vec::new(),
        }
    }
    
    pub fn add_processor(&mut self, processor: Box<dyn EventProcessor>) {
        self.processors.push(processor);
    }
    
    pub async fn start_processing(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        while let Some(event) = self.event_stream.recv().await {
            // 并行处理事件
            let futures: Vec<_> = self.processors
                .iter()
                .map(|processor| processor.process(&event))
                .collect();
            
            futures::future::join_all(futures).await;
        }
        Ok(())
    }
}

pub trait EventProcessor: Send + Sync {
    async fn process(&self, event: &AppEvent) -> Result<(), Box<dyn std::error::Error>>;
}
```

## ⚡ 性能优化架构

### 缓存架构

```rust
//! 性能优化架构 - 多级缓存系统

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use std::time::{Duration, Instant};

pub struct MultiLevelCache<K, V> {
    l1_cache: Arc<RwLock<HashMap<K, CacheEntry<V>>>>>,
    l2_cache: Arc<RwLock<HashMap<K, CacheEntry<V>>>>>,
    l1_size: usize,
    l2_size: usize,
    ttl: Duration,
}

struct CacheEntry<V> {
    value: V,
    created_at: Instant,
    access_count: u64,
}

impl<K: Eq + Hash + Clone, V: Clone> MultiLevelCache<K, V> {
    pub fn new(l1_size: usize, l2_size: usize, ttl: Duration) -> Self {
        Self {
            l1_cache: Arc::new(RwLock::new(HashMap::new())),
            l2_cache: Arc::new(RwLock::new(HashMap::new())),
            l1_size,
            l2_size,
            ttl,
        }
    }
    
    pub async fn get(&self, key: &K) -> Option<V> {
        // 首先检查L1缓存
        {
            let l1 = self.l1_cache.read().await;
            if let Some(entry) = l1.get(key) {
                if !self.is_expired(entry) {
                    return Some(entry.value.clone());
                }
            }
        }
        
        // 检查L2缓存
        {
            let l2 = self.l2_cache.read().await;
            if let Some(entry) = l2.get(key) {
                if !self.is_expired(entry) {
                    // 提升到L1缓存
                    self.promote_to_l1(key.clone(), entry.clone()).await;
                    return Some(entry.value.clone());
                }
            }
        }
        
        None
    }
    
    pub async fn set(&self, key: K, value: V) {
        let entry = CacheEntry {
            value,
            created_at: Instant::now(),
            access_count: 1,
        };
        
        // 设置到L1缓存
        self.set_to_l1(key, entry).await;
    }
    
    async fn promote_to_l1(&self, key: K, mut entry: CacheEntry<V>) {
        entry.access_count += 1;
        self.set_to_l1(key, entry).await;
    }
    
    async fn set_to_l1(&self, key: K, entry: CacheEntry<V>) {
        let mut l1 = self.l1_cache.write().await;
        
        if l1.len() >= self.l1_size {
            // 执行LRU清理
            self.evict_lru_from_l1(&mut l1).await;
        }
        
        l1.insert(key, entry);
    }
    
    async fn evict_lru_from_l1(&self, l1: &mut HashMap<K, CacheEntry<V>>) {
        if let Some((key, entry)) = l1.iter()
            .min_by_key(|(_, entry)| entry.access_count)
            .map(|(k, v)| (k.clone(), v.clone())) {
            
            l1.remove(&key);
            
            // 降级到L2缓存
            let mut l2 = self.l2_cache.write().await;
            if l2.len() >= self.l2_size {
                self.evict_lru_from_l2(&mut l2);
            }
            l2.insert(key, entry);
        }
    }
    
    async fn evict_lru_from_l2(&self, l2: &mut HashMap<K, CacheEntry<V>>) {
        if let Some((key, _)) = l2.iter()
            .min_by_key(|(_, entry)| entry.access_count)
            .map(|(k, _)| k.clone()) {
            l2.remove(&key);
        }
    }
    
    fn is_expired(&self, entry: &CacheEntry<V>) -> bool {
        Instant::now().duration_since(entry.created_at) > self.ttl
    }
}
```

### 连接池架构

```rust
//! 连接池架构 - 数据库连接管理

use std::sync::Arc;
use tokio::sync::{Semaphore, Mutex};
use std::collections::VecDeque;
use std::time::{Duration, Instant};

pub struct ConnectionPool<T> {
    connections: Arc<Mutex<VecDeque<PooledConnection<T>>>>>,
    semaphore: Arc<Semaphore>,
    max_connections: usize,
    idle_timeout: Duration,
}

struct PooledConnection<T> {
    connection: T,
    created_at: Instant,
    last_used: Instant,
}

impl<T: Clone + Send + 'static> ConnectionPool<T> {
    pub fn new(
        max_connections: usize,
        idle_timeout: Duration,
        factory: impl Fn() -> T + Send + Sync + 'static,
    ) -> Self {
        let connections = Arc::new(Mutex::new(VecDeque::new()));
        let semaphore = Arc::new(Semaphore::new(max_connections));
        
        // 启动清理任务
        let connections_clone = connections.clone();
        let idle_timeout_clone = idle_timeout;
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(60));
            loop {
                interval.tick().await;
                Self::cleanup_idle_connections(&connections_clone, idle_timeout_clone).await;
            }
        });
        
        Self {
            connections,
            semaphore,
            max_connections,
            idle_timeout,
        }
    }
    
    pub async fn get_connection(&self) -> Result<PooledConnection<T>, PoolError> {
        let _permit = self.semaphore.acquire().await.map_err(|_| PoolError::AcquireFailed)?;
        
        let mut connections = self.connections.lock().await;
        
        // 尝试从池中获取连接
        while let Some(mut pooled_conn) = connections.pop_front() {
            if !self.is_connection_expired(&pooled_conn) {
                pooled_conn.last_used = Instant::now();
                return Ok(pooled_conn);
            }
        }
        
        // 池中没有可用连接，创建新连接
        drop(connections);
        Ok(PooledConnection {
            connection: self.create_connection().await?,
            created_at: Instant::now(),
            last_used: Instant::now(),
        })
    }
    
    pub async fn return_connection(&self, mut pooled_conn: PooledConnection<T>) {
        if self.is_connection_expired(&pooled_conn) {
            return; // 连接已过期，丢弃
        }
        
        let mut connections = self.connections.lock().await;
        if connections.len() < self.max_connections {
            pooled_conn.last_used = Instant::now();
            connections.push_back(pooled_conn);
        }
    }
    
    async fn create_connection(&self) -> Result<T, PoolError> {
        // 这里应该调用实际的连接创建逻辑
        // 为了示例，我们返回一个默认值
        Err(PoolError::ConnectionFailed)
    }
    
    fn is_connection_expired(&self, pooled_conn: &PooledConnection<T>) -> bool {
        Instant::now().duration_since(pooled_conn.last_used) > self.idle_timeout
    }
    
    async fn cleanup_idle_connections(
        connections: &Arc<Mutex<VecDeque<PooledConnection<T>>>>,
        idle_timeout: Duration,
    ) {
        let mut conns = connections.lock().await;
        let now = Instant::now();
        
        conns.retain(|conn| {
            now.duration_since(conn.last_used) <= idle_timeout
        });
    }
}

#[derive(Debug)]
pub enum PoolError {
    AcquireFailed,
    ConnectionFailed,
}

pub struct PooledConnection<T> {
    pub connection: T,
    created_at: Instant,
    last_used: Instant,
}
```

---

**本文档展示了Rust 1.90中实现的各种高级架构模式，为构建企业级应用提供了完整的指导。结合最新的语言特性和成熟的架构模式，开发者可以构建出高性能、可扩展、可维护的现代应用程序。**
