//! 高级UI设计模式示例 - Rust 1.90优化
//! 
//! 本示例展示了现代UI开发中的高级设计模式：
//! - 状态管理模式 (Redux-like)
//! - 组件组合模式
//! - 渲染优化模式
//! - 错误边界模式
//! - 依赖注入模式
//! - 观察者模式
//! - 策略模式

use dioxus::prelude::*;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};

/// 应用状态 - 集中式状态管理
#[derive(Debug, Clone)]
struct AppState {
    user: UserState,
    ui: UIState,
    data: DataState,
    performance: PerformanceState,
}

/// 用户状态
#[derive(Debug, Clone)]
struct UserState {
    profile: UserProfile,
    preferences: UserPreferences,
    session: SessionInfo,
}

/// UI状态
#[derive(Debug, Clone)]
struct UIState {
    theme: Theme,
    sidebar_collapsed: bool,
    modal_open: Option<ModalType>,
    notifications: Vec<Notification>,
    loading: HashMap<String, bool>,
}

/// 数据状态
#[derive(Debug, Clone)]
struct DataState {
    todos: Vec<Todo>,
    categories: Vec<Category>,
    filters: FilterState,
    cache: HashMap<String, CachedData>,
}

/// 性能状态
#[derive(Debug, Clone)]
struct PerformanceState {
    render_count: u32,
    last_render_time: Duration,
    memory_usage: u64,
    component_stats: HashMap<String, ComponentStats>,
}

/// 用户配置文件
#[derive(Debug, Clone)]
struct UserProfile {
    id: String,
    name: String,
    email: String,
    avatar: String,
    role: UserRole,
}

/// 用户角色
#[derive(Debug, Clone, PartialEq)]
enum UserRole {
    Guest,
    User,
    Admin,
    Developer,
}

/// 用户偏好设置
#[derive(Debug, Clone)]
struct UserPreferences {
    theme: Theme,
    language: String,
    timezone: String,
    notifications: NotificationSettings,
    accessibility: AccessibilitySettings,
}

/// 会话信息
#[derive(Debug, Clone)]
struct SessionInfo {
    token: String,
    expires_at: Instant,
    last_activity: Instant,
    permissions: Vec<Permission>,
}

/// 主题枚举
#[derive(Debug, Clone, PartialEq)]
enum Theme {
    Light,
    Dark,
    Auto,
    HighContrast,
}

/// 模态框类型
#[derive(Debug, Clone, PartialEq)]
enum ModalType {
    Settings,
    Profile,
    Help,
    About,
}

/// 通知
#[derive(Debug, Clone)]
struct Notification {
    id: String,
    title: String,
    message: String,
    level: NotificationLevel,
    timestamp: Instant,
    read: bool,
}

/// 通知级别
#[derive(Debug, Clone, PartialEq)]
enum NotificationLevel {
    Info,
    Warning,
    Error,
    Success,
}

/// 通知设置
#[derive(Debug, Clone)]
struct NotificationSettings {
    email_enabled: bool,
    push_enabled: bool,
    sound_enabled: bool,
    frequency: NotificationFrequency,
}

/// 通知频率
#[derive(Debug, Clone)]
enum NotificationFrequency {
    Immediate,
    Hourly,
    Daily,
    Weekly,
}

/// 无障碍设置
#[derive(Debug, Clone)]
struct AccessibilitySettings {
    high_contrast: bool,
    large_text: bool,
    screen_reader: bool,
    keyboard_navigation: bool,
    reduced_motion: bool,
}

/// 权限
#[derive(Debug, Clone, PartialEq)]
enum Permission {
    Read,
    Write,
    Admin,
    Delete,
}

/// 待办事项
#[derive(Debug, Clone)]
struct Todo {
    id: String,
    title: String,
    description: String,
    completed: bool,
    priority: Priority,
    category: String,
    tags: Vec<String>,
    created_at: Instant,
    updated_at: Instant,
    due_date: Option<Instant>,
}

/// 优先级
#[derive(Debug, Clone, PartialEq)]
enum Priority {
    Low,
    Medium,
    High,
    Critical,
}

/// 分类
#[derive(Debug, Clone)]
struct Category {
    id: String,
    name: String,
    color: String,
    icon: String,
    todo_count: u32,
}

/// 过滤状态
#[derive(Debug, Clone)]
struct FilterState {
    search_query: String,
    selected_category: Option<String>,
    selected_priority: Option<Priority>,
    show_completed: bool,
    sort_by: SortOption,
}

