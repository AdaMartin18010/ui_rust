//! 实时通信示例 - 展示WebSocket、Server-Sent Events等实时通信技术
//! 
//! 本示例展示了如何在Rust应用中实现实时通信功能，包括：
//! - WebSocket服务器和客户端
//! - Server-Sent Events (SSE)
//! - 消息广播和房间管理
//! - 实时数据同步
//! - 连接管理和错误处理

use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};
use std::time::{Duration, Instant};
use serde::{Deserialize, Serialize};
use tokio::sync::{broadcast, mpsc, RwLock as AsyncRwLock};
use tokio::time::{sleep, interval};
use anyhow::{Result, Context};
use uuid::Uuid;

/// 消息类型
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Message {
    /// 文本消息
    Text {
        id: String,
        content: String,
        timestamp: chrono::DateTime<chrono::Utc>,
        sender: String,
    },
    /// 系统消息
    System {
        id: String,
        content: String,
        timestamp: chrono::DateTime<chrono::Utc>,
    },
    /// 用户加入
    UserJoined {
        id: String,
        username: String,
        timestamp: chrono::DateTime<chrono::Utc>,
    },
    /// 用户离开
    UserLeft {
        id: String,
        username: String,
        timestamp: chrono::DateTime<chrono::Utc>,
    },
    /// 心跳消息
    Heartbeat {
        id: String,
        timestamp: chrono::DateTime<chrono::Utc>,
    },
    /// 错误消息
    Error {
        id: String,
        message: String,
        timestamp: chrono::DateTime<chrono::Utc>,
    },
}

/// 用户会话
#[derive(Debug, Clone)]
pub struct UserSession {
    pub id: String,
    pub username: String,
    pub sender: mpsc::UnboundedSender<Message>,
    pub last_heartbeat: Instant,
    pub connected_at: Instant,
}

/// 房间管理
#[derive(Debug, Clone)]
pub struct Room {
    pub id: String,
    pub name: String,
    pub description: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub max_users: usize,
}

/// 实时通信服务器
pub struct RealtimeServer {
    /// 用户会话管理
    sessions: Arc<AsyncRwLock<HashMap<String, UserSession>>>,
    
    /// 房间管理
    rooms: Arc<AsyncRwLock<HashMap<String, Room>>>,
    
    /// 房间用户映射
    room_users: Arc<AsyncRwLock<HashMap<String, Vec<String>>>>,
    
    /// 消息广播器
    message_broadcaster: broadcast::Sender<Message>,
    
    /// 服务器配置
    config: ServerConfig,
    
    /// 统计信息
    stats: Arc<Mutex<ServerStats>>,
}

#[derive(Debug, Clone)]
pub struct ServerConfig {
    pub max_connections: usize,
    pub heartbeat_interval: Duration,
    pub heartbeat_timeout: Duration,
    pub max_message_size: usize,
    pub enable_compression: bool,
}

#[derive(Debug, Default)]
pub struct ServerStats {
    pub total_connections: u64,
    pub active_connections: u64,
    pub total_messages: u64,
    pub total_rooms: u64,
    pub uptime: Duration,
}

impl RealtimeServer {
    pub fn new(config: ServerConfig) -> Self {
        let (message_broadcaster, _) = broadcast::channel(1000);
        
        Self {
            sessions: Arc::new(AsyncRwLock::new(HashMap::new())),
            rooms: Arc::new(AsyncRwLock::new(HashMap::new())),
            room_users: Arc::new(AsyncRwLock::new(HashMap::new())),
            message_broadcaster,
            config,
            stats: Arc::new(Mutex::new(ServerStats::default())),
        }
    }
    
    /// 启动服务器
    pub async fn start(&self) -> Result<()> {
        println!("🚀 启动实时通信服务器...");
        
        // 启动心跳检查任务
        self.start_heartbeat_checker().await;
        
        // 启动统计信息更新任务
        self.start_stats_updater().await;
        
        // 启动消息处理任务
        self.start_message_processor().await;
        
        println!("✅ 实时通信服务器启动完成");
        Ok(())
    }
    
    /// 用户连接
    pub async fn user_connect(&self, username: String) -> Result<(String, mpsc::UnboundedReceiver<Message>)> {
        let session_id = Uuid::new_v4().to_string();
        let (sender, receiver) = mpsc::unbounded_channel();
        
        let session = UserSession {
            id: session_id.clone(),
            username: username.clone(),
            sender,
            last_heartbeat: Instant::now(),
            connected_at: Instant::now(),
        };
        
        // 添加会话
        {
            let mut sessions = self.sessions.write().await;
            sessions.insert(session_id.clone(), session);
        }
        
        // 更新统计信息
        {
            let mut stats = self.stats.lock().unwrap();
            stats.total_connections += 1;
            stats.active_connections += 1;
        }
        
        // 广播用户加入消息
        let join_message = Message::UserJoined {
            id: session_id.clone(),
            username: username.clone(),
            timestamp: chrono::Utc::now(),
        };
        
        self.broadcast_message(join_message).await?;
        
        println!("👤 用户连接: {} (ID: {})", username, session_id);
        
        Ok((session_id, receiver))
    }
    
