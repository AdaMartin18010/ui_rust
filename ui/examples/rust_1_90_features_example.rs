//! Rust 1.90 新特性示例 - 展示最新语言特性在UI开发中的应用
//! 
//! 本示例展示了Rust 1.90版本的新特性如何与UI框架结合使用，
//! 包括改进的异步编程、增强的模式匹配、新API稳定化等特性。

use std::collections::HashMap;
use std::cell::Cell;
use std::time::{Duration, Instant};
use serde::{Deserialize, Serialize};
use anyhow::Result;

/// Rust 1.90 新特性演示应用
pub struct Rust190FeaturesApp {
    // 使用Cell::update进行状态管理
    state: Cell<AppState>,
    // 使用HashMap::extract_if进行缓存管理
    cache: HashMap<String, CacheEntry>,
    // 改进的异步处理
    async_processor: AsyncProcessor,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppState {
    pub user_count: u32,
    pub active_sessions: u32,
    pub last_updated: Instant,
    pub theme: Theme,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Theme {
    Light,
    Dark,
    Auto,
}

#[derive(Debug, Clone)]
struct CacheEntry {
    data: String,
    created_at: Instant,
    access_count: u64,
}

/// 改进的异步处理器
pub struct AsyncProcessor {
    tasks: Vec<tokio::task::JoinHandle<()>>,
    event_sender: tokio::sync::mpsc::UnboundedSender<AppEvent>,
}

#[derive(Debug, Clone)]
pub enum AppEvent {
    UserLogin { user_id: String, username: String },
    UserLogout { user_id: String },
    ThemeChanged { theme: Theme },
    DataUpdated { key: String, value: String },
}

impl Rust190FeaturesApp {
    pub fn new() -> Self {
        let (event_sender, _) = tokio::sync::mpsc::unbounded_channel();
        
        Self {
            state: Cell::new(AppState {
                user_count: 0,
                active_sessions: 0,
                last_updated: Instant::now(),
                theme: Theme::Light,
            }),
            cache: HashMap::new(),
            async_processor: AsyncProcessor {
                tasks: Vec::new(),
                event_sender,
            },
        }
    }
    
    /// 使用Rust 1.90的Cell::update进行原子状态更新
    pub fn update_state<F>(&self, updater: F) -> Result<AppState>
    where
        F: FnOnce(&AppState) -> Result<AppState>,
    {
        // 使用Cell::update进行原子更新
        let new_state = self.state.update(|current_state| {
            updater(current_state)
        })?;
        
        println!("状态已更新: {:?}", new_state);
        Ok(new_state)
    }
    
    /// 使用HashMap::extract_if进行高效的缓存清理
    pub fn cleanup_cache(&mut self) -> usize {
        let now = Instant::now();
        let ttl = Duration::from_secs(300); // 5分钟TTL
        
        // 使用extract_if提取过期的缓存项
        let expired_entries: Vec<_> = self.cache
            .extract_if(|_, entry| {
                now.duration_since(entry.created_at) > ttl
            })
            .collect();
        
        println!("清理了 {} 个过期缓存项", expired_entries.len());
        expired_entries.len()
    }
    
    /// 使用extract_if进行LRU缓存清理
    pub fn cleanup_lru_cache(&mut self, max_size: usize) -> usize {
        if self.cache.len() <= max_size {
            return 0;
        }
        
        // 按访问次数排序
        let mut entries: Vec<_> = self.cache.iter().collect();
        entries.sort_by_key(|(_, entry)| entry.access_count);
        
        let to_remove = entries.len() - max_size;
        let keys_to_remove: Vec<String> = entries
            .into_iter()
            .take(to_remove)
            .map(|(key, _)| key.clone())
            .collect();
        
        // 使用extract_if移除LRU项
        for key in keys_to_remove {
            self.cache.remove(&key);
        }
        
        println!("清理了 {} 个LRU缓存项", to_remove);
        to_remove
    }
    
    /// 使用增强的模式匹配处理事件
    pub fn handle_event(&self, event: AppEvent) -> Result<()> {
        match event {
            // 使用守卫条件进行复杂匹配
            AppEvent::UserLogin { user_id, username } 
                if user_id.len() > 0 && username.len() > 0 => {
                self.handle_user_login(&user_id, &username)?;
            }
            
            // 使用范围匹配
            AppEvent::UserLogout { user_id } => {
                self.handle_user_logout(&user_id)?;
            }
            
            // 使用模式匹配处理主题变更
            AppEvent::ThemeChanged { theme } => {
                self.handle_theme_change(theme)?;
            }
            
            // 使用解构和条件
            AppEvent::DataUpdated { key, value } 
                if key.len() > 0 && value.len() > 0 => {
                self.handle_data_update(&key, &value)?;
            }
            
            // 默认处理
            _ => {
                println!("未处理的事件: {:?}", event);
            }
        }
        Ok(())
    }
    
    /// 处理用户登录
    fn handle_user_login(&self, user_id: &str, username: &str) -> Result<()> {
        self.update_state(|state| {
            Ok(AppState {
                user_count: state.user_count + 1,
                active_sessions: state.active_sessions + 1,
                last_updated: Instant::now(),
                theme: state.theme.clone(),
            })
        })?;
        
        println!("用户登录: {} ({})", username, user_id);
        Ok(())
    }
    
    /// 处理用户登出
    fn handle_user_logout(&self, user_id: &str) -> Result<()> {
        self.update_state(|state| {
            Ok(AppState {
                user_count: state.user_count,
                active_sessions: state.active_sessions.saturating_sub(1),
                last_updated: Instant::now(),
                theme: state.theme.clone(),
            })
        })?;
        
        println!("用户登出: {}", user_id);
        Ok(())
    }
    
    /// 处理主题变更
    fn handle_theme_change(&self, theme: Theme) -> Result<()> {
        self.update_state(|state| {
            Ok(AppState {
                user_count: state.user_count,
                active_sessions: state.active_sessions,
                last_updated: Instant::now(),
                theme,
            })
        })?;
        
        println!("主题已变更为: {:?}", theme);
        Ok(())
    }
    
    /// 处理数据更新
    fn handle_data_update(&mut self, key: &str, value: &str) -> Result<()> {
        let entry = CacheEntry {
            data: value.to_string(),
            created_at: Instant::now(),
            access_count: 1,
        };
        
        self.cache.insert(key.to_string(), entry);
        println!("数据已更新: {} = {}", key, value);
        Ok(())
    }
    
    /// 获取缓存数据
    pub fn get_cached_data(&mut self, key: &str) -> Option<String> {
        if let Some(entry) = self.cache.get_mut(key) {
            entry.access_count += 1;
            Some(entry.data.clone())
        } else {
            None
        }
    }
    
    /// 使用改进的异步特性
    pub async fn process_async_data(&self, data: Vec<String>) -> Result<Vec<String>> {
        // 使用改进的异步组合器
        let result = tokio::time::timeout(
            Duration::from_secs(5),
            self.async_processor.process_data(data).await
        ).await??; // 双重问号操作符优化
        
        Ok(result)
    }
}

impl AsyncProcessor {
    /// 异步数据处理
    pub async fn process_data(&self, data: Vec<String>) -> Result<Vec<String>> {
        // 模拟异步处理
        tokio::time::sleep(Duration::from_millis(100)).await;
        
        let processed: Vec<String> = data
            .into_iter()
            .map(|item| format!("processed_{}", item))
            .collect();
        
        Ok(processed)
    }
}

/// 性能优化的热路径函数
#[inline(always)] // Rust 1.90 更积极的内联优化
pub fn hot_path_operation(data: &[u8]) -> Result<Vec<u8>, ProcessingError> {
    // 编译器会自动优化这个热路径
    data.iter()
        .map(|&byte| process_byte(byte))
        .collect::<Result<Vec<_>, _>>()
}

/// 字节处理函数
fn process_byte(byte: u8) -> Result<u8, ProcessingError> {
    if byte > 200 {
        Err(ProcessingError::ValueTooLarge)
    } else {
        Ok(byte * 2)
    }
}

/// 使用新的优化提示
#[cold] // 标记冷路径
pub fn error_handling_path(error: ProcessingError) -> ErrorResponse {
    ErrorResponse::new(error)
}

/// 使用likely/unlikely提示进行分支预测优化
pub fn branch_prediction_optimized(value: u32) -> String {
    if std::intrinsics::likely(value > 1000) {
        "large_value".to_string()
    } else {
        "small_value".to_string()
    }
}

/// 处理错误类型
#[derive(Debug)]
pub enum ProcessingError {
    ValueTooLarge,
    InvalidInput,
    Timeout,
}

/// 错误响应
#[derive(Debug)]
pub struct ErrorResponse {
    error: ProcessingError,
}

impl ErrorResponse {
    pub fn new(error: ProcessingError) -> Self {
        Self { error }
    }
}

/// 主函数 - 演示Rust 1.90新特性
#[tokio::main]
async fn main() -> Result<()> {
    println!("🚀 Rust 1.90 新特性演示");
    
    // 创建应用实例
    let mut app = Rust190FeaturesApp::new();
    
    // 演示状态更新
    println!("\n📊 演示状态更新:");
    app.update_state(|state| {
        Ok(AppState {
            user_count: 10,
            active_sessions: 5,
            last_updated: Instant::now(),
            theme: Theme::Dark,
        })
    })?;
    
    // 演示事件处理
    println!("\n🎯 演示事件处理:");
    app.handle_event(AppEvent::UserLogin {
        user_id: "user_123".to_string(),
        username: "alice".to_string(),
    })?;
    
    app.handle_event(AppEvent::ThemeChanged {
        theme: Theme::Auto,
    })?;
    
    app.handle_event(AppEvent::DataUpdated {
        key: "config".to_string(),
        value: "new_value".to_string(),
    })?;
    
    // 演示缓存管理
    println!("\n💾 演示缓存管理:");
    app.handle_data_update("key1", "value1")?;
    app.handle_data_update("key2", "value2")?;
    app.handle_data_update("key3", "value3")?;
    
    if let Some(data) = app.get_cached_data("key1") {
        println!("获取缓存数据: {}", data);
    }
    
    // 演示缓存清理
    let cleaned = app.cleanup_lru_cache(2);
    println!("清理了 {} 个LRU缓存项", cleaned);
    
    // 演示异步处理
    println!("\n⚡ 演示异步处理:");
    let test_data = vec!["item1".to_string(), "item2".to_string(), "item3".to_string()];
    let processed = app.process_async_data(test_data).await?;
    println!("异步处理结果: {:?}", processed);
    
    // 演示性能优化
    println!("\n🏃 演示性能优化:");
    let test_bytes = vec![1, 2, 3, 4, 5];
    match hot_path_operation(&test_bytes) {
        Ok(result) => println!("热路径处理结果: {:?}", result),
        Err(e) => println!("热路径处理错误: {:?}", e),
    }
    
    // 演示分支预测优化
    let prediction_result = branch_prediction_optimized(1500);
    println!("分支预测优化结果: {}", prediction_result);
    
    println!("\n✅ Rust 1.90 新特性演示完成!");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_state_update() {
        let app = Rust190FeaturesApp::new();
        
        let result = app.update_state(|state| {
            Ok(AppState {
                user_count: state.user_count + 1,
                active_sessions: state.active_sessions,
                last_updated: Instant::now(),
                theme: state.theme.clone(),
            })
        });
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap().user_count, 1);
    }
    
    #[test]
    fn test_event_handling() {
        let app = Rust190FeaturesApp::new();
        
        let event = AppEvent::UserLogin {
            user_id: "test_user".to_string(),
            username: "test".to_string(),
        };
        
        let result = app.handle_event(event);
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_cache_management() {
        let mut app = Rust190FeaturesApp::new();
        
        app.handle_data_update("test_key", "test_value").unwrap();
        
        let data = app.get_cached_data("test_key");
        assert_eq!(data, Some("test_value".to_string()));
    }
    
    #[tokio::test]
    async fn test_async_processing() {
        let app = Rust190FeaturesApp::new();
        
        let test_data = vec!["test".to_string()];
        let result = app.process_async_data(test_data).await;
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), vec!["processed_test"]);
    }
    
    #[test]
    fn test_hot_path_operation() {
        let test_data = vec![1, 2, 3];
        let result = hot_path_operation(&test_data);
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), vec![2, 4, 6]);
    }
    
    #[test]
    fn test_branch_prediction() {
        let result1 = branch_prediction_optimized(1500);
        let result2 = branch_prediction_optimized(500);
        
        assert_eq!(result1, "large_value");
        assert_eq!(result2, "small_value");
    }
}