/// 排序选项
#[derive(Debug, Clone)]
enum SortOption {
    Created,
    Updated,
    Priority,
    Title,
    DueDate,
}

/// 缓存数据
#[derive(Debug, Clone)]
struct CachedData {
    data: String,
    expires_at: Instant,
    access_count: u32,
}

/// 组件统计
#[derive(Debug, Clone)]
struct ComponentStats {
    render_count: u32,
    average_render_time: Duration,
    last_render_time: Duration,
    memory_usage: u64,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            user: UserState {
                profile: UserProfile {
                    id: "user_1".to_string(),
                    name: "Rust开发者".to_string(),
                    email: "developer@rust-lang.org".to_string(),
                    avatar: "👨‍💻".to_string(),
                    role: UserRole::Developer,
                },
                preferences: UserPreferences {
                    theme: Theme::Auto,
                    language: "zh-CN".to_string(),
                    timezone: "Asia/Shanghai".to_string(),
                    notifications: NotificationSettings {
                        email_enabled: true,
                        push_enabled: true,
                        sound_enabled: false,
                        frequency: NotificationFrequency::Immediate,
                    },
                    accessibility: AccessibilitySettings {
                        high_contrast: false,
                        large_text: false,
                        screen_reader: false,
                        keyboard_navigation: true,
                        reduced_motion: false,
                    },
                },
                session: SessionInfo {
                    token: "mock_token_123".to_string(),
                    expires_at: Instant::now() + Duration::from_secs(3600),
                    last_activity: Instant::now(),
                    permissions: vec![Permission::Read, Permission::Write, Permission::Admin],
                },
            },
            ui: UIState {
                theme: Theme::Auto,
                sidebar_collapsed: false,
                modal_open: None,
                notifications: vec![],
                loading: HashMap::new(),
            },
            data: DataState {
                todos: vec![
                    Todo {
                        id: "todo_1".to_string(),
                        title: "学习Rust 1.90新特性".to_string(),
                        description: "深入研究Rust 1.90的语言改进和性能优化".to_string(),
                        completed: false,
                        priority: Priority::High,
                        category: "学习".to_string(),
                        tags: vec!["rust".to_string(), "learning".to_string()],
                        created_at: Instant::now(),
                        updated_at: Instant::now(),
                        due_date: None,
                    },
                ],
                categories: vec![
                    Category {
                        id: "cat_1".to_string(),
                        name: "学习".to_string(),
                        color: "#007bff".to_string(),
                        icon: "📚".to_string(),
                        todo_count: 1,
                    },
                ],
                filters: FilterState {
                    search_query: String::new(),
                    selected_category: None,
                    selected_priority: None,
                    show_completed: false,
                    sort_by: SortOption::Created,
                },
                cache: HashMap::new(),
            },
            performance: PerformanceState {
                render_count: 0,
                last_render_time: Duration::ZERO,
                memory_usage: 128,
                component_stats: HashMap::new(),
            },
        }
    }
}

/// 动作枚举 - Redux-like状态管理
#[derive(Debug, Clone)]
enum Action {
    // 用户相关动作
    UpdateProfile(UserProfile),
    UpdatePreferences(UserPreferences),
    
    // UI相关动作
    ToggleSidebar,
    OpenModal(ModalType),
    CloseModal,
    AddNotification(Notification),
    RemoveNotification(String),
    SetLoading(String, bool),
    
    // 数据相关动作
    AddTodo(Todo),
    UpdateTodo(String, Todo),
    DeleteTodo(String),
    SetFilter(FilterState),
    
    // 性能相关动作
    UpdatePerformanceMetrics(PerformanceState),
}

/// 状态管理器
struct StateManager {
    state: Signal<AppState>,
    middleware: Vec<Box<dyn Middleware>>,
}

/// 中间件trait
trait Middleware {
    fn process(&self, action: &Action, state: &AppState) -> Option<Action>;
}

/// 日志中间件
struct LoggingMiddleware;

impl Middleware for LoggingMiddleware {
    fn process(&self, action: &Action, state: &AppState) -> Option<Action> {
        println!("Action: {:?}", action);
        None
    }
}

/// 性能中间件
struct PerformanceMiddleware;