    /// 用户断开连接
    pub async fn user_disconnect(&self, session_id: &str) -> Result<()> {
        let username = {
            let mut sessions = self.sessions.write().await;
            if let Some(session) = sessions.remove(session_id) {
                // 从所有房间中移除用户
                let mut room_users = self.room_users.write().await;
                for (room_id, users) in room_users.iter_mut() {
                    users.retain(|user_id| user_id != session_id);
                }
                
                session.username.clone()
            } else {
                return Ok(());
            }
        };
        
        // 更新统计信息
        {
            let mut stats = self.stats.lock().unwrap();
            stats.active_connections = stats.active_connections.saturating_sub(1);
        }
        
        // 广播用户离开消息
        let leave_message = Message::UserLeft {
            id: session_id.to_string(),
            username: username.clone(),
            timestamp: chrono::Utc::now(),
        };
        
        self.broadcast_message(leave_message).await?;
        
        println!("👋 用户断开连接: {} (ID: {})", username, session_id);
        
        Ok(())
    }
    
    /// 发送消息
    pub async fn send_message(&self, session_id: &str, content: String) -> Result<()> {
        let username = {
            let sessions = self.sessions.read().await;
            sessions.get(session_id).map(|s| s.username.clone())
        };
        
        let username = match username {
            Some(username) => username,
            None => return Err(anyhow::anyhow!("会话不存在")),
        };
        
        let message = Message::Text {
            id: Uuid::new_v4().to_string(),
            content,
            timestamp: chrono::Utc::now(),
            sender: username,
        };
        
        self.broadcast_message(message).await?;
        
        // 更新统计信息
        {
            let mut stats = self.stats.lock().unwrap();
            stats.total_messages += 1;
        }
        
        Ok(())
    }
    
    /// 广播消息
    pub async fn broadcast_message(&self, message: Message) -> Result<()> {
        let sessions = self.sessions.read().await;
        
        for (session_id, session) in sessions.iter() {
            if let Err(e) = session.sender.send(message.clone()) {
                eprintln!("发送消息失败到会话 {}: {}", session_id, e);
            }
        }
        
        Ok(())
    }
    
    /// 创建房间
    pub async fn create_room(&self, name: String, description: String, max_users: usize) -> Result<String> {
        let room_id = Uuid::new_v4().to_string();
        
        let room = Room {
            id: room_id.clone(),
            name: name.clone(),
            description,
            created_at: chrono::Utc::now(),
            max_users,
        };
        
        {
            let mut rooms = self.rooms.write().await;
            rooms.insert(room_id.clone(), room);
        }
        
        {
            let mut room_users = self.room_users.write().await;
            room_users.insert(room_id.clone(), Vec::new());
        }
        
        // 更新统计信息
        {
            let mut stats = self.stats.lock().unwrap();
            stats.total_rooms += 1;
        }
        
        println!("🏠 创建房间: {} (ID: {})", name, room_id);
        
        Ok(room_id)
    }
    
    /// 加入房间
    pub async fn join_room(&self, session_id: &str, room_id: &str) -> Result<()> {
        // 检查房间是否存在
        {
            let rooms = self.rooms.read().await;
            if !rooms.contains_key(room_id) {
                return Err(anyhow::anyhow!("房间不存在"));
            }
        }
        
        // 检查房间是否已满
        {
            let room_users = self.room_users.read().await;
            if let Some(users) = room_users.get(room_id) {
                let rooms = self.rooms.read().await;
                if let Some(room) = rooms.get(room_id) {
                    if users.len() >= room.max_users {
                        return Err(anyhow::anyhow!("房间已满"));
                    }
                }
            }
        }
        
        // 添加用户到房间
        {
            let mut room_users = self.room_users.write().await;
            if let Some(users) = room_users.get_mut(room_id) {
                if !users.contains(&session_id.to_string()) {
                    users.push(session_id.to_string());
                }
            }
        }
        
        println!("🚪 用户 {} 加入房间 {}", session_id, room_id);
        
        Ok(())
    }
    
    /// 离开房间
    pub async fn leave_room(&self, session_id: &str, room_id: &str) -> Result<()> {
        {
            let mut room_users = self.room_users.write().await;
            if let Some(users) = room_users.get_mut(room_id) {
                users.retain(|user_id| user_id != session_id);
            }
        }
        
        println!("🚪 用户 {} 离开房间 {}", session_id, room_id);
        
        Ok(())
    }
    