impl Middleware for PerformanceMiddleware {
    fn process(&self, action: &Action, state: &AppState) -> Option<Action> {
        match action {
            Action::UpdatePerformanceMetrics(_) => None,
            _ => Some(Action::UpdatePerformanceMetrics(PerformanceState {
                render_count: state.performance.render_count + 1,
                last_render_time: Duration::from_millis(16),
                memory_usage: state.performance.memory_usage,
                component_stats: state.performance.component_stats.clone(),
            })),
        }
    }
}

impl StateManager {
    fn new() -> Self {
        let mut manager = Self {
            state: use_signal(|| AppState::default()),
            middleware: Vec::new(),
        };
        
        // 注册中间件
        manager.middleware.push(Box::new(LoggingMiddleware));
        manager.middleware.push(Box::new(PerformanceMiddleware));
        
        manager
    }
    
    fn dispatch(&self, action: Action) {
        let mut current_action = action;
        
        // 处理中间件
        for middleware in &self.middleware {
            if let Some(new_action) = middleware.process(&current_action, &self.state.read()) {
                current_action = new_action;
            }
        }
        
        // 更新状态
        self.state.with_mut(|state| {
            match current_action {
                Action::UpdateProfile(profile) => {
                    state.user.profile = profile;
                }
                Action::UpdatePreferences(preferences) => {
                    state.user.preferences = preferences;
                }
                Action::ToggleSidebar => {
                    state.ui.sidebar_collapsed = !state.ui.sidebar_collapsed;
                }
                Action::OpenModal(modal_type) => {
                    state.ui.modal_open = Some(modal_type);
                }
                Action::CloseModal => {
                    state.ui.modal_open = None;
                }
                Action::AddNotification(notification) => {
                    state.ui.notifications.push(notification);
                }
                Action::RemoveNotification(id) => {
                    state.ui.notifications.retain(|n| n.id != id);
                }
                Action::SetLoading(key, value) => {
                    state.ui.loading.insert(key, value);
                }
                Action::AddTodo(todo) => {
                    state.data.todos.push(todo);
                }
                Action::UpdateTodo(id, updated_todo) => {
                    if let Some(todo) = state.data.todos.iter_mut().find(|t| t.id == id) {
                        *todo = updated_todo;
                    }
                }
                Action::DeleteTodo(id) => {
                    state.data.todos.retain(|t| t.id != id);
                }
                Action::SetFilter(filter) => {
                    state.data.filters = filter;
                }
                Action::UpdatePerformanceMetrics(performance) => {
                    state.performance = performance;
                }
            }
        });
    }
}

/// 主应用组件
fn AdvancedUIApp() -> Element {
    let state_manager = use_signal(|| StateManager::new());
    
    rsx! {
        div {
            class: "app",
            style: "min-height: 100vh; background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;",
            
            // 错误边界
            ErrorBoundary {
                ErrorFallback {
                    title: "应用错误",
                    message: "发生了意外错误，请刷新页面重试。"
                }
                
                // 主布局
                MainLayout { state_manager: state_manager.clone() }
            }
        }
    }
}

/// 错误边界组件
#[component]
fn ErrorBoundary(children: Element) -> Element {
    rsx! {
        div {
            class: "error-boundary",
            {children}
        }
    }
}

/// 错误回退组件
#[component]
fn ErrorFallback(title: &'static str, message: &'static str) -> Element {
    rsx! {
        div {
            style: "display: flex; flex-direction: column; align-items: center; justify-content: center; min-height: 100vh; padding: 20px; text-align: center; color: white;",
            
            div {
                style: "background: rgba(255,255,255,0.1); padding: 30px; border-radius: 15px; backdrop-filter: blur(10px);",
                
                h1 {
                    style: "font-size: 2rem; margin: 0 0 15px 0;",
                    "⚠️ {title}"
                }
                
                p {
                    style: "font-size: 1.1rem; margin: 0 0 20px 0; opacity: 0.9;",
                    "{message}"
                }
                
                button {
                    style: "background: #007bff; color: white; border: none; padding: 12px 24px; border-radius: 8px; font-size: 1rem; cursor: pointer;",
                    onclick: move |_| {
                        // 重新加载页面
                        web_sys::window().unwrap().location().reload().unwrap();
                    },
                    "刷新页面"
                }
            }
        }
    }
}