    /// 发送房间消息
    pub async fn send_room_message(&self, session_id: &str, room_id: &str, content: String) -> Result<()> {
        let username = {
            let sessions = self.sessions.read().await;
            sessions.get(session_id).map(|s| s.username.clone())
        };
        
        let username = match username {
            Some(username) => username,
            None => return Err(anyhow::anyhow!("会话不存在")),
        };
        
        // 检查用户是否在房间中
        {
            let room_users = self.room_users.read().await;
            if let Some(users) = room_users.get(room_id) {
                if !users.contains(&session_id.to_string()) {
                    return Err(anyhow::anyhow!("用户不在房间中"));
                }
            }
        }
        
        let message = Message::Text {
            id: Uuid::new_v4().to_string(),
            content,
            timestamp: chrono::Utc::now(),
            sender: username,
        };
        
        // 发送消息给房间中的所有用户
        {
            let room_users = self.room_users.read().await;
            if let Some(users) = room_users.get(room_id) {
                let sessions = self.sessions.read().await;
                
                for user_id in users {
                    if let Some(session) = sessions.get(user_id) {
                        if let Err(e) = session.sender.send(message.clone()) {
                            eprintln!("发送房间消息失败到用户 {}: {}", user_id, e);
                        }
                    }
                }
            }
        }
        
        Ok(())
    }
    
    /// 处理心跳
    pub async fn handle_heartbeat(&self, session_id: &str) -> Result<()> {
        {
            let mut sessions = self.sessions.write().await;
            if let Some(session) = sessions.get_mut(session_id) {
                session.last_heartbeat = Instant::now();
            }
        }
        
        let heartbeat_message = Message::Heartbeat {
            id: session_id.to_string(),
            timestamp: chrono::Utc::now(),
        };
        
        self.broadcast_message(heartbeat_message).await?;
        
        Ok(())
    }
    
    /// 获取服务器统计信息
    pub fn get_stats(&self) -> ServerStats {
        self.stats.lock().unwrap().clone()
    }
    
    /// 获取在线用户列表
    pub async fn get_online_users(&self) -> Vec<String> {
        let sessions = self.sessions.read().await;
        sessions.values().map(|s| s.username.clone()).collect()
    }
    
    /// 获取房间列表
    pub async fn get_rooms(&self) -> Vec<Room> {
        let rooms = self.rooms.read().await;
        rooms.values().cloned().collect()
    }
    
    /// 获取房间用户列表
    pub async fn get_room_users(&self, room_id: &str) -> Vec<String> {
        let room_users = self.room_users.read().await;
        let sessions = self.sessions.read().await;
        
        if let Some(user_ids) = room_users.get(room_id) {
            user_ids.iter()
                .filter_map(|user_id| sessions.get(user_id).map(|s| s.username.clone()))
                .collect()
        } else {
            Vec::new()
        }
    }
    
    // 私有方法
    
    /// 启动心跳检查器
    async fn start_heartbeat_checker(&self) {
        let sessions = self.sessions.clone();
        let config = self.config.clone();
        
        tokio::spawn(async move {
            let mut interval = interval(config.heartbeat_interval);
            
            loop {
                interval.tick().await;
                
                let now = Instant::now();
                let mut expired_sessions = Vec::new();
                
                {
                    let sessions_guard = sessions.read().await;
                    for (session_id, session) in sessions_guard.iter() {
                        if now.duration_since(session.last_heartbeat) > config.heartbeat_timeout {
                            expired_sessions.push(session_id.clone());
                        }
                    }
                }
                
                // 清理过期会话
                for session_id in expired_sessions {
                    let mut sessions_guard = sessions.write().await;
                    sessions_guard.remove(&session_id);
                    println!("⏰ 清理过期会话: {}", session_id);
                }
            }
        });
    }
    
    /// 启动统计信息更新器
    async fn start_stats_updater(&self) {
        let stats = self.stats.clone();
        let start_time = Instant::now();
        
        tokio::spawn(async move {
            let mut interval = interval(Duration::from_secs(60));
            
            loop {
                interval.tick().await;
                
                let mut stats_guard = stats.lock().unwrap();
                stats_guard.uptime = start_time.elapsed();
            }
        });
    }
    