/// 主布局组件
#[component]
fn MainLayout(state_manager: Signal<StateManager>) -> Element {
    rsx! {
        div {
            class: "main-layout",
            style: "display: flex; min-height: 100vh;",
            
            // 侧边栏
            Sidebar { state_manager: state_manager.clone() }
            
            // 主内容区域
            div {
                class: "main-content",
                style: "flex: 1; display: flex; flex-direction: column;",
                
                // 顶部导航栏
                TopNavigation { state_manager: state_manager.clone() }
                
                // 内容区域
                div {
                    class: "content-area",
                    style: "flex: 1; padding: 20px;",
                    
                    ContentArea { state_manager: state_manager.clone() }
                }
            }
        }
        
        // 模态框
        ModalContainer { state_manager: state_manager.clone() }
        
        // 通知容器
        NotificationContainer { state_manager: state_manager.clone() }
    }
}

/// 侧边栏组件
#[component]
fn Sidebar(state_manager: Signal<StateManager>) -> Element {
    let collapsed = state_manager.read().state.read().ui.sidebar_collapsed;
    
    rsx! {
        aside {
            class: "sidebar",
            style: "width: {if collapsed { '60px' } else { '250px' }}; background: rgba(255,255,255,0.95); backdrop-filter: blur(10px); transition: width 0.3s ease; padding: 20px; box-shadow: 2px 0 10px rgba(0,0,0,0.1);",
            
            // 侧边栏头部
            div {
                class: "sidebar-header",
                style: "display: flex; align-items: center; gap: 10px; margin-bottom: 30px;",
                
                div {
                    style: "font-size: 24px;",
                    "🦀"
                }
                
                if !collapsed {
                    h2 {
                        style: "margin: 0; color: #333; font-size: 18px;",
                        "Rust UI"
                    }
                }
            }
            
            // 导航菜单
            nav {
                class: "sidebar-nav",
                
                NavItem {
                    icon: "🏠",
                    label: "首页",
                    active: true,
                    collapsed: collapsed,
                    on_click: move || {}
                }
                
                NavItem {
                    icon: "📋",
                    label: "待办事项",
                    active: false,
                    collapsed: collapsed,
                    on_click: move || {}
                }
                
                NavItem {
                    icon: "📊",
                    label: "统计",
                    active: false,
                    collapsed: collapsed,
                    on_click: move || {}
                }
                
                NavItem {
                    icon: "⚙️",
                    label: "设置",
                    active: false,
                    collapsed: collapsed,
                    on_click: move || {
                        state_manager.write().dispatch(Action::OpenModal(ModalType::Settings));
                    }
                }
            }
            
            // 侧边栏底部
            div {
                class: "sidebar-footer",
                style: "margin-top: auto; padding-top: 20px;",
                
                button {
                    style: "background: none; border: none; padding: 10px; border-radius: 8px; cursor: pointer; color: #666; transition: background 0.2s;",
                    onclick: move |_| {
                        state_manager.write().dispatch(Action::ToggleSidebar);
                    },
                    
                    div {
                        style: "font-size: 20px;",
                        if collapsed { "▶️" } else { "◀️" }
                    }
                }
            }
        }
    }
}

/// 导航项组件
#[component]
fn NavItem(icon: &'static str, label: &'static str, active: bool, collapsed: bool, on_click: EventHandler<()>) -> Element {
    rsx! {
        button {
            class: "nav-item",
            style: "width: 100%; display: flex; align-items: center; gap: 12px; padding: 12px; margin-bottom: 8px; background: {if active { 'rgba(0,123,255,0.1)' } else { 'transparent' }}; border: none; border-radius: 8px; cursor: pointer; color: {if active { '#007bff' } else { '#666' }}; transition: all 0.2s;",
            onclick: move |_| on_click.call(()),
            
            div {
                style: "font-size: 20px;",
                "{icon}"
            }
            
            if !collapsed {
                span {
                    style: "font-size: 14px; font-weight: {if active { '500' } else { '400' }};",
                    "{label}"
                }
            }
        }
    }
}

/// 顶部导航栏组件
#[component]
fn TopNavigation(state_manager: Signal<StateManager>) -> Element {
    rsx! {
        header {
            class: "top-navigation",
            style: "background: rgba(255,255,255,0.95); backdrop-filter: blur(10px); padding: 15px 20px; border-bottom: 1px solid rgba(0,0,0,0.1); display: flex; justify-content: space-between; align-items: center;",
            
            // 左侧
            div {
                class: "nav-left",
                h1 {
                    style: "margin: 0; color: #333; font-size: 24px; font-weight: bold;",
                    "高级UI设计模式示例"
                }
            }
            
            // 右侧
            div {
                class: "nav-right",
                style: "display: flex; align-items: center; gap: 15px;",
                
                // 搜索框
                SearchBox { state_manager: state_manager.clone() }
                
                // 用户菜单
                UserMenu { state_manager: state_manager.clone() }
                
                // 通知按钮
                NotificationButton { state_manager: state_manager.clone() }
            }
        }
    }
}

/// 搜索框组件
#[component]
fn SearchBox(state_manager: Signal<StateManager>) -> Element {
    rsx! {
        div {
            class: "search-box",
            style: "position: relative;",
            
            input {
                r#type: "text",
                placeholder: "搜索...",
                style: "padding: 8px 12px 8px 35px; border: 1px solid #ddd; border-radius: 20px; width: 200px; font-size: 14px; outline: none; transition: border-color 0.2s;",
            }
            
            div {
                style: "position: absolute; left: 10px; top: 50%; transform: translateY(-50%); color: #999; font-size: 14px;",
                "🔍"
            }
        }
    }
}

/// 用户菜单组件
#[component]
fn UserMenu(state_manager: Signal<StateManager>) -> Element {
    let user = state_manager.read().state.read().user.profile.clone();
    
    rsx! {
        div {
            class: "user-menu",
            style: "display: flex; align-items: center; gap: 10px; cursor: pointer; padding: 8px 12px; border-radius: 20px; transition: background 0.2s;",
            onclick: move |_| {
                state_manager.write().dispatch(Action::OpenModal(ModalType::Profile));
            },
            
            div {
                style: "font-size: 24px;",
                "{user.avatar}"
            }
            
            div {
                span {
                    style: "font-size: 14px; color: #333; font-weight: 500;",
                    "{user.name}"
                }
                br {}
                span {
                    style: "font-size: 12px; color: #666;",
                    "{user.role:?}"
                }
            }
        }
    }
}

/// 通知按钮组件
#[component]
fn NotificationButton(state_manager: Signal<StateManager>) -> Element {
    let notification_count = state_manager.read().state.read().ui.notifications.len();
    
    rsx! {
        button {
            style: "background: none; border: none; padding: 10px; border-radius: 50%; cursor: pointer; color: #666; transition: background 0.2s; position: relative;",
            onclick: move |_| {
                // 显示通知列表
            },
            
            div {
                style: "font-size: 20px;",
                "🔔"
            }
            
            if notification_count > 0 {
                div {
                    style: "position: absolute; top: 5px; right: 5px; background: #dc3545; color: white; border-radius: 50%; width: 18px; height: 18px; display: flex; align-items: center; justify-content: center; font-size: 10px; font-weight: bold;",
                    "{notification_count}"
                }
            }
        }
    }
}

/// 内容区域组件
#[component]
fn ContentArea(state_manager: Signal<StateManager>) -> Element {
    rsx! {
        div {
            class: "content-area",
            
            // 仪表板
            Dashboard { state_manager: state_manager.clone() }
            
            // 待办事项列表
            TodoList { state_manager: state_manager.clone() }
            
            // 统计图表
            Statistics { state_manager: state_manager.clone() }
        }
    }
}

/// 仪表板组件
#[component]
fn Dashboard(state_manager: Signal<StateManager>) -> Element {
    let stats = state_manager.read().state.read().performance.clone();
    
    rsx! {
        div {
            class: "dashboard",
            style: "display: grid; grid-template-columns: repeat(auto-fit, minmax(200px, 1fr)); gap: 20px; margin-bottom: 30px;",
            
            StatCard {
                title: "渲染次数",
                value: "{stats.render_count}",
                icon: "🔄",
                color: "#007bff"
            }
            
            StatCard {
                title: "内存使用",
                value: "{stats.memory_usage}MB",
                icon: "💾",
                color: "#28a745"
            }
            
            StatCard {
                title: "渲染时间",
                value: "{stats.last_render_time.as_millis()}ms",
                icon: "⚡",
                color: "#ffc107"
            }
            
            StatCard {
                title: "组件数量",
                value: "{stats.component_stats.len()}",
                icon: "🧩",
                color: "#6f42c1"
            }
        }
    }
}