    /// 启动消息处理器
    async fn start_message_processor(&self) {
        let mut receiver = self.message_broadcaster.subscribe();
        
        tokio::spawn(async move {
            while let Ok(message) = receiver.recv().await {
                match message {
                    Message::Text { content, sender, .. } => {
                        println!("💬 消息: {} 说: {}", sender, content);
                    }
                    Message::UserJoined { username, .. } => {
                        println!("👋 用户加入: {}", username);
                    }
                    Message::UserLeft { username, .. } => {
                        println!("👋 用户离开: {}", username);
                    }
                    Message::Heartbeat { .. } => {
                        // 心跳消息不需要处理
                    }
                    _ => {
                        println!("📨 其他消息: {:?}", message);
                    }
                }
            }
        });
    }
}

/// WebSocket客户端
pub struct WebSocketClient {
    pub id: String,
    pub username: String,
    pub server: Arc<RealtimeServer>,
    pub message_receiver: mpsc::UnboundedReceiver<Message>,
}

impl WebSocketClient {
    pub fn new(username: String, server: Arc<RealtimeServer>) -> Result<Self> {
        let (session_id, message_receiver) = tokio::runtime::Runtime::new()?
            .block_on(server.user_connect(username.clone()))?;
        
        Ok(Self {
            id: session_id,
            username,
            server,
            message_receiver,
        })
    }
    
    /// 发送消息
    pub async fn send_message(&self, content: String) -> Result<()> {
        self.server.send_message(&self.id, content).await
    }
    
    /// 加入房间
    pub async fn join_room(&self, room_id: &str) -> Result<()> {
        self.server.join_room(&self.id, room_id).await
    }
    
    /// 离开房间
    pub async fn leave_room(&self, room_id: &str) -> Result<()> {
        self.server.leave_room(&self.id, room_id).await
    }
    
    /// 发送房间消息
    pub async fn send_room_message(&self, room_id: &str, content: String) -> Result<()> {
        self.server.send_room_message(&self.id, room_id, content).await
    }
    
    /// 发送心跳
    pub async fn send_heartbeat(&self) -> Result<()> {
        self.server.handle_heartbeat(&self.id).await
    }
    
    /// 接收消息
    pub async fn receive_message(&mut self) -> Option<Message> {
        self.message_receiver.recv().await
    }
    
    /// 断开连接
    pub async fn disconnect(self) -> Result<()> {
        self.server.user_disconnect(&self.id).await
    }
}

/// Server-Sent Events 实现
pub struct ServerSentEvents {
    pub server: Arc<RealtimeServer>,
    pub event_stream: mpsc::UnboundedReceiver<Message>,
}

impl ServerSentEvents {
    pub fn new(server: Arc<RealtimeServer>) -> Self {
        let (_, event_stream) = server.message_broadcaster.subscribe();
        
        Self {
            server,
            event_stream,
        }
    }
    
    /// 获取事件流
    pub async fn get_event_stream(&mut self) -> Option<Message> {
        self.event_stream.recv().await
    }
}

/// 主函数 - 演示实时通信功能
#[tokio::main]
async fn main() -> Result<()> {
    // 初始化日志
    env_logger::init();
    
    // 创建服务器配置
    let config = ServerConfig {
        max_connections: 1000,
        heartbeat_interval: Duration::from_secs(30),
        heartbeat_timeout: Duration::from_secs(60),
        max_message_size: 1024 * 1024, // 1MB
        enable_compression: true,
    };
    
    // 创建服务器
    let server = Arc::new(RealtimeServer::new(config));
    
    // 启动服务器
    server.start().await?;
    
    // 创建房间
    let room_id = server.create_room(
        "测试房间".to_string(),
        "这是一个测试房间".to_string(),
        10,
    ).await?;
    
    // 创建客户端
    let client1 = WebSocketClient::new("用户1".to_string(), server.clone())?;
    let client2 = WebSocketClient::new("用户2".to_string(), server.clone())?;
    
    // 客户端加入房间
    client1.join_room(&room_id).await?;
    client2.join_room(&room_id).await?;
    
    // 发送消息
    client1.send_room_message(&room_id, "大家好！".to_string()).await?;
    client2.send_room_message(&room_id, "你好！".to_string()).await?;
    
    // 等待消息
    tokio::time::sleep(Duration::from_secs(1)).await;
    
    // 获取统计信息
    let stats = server.get_stats();
    println!("📊 服务器统计信息: {:?}", stats);
    
    // 获取在线用户
    let online_users = server.get_online_users().await;
    println!("👥 在线用户: {:?}", online_users);
    
    // 获取房间列表
    let rooms = server.get_rooms().await;
    println!("🏠 房间列表: {:?}", rooms);
    
    // 获取房间用户
    let room_users = server.get_room_users(&room_id).await;
    println!("👥 房间用户: {:?}", room_users);
    
    // 保持服务器运行
    tokio::signal::ctrl_c().await?;
    println!("🛑 服务器已停止");
    
    Ok(())
}