/// 统计卡片组件
#[component]
fn StatCard(title: &'static str, value: &'static str, icon: &'static str, color: &'static str) -> Element {
    rsx! {
        div {
            style: "background: rgba(255,255,255,0.95); padding: 20px; border-radius: 12px; box-shadow: 0 4px 15px rgba(0,0,0,0.1); text-align: center;",
            
            div {
                style: "font-size: 32px; margin-bottom: 10px;",
                "{icon}"
            }
            
            h3 {
                style: "margin: 0 0 5px 0; color: #333; font-size: 14px; font-weight: 500;",
                "{title}"
            }
            
            div {
                style: "font-size: 24px; font-weight: bold; color: {color};",
                "{value}"
            }
        }
    }
}

/// 待办事项列表组件
#[component]
fn TodoList(state_manager: Signal<StateManager>) -> Element {
    rsx! {
        div {
            class: "todo-list",
            style: "background: rgba(255,255,255,0.95); padding: 25px; border-radius: 12px; box-shadow: 0 4px 15px rgba(0,0,0,0.1); margin-bottom: 30px;",
            
            div {
                style: "display: flex; justify-content: space-between; align-items: center; margin-bottom: 20px;",
                
                h2 {
                    style: "margin: 0; color: #333; font-size: 20px;",
                    "待办事项"
                }
                
                button {
                    style: "background: #007bff; color: white; border: none; padding: 10px 20px; border-radius: 8px; cursor: pointer; font-size: 14px;",
                    onclick: move |_| {
                        // 添加新待办事项
                    },
                    "+ 添加"
                }
            }
            
            // 待办事项项
            for todo in state_manager.read().state.read().data.todos.iter() {
                TodoItem { 
                    todo: todo.clone(),
                    on_update: move |updated_todo| {
                        state_manager.write().dispatch(Action::UpdateTodo(todo.id.clone(), updated_todo));
                    },
                    on_delete: move || {
                        state_manager.write().dispatch(Action::DeleteTodo(todo.id.clone()));
                    }
                }
            }
        }
    }
}

/// 待办事项项组件
#[component]
fn TodoItem(todo: Todo, on_update: EventHandler<Todo>, on_delete: EventHandler<()>) -> Element {
    rsx! {
        div {
            style: "display: flex; align-items: center; gap: 15px; padding: 15px; border: 1px solid #eee; border-radius: 8px; margin-bottom: 10px; background: {if todo.completed { '#f8f9fa' } else { 'white' }};",
            
            input {
                r#type: "checkbox",
                checked: todo.completed,
                onchange: move |event| {
                    let mut updated_todo = todo.clone();
                    updated_todo.completed = event.checked();
                    on_update.call(updated_todo);
                },
                style: "transform: scale(1.2);"
            }
            
            div {
                style: "flex: 1;",
                
                h4 {
                    style: "margin: 0 0 5px 0; color: #333; font-size: 16px; {if todo.completed { 'text-decoration: line-through; opacity: 0.6;' } else { '' }}",
                    "{todo.title}"
                }
                
                p {
                    style: "margin: 0 0 8px 0; color: #666; font-size: 14px;",
                    "{todo.description}"
                }
                
                div {
                    style: "display: flex; gap: 8px; align-items: center;",
                    
                    span {
                        style: "background: {match todo.priority { Priority::Low => '#28a745', Priority::Medium => '#ffc107', Priority::High => '#fd7e14', Priority::Critical => '#dc3545' }}; color: white; padding: 2px 8px; border-radius: 12px; font-size: 12px;",
                        "{todo.priority:?}"
                    }
                    
                    span {
                        style: "background: #e9ecef; color: #495057; padding: 2px 8px; border-radius: 12px; font-size: 12px;",
                        "{todo.category}"
                    }
                }
            }
            
            button {
                style: "background: #dc3545; color: white; border: none; padding: 6px 12px; border-radius: 6px; cursor: pointer; font-size: 12px;",
                onclick: move |_| on_delete.call(()),
                "删除"
            }
        }
    }
}

/// 统计组件
#[component]
fn Statistics(state_manager: Signal<StateManager>) -> Element {
    rsx! {
        div {
            class: "statistics",
            style: "background: rgba(255,255,255,0.95); padding: 25px; border-radius: 12px; box-shadow: 0 4px 15px rgba(0,0,0,0.1);",
            
            h2 {
                style: "margin: 0 0 20px 0; color: #333; font-size: 20px;",
                "性能统计"
            }
            
            div {
                style: "display: grid; grid-template-columns: repeat(auto-fit, minmax(150px, 1fr)); gap: 15px;",
                
                for (component_name, stats) in state_manager.read().state.read().performance.component_stats.iter() {
                    ComponentStats { 
                        name: component_name.clone(),
                        stats: stats.clone()
                    }
                }
            }
        }
    }
}

/// 组件统计组件
#[component]
fn ComponentStats(name: String, stats: ComponentStats) -> Element {
    rsx! {
        div {
            style: "text-align: center; padding: 15px; border: 1px solid #eee; border-radius: 8px;",
            
            h4 {
                style: "margin: 0 0 10px 0; color: #333; font-size: 14px;",
                "{name}"
            }
            
            div {
                style: "display: flex; flex-direction: column; gap: 5px;",
                
                div {
                    style: "font-size: 12px; color: #666;",
                    "渲染: {stats.render_count}"
                }
                
                div {
                    style: "font-size: 12px; color: #666;",
                    "时间: {stats.average_render_time.as_millis()}ms"
                }
                
                div {
                    style: "font-size: 12px; color: #666;",
                    "内存: {stats.memory_usage}KB"
                }
            }
        }
    }
}

/// 模态框容器组件
#[component]
fn ModalContainer(state_manager: Signal<StateManager>) -> Element {
    let modal_type = state_manager.read().state.read().ui.modal_open.clone();
    
    if let Some(modal) = modal_type {
        rsx! {
            div {
                class: "modal-overlay",
                style: "position: fixed; top: 0; left: 0; right: 0; bottom: 0; background: rgba(0,0,0,0.5); display: flex; align-items: center; justify-content: center; z-index: 1000;",
                onclick: move |_| {
                    state_manager.write().dispatch(Action::CloseModal);
                },
                
                div {
                    class: "modal-content",
                    style: "background: white; padding: 30px; border-radius: 12px; max-width: 500px; width: 90%; max-height: 80vh; overflow-y: auto;",
                    onclick: move |e| e.stop_propagation(),
                    
                    match modal {
                        ModalType::Settings => SettingsModal { state_manager: state_manager.clone() },
                        ModalType::Profile => ProfileModal { state_manager: state_manager.clone() },
                        ModalType::Help => HelpModal {},
                        ModalType::About => AboutModal {},
                    }
                }
            }
        }
    }
}

/// 设置模态框组件
#[component]
fn SettingsModal(state_manager: Signal<StateManager>) -> Element {
    rsx! {
        div {
            h2 {
                style: "margin: 0 0 20px 0; color: #333;",
                "设置"
            }
            
            div {
                style: "display: flex; flex-direction: column; gap: 15px;",
                
                SettingItem {
                    label: "主题",
                    value: "{state_manager.read().state.read().user.preferences.theme:?}",
                    on_change: move |_| {}
                }
                
                SettingItem {
                    label: "语言",
                    value: "{state_manager.read().state.read().user.preferences.language}",
                    on_change: move |_| {}
                }
                
                SettingItem {
                    label: "通知",
                    value: if state_manager.read().state.read().user.preferences.notifications.email_enabled { "开启" } else { "关闭" },
                    on_change: move |_| {}
                }
            }
            
            div {
                style: "display: flex; gap: 10px; justify-content: flex-end; margin-top: 20px;",
                
                button {
                    style: "padding: 10px 20px; background: #6c757d; color: white; border: none; border-radius: 6px; cursor: pointer;",
                    onclick: move |_| {
                        state_manager.write().dispatch(Action::CloseModal);
                    },
                    "取消"
                }
                
                button {
                    style: "padding: 10px 20px; background: #007bff; color: white; border: none; border-radius: 6px; cursor: pointer;",
                    onclick: move |_| {
                        state_manager.write().dispatch(Action::CloseModal);
                    },
                    "保存"
                }
            }
        }
    }
}

/// 个人资料模态框组件
#[component]
fn ProfileModal(state_manager: Signal<StateManager>) -> Element {
    let user = state_manager.read().state.read().user.profile.clone();
    
    rsx! {
        div {
            h2 {
                style: "margin: 0 0 20px 0; color: #333;",
                "个人资料"
            }
            
            div {
                style: "text-align: center; margin-bottom: 20px;",
                
                div {
                    style: "font-size: 64px; margin-bottom: 15px;",
                    "{user.avatar}"
                }
                
                h3 {
                    style: "margin: 0 0 5px 0; color: #333;",
                    "{user.name}"
                }
                
                p {
                    style: "margin: 0; color: #666;",
                    "{user.email}"
                }
            }
            
            div {
                style: "display: flex; gap: 10px; justify-content: center;",
                
                button {
                    style: "padding: 10px 20px; background: #007bff; color: white; border: none; border-radius: 6px; cursor: pointer;",
                    onclick: move |_| {
                        state_manager.write().dispatch(Action::CloseModal);
                    },
                    "关闭"
                }
            }
        }
    }
}

/// 帮助模态框组件
#[component]
fn HelpModal() -> Element {
    rsx! {
        div {
            h2 {
                style: "margin: 0 0 20px 0; color: #333;",
                "帮助"
            }
            
            div {
                style: "line-height: 1.6; color: #666;",
                p {
                    "这是一个展示高级UI设计模式的示例应用。"
                }
                p {
                    "它包含了状态管理、组件组合、错误处理等现代前端开发的最佳实践。"
                }
                p {
                    "使用Rust 1.90的最新特性构建，展示了跨平台UI开发的强大能力。"
                }
            }
        }
    }
}

/// 关于模态框组件
#[component]
fn AboutModal() -> Element {
    rsx! {
        div {
            h2 {
                style: "margin: 0 0 20px 0; color: #333;",
                "关于"
            }
            
            div {
                style: "text-align: center;",
                
                div {
                    style: "font-size: 48px; margin-bottom: 15px;",
                    "🦀"
                }
                
                h3 {
                    style: "margin: 0 0 10px 0; color: #333;",
                    "Rust UI 高级示例"
                }
                
                p {
                    style: "margin: 0; color: #666;",
                    "版本 1.0.0"
                }
                
                p {
                    style: "margin: 10px 0 0 0; color: #666; font-size: 14px;",
                    "基于Rust 1.90和Dioxus 0.6构建"
                }
            }
        }
    }
}

/// 设置项组件
#[component]
fn SettingItem(label: &'static str, value: &'static str, on_change: EventHandler<()>) -> Element {
    rsx! {
        div {
            style: "display: flex; justify-content: space-between; align-items: center; padding: 10px 0; border-bottom: 1px solid #eee;",
            
            span {
                style: "color: #333; font-weight: 500;",
                "{label}"
            }
            
            span {
                style: "color: #666;",
                "{value}"
            }
        }
    }
}

/// 通知容器组件
#[component]
fn NotificationContainer(state_manager: Signal<StateManager>) -> Element {
    rsx! {
        div {
            class: "notification-container",
            style: "position: fixed; top: 20px; right: 20px; z-index: 1001; display: flex; flex-direction: column; gap: 10px;",
            
            for notification in state_manager.read().state.read().ui.notifications.iter() {
                NotificationToast {
                    notification: notification.clone(),
                    on_dismiss: move |id| {
                        state_manager.write().dispatch(Action::RemoveNotification(id));
                    }
                }
            }
        }
    }
}

/// 通知提示组件
#[component]
fn NotificationToast(notification: Notification, on_dismiss: EventHandler<String>) -> Element {
    rsx! {
        div {
            style: "background: white; padding: 15px 20px; border-radius: 8px; box-shadow: 0 4px 15px rgba(0,0,0,0.2); min-width: 300px; border-left: 4px solid {match notification.level { NotificationLevel::Info => '#17a2b8', NotificationLevel::Success => '#28a745', NotificationLevel::Warning => '#ffc107', NotificationLevel::Error => '#dc3545' }};",
            
            div {
                style: "display: flex; justify-content: space-between; align-items: flex-start; gap: 10px;",
                
                div {
                    style: "flex: 1;",
                    
                    h4 {
                        style: "margin: 0 0 5px 0; color: #333; font-size: 14px; font-weight: 500;",
                        "{notification.title}"
                    }
                    
                    p {
                        style: "margin: 0; color: #666; font-size: 13px; line-height: 1.4;",
                        "{notification.message}"
                    }
                }
                
                button {
                    style: "background: none; border: none; color: #999; cursor: pointer; font-size: 16px; padding: 0;",
                    onclick: move |_| on_dismiss.call(notification.id.clone()),
                    "×"
                }
            }
        }
    }
}

/// 主函数
fn main() {
    // 启动高级UI应用
    dioxus_web::launch(AdvancedUIApp);
}
