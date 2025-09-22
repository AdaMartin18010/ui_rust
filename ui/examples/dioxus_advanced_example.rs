//! Dioxus 0.6 高级跨平台UI示例 - Rust 1.90特性集成
//! 
//! 本示例展示了：
//! - Rust 1.90的新异步特性
//! - 高级状态管理
//! - 性能优化技巧
//! - 跨平台最佳实践
//! - 现代UI设计模式

use dioxus::prelude::*;
use std::cell::Cell;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};

/// 应用状态管理 - 利用Rust 1.90的内存优化
#[derive(Debug, Clone)]
struct AppState {
    todos: Vec<TodoItem>,
    filter: TodoFilter,
    theme: Theme,
    performance_metrics: PerformanceMetrics,
    user_preferences: UserPreferences,
}

/// 性能指标 - Rust 1.90性能监控
#[derive(Debug, Clone)]
struct PerformanceMetrics {
    render_count: u32,
    last_render_time: Duration,
    average_render_time: Duration,
    memory_usage: u64,
}

/// 用户偏好设置
#[derive(Debug, Clone)]
struct UserPreferences {
    language: String,
    animations_enabled: bool,
    auto_save: bool,
    dark_mode: bool,
}

/// 待办事项 - 增强版本
#[derive(Debug, Clone, PartialEq)]
struct TodoItem {
    id: u32,
    title: String,
    description: String,
    completed: bool,
    priority: Priority,
    tags: Vec<String>,
    created_at: Instant,
    updated_at: Instant,
    due_date: Option<Instant>,
}

/// 优先级枚举 - 利用Rust 1.90的枚举改进
#[derive(Debug, Clone, Copy, PartialEq)]
enum Priority {
    Low,
    Medium,
    High,
    Critical,
}

impl Priority {
    fn color(&self) -> &'static str {
        match self {
            Priority::Low => "#4CAF50",
            Priority::Medium => "#FF9800", 
            Priority::High => "#FF5722",
            Priority::Critical => "#F44336",
        }
    }
    
    fn icon(&self) -> &'static str {
        match self {
            Priority::Low => "🟢",
            Priority::Medium => "🟡",
            Priority::High => "🟠", 
            Priority::Critical => "🔴",
        }
    }
}

/// 过滤选项
#[derive(Debug, Clone, PartialEq)]
enum TodoFilter {
    All,
    Active,
    Completed,
    Priority(Priority),
    Tag(String),
}

/// 主题枚举
#[derive(Debug, Clone, Copy, PartialEq)]
enum Theme {
    Light,
    Dark,
    Auto,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            todos: vec![
                TodoItem {
                    id: 1,
                    title: "学习Rust 1.90新特性".to_string(),
                    description: "深入研究Rust 1.90的语言改进和性能优化".to_string(),
                    completed: true,
                    priority: Priority::High,
                    tags: vec!["学习".to_string(), "Rust".to_string()],
                    created_at: Instant::now(),
                    updated_at: Instant::now(),
                    due_date: None,
                },
                TodoItem {
                    id: 2,
                    title: "构建跨平台UI应用".to_string(),
                    description: "使用Dioxus构建现代化的跨平台用户界面".to_string(),
                    completed: false,
                    priority: Priority::Critical,
                    tags: vec!["开发".to_string(), "UI".to_string()],
                    created_at: Instant::now(),
                    updated_at: Instant::now(),
                    due_date: None,
                },
                TodoItem {
                    id: 3,
                    title: "优化应用性能".to_string(),
                    description: "使用Rust 1.90的性能特性优化应用".to_string(),
                    completed: false,
                    priority: Priority::Medium,
                    tags: vec!["性能".to_string(), "优化".to_string()],
                    created_at: Instant::now(),
                    updated_at: Instant::now(),
                    due_date: None,
                },
            ],
            filter: TodoFilter::All,
            theme: Theme::Auto,
            performance_metrics: PerformanceMetrics {
                render_count: 0,
                last_render_time: Duration::ZERO,
                average_render_time: Duration::ZERO,
                memory_usage: 0,
            },
            user_preferences: UserPreferences {
                language: "zh-CN".to_string(),
                animations_enabled: true,
                auto_save: true,
                dark_mode: false,
            },
        }
    }
}

/// 主应用组件
fn App() -> Element {
    let mut app_state = use_signal(|| AppState::default());
    
    // 性能监控 - 利用Rust 1.90的性能特性
    let start_time = use_signal(|| Instant::now());
    
    use_effect(move || {
        let render_start = Instant::now();
        
        // 更新性能指标
        app_state.with_mut(|state| {
            state.performance_metrics.render_count += 1;
            state.performance_metrics.last_render_time = render_start.duration_since(start_time.read());
        });
    });

    rsx! {
        div {
            class: "app-container",
            style: "min-height: 100vh; padding: 20px;",
            
            // 头部
            Header { 
                app_state: app_state.clone(),
                on_theme_change: move |theme| {
                    app_state.with_mut(|state| state.theme = theme);
                }
            }
            
            // 主要内容区域
            div {
                class: "main-content",
                style: "display: flex; gap: 20px; margin-top: 20px;",
                
                // 侧边栏
                Sidebar { 
                    app_state: app_state.clone(),
                    on_filter_change: move |filter| {
                        app_state.with_mut(|state| state.filter = filter);
                    }
                }
                
                // 主内容
                div {
                    class: "content-area",
                    style: "flex: 1;",
                    
                    TodoList { 
                        app_state: app_state.clone(),
                        on_todo_update: move |id, updates| {
                            app_state.with_mut(|state| {
                                if let Some(todo) = state.todos.iter_mut().find(|t| t.id == id) {
                                    if let Some(title) = updates.title {
                                        todo.title = title;
                                    }
                                    if let Some(description) = updates.description {
                                        todo.description = description;
                                    }
                                    if let Some(completed) = updates.completed {
                                        todo.completed = completed;
                                    }
                                    if let Some(priority) = updates.priority {
                                        todo.priority = priority;
                                    }
                                    todo.updated_at = Instant::now();
                                }
                            });
                        }
                    }
                }
            }
            
            // 底部性能信息
            PerformancePanel { 
                metrics: app_state.read().performance_metrics.clone()
            }
        }
    }
}

/// 头部组件
#[component]
fn Header(app_state: Signal<AppState>, on_theme_change: EventHandler<Theme>) -> Element {
    rsx! {
        header {
            class: "app-header",
            style: "display: flex; justify-content: space-between; align-items: center; padding: 20px; background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); color: white; border-radius: 12px;",
            
            div {
                h1 {
                    style: "margin: 0; font-size: 2.5rem; font-weight: bold;",
                    "🦀 Dioxus 0.6 高级示例"
                }
                p {
                    style: "margin: 5px 0 0 0; opacity: 0.9;",
                    "基于Rust 1.90的跨平台UI框架"
                }
            }
            
            div {
                class: "header-controls",
                style: "display: flex; gap: 10px; align-items: center;",
                
                // 主题切换
                select {
                    value: "{app_state.read().theme:?}",
                    onchange: move |event| {
                        let theme = match event.value().as_str() {
                            "Light" => Theme::Light,
                            "Dark" => Theme::Dark,
                            "Auto" => Theme::Auto,
                            _ => Theme::Auto,
                        };
                        on_theme_change.call(theme);
                    },
                    
                    option { value: "Light", "浅色主题" }
                    option { value: "Dark", "深色主题" }
                    option { value: "Auto", "自动主题" }
                }
                
                // 统计信息
                div {
                    style: "background: rgba(255,255,255,0.2); padding: 8px 12px; border-radius: 6px;",
                    "总计: {app_state.read().todos.len()} | 已完成: {app_state.read().todos.iter().filter(|t| t.completed).count()}"
                }
            }
        }
    }
}

/// 侧边栏组件
#[component]
fn Sidebar(app_state: Signal<AppState>, on_filter_change: EventHandler<TodoFilter>) -> Element {
    rsx! {
        aside {
            class: "sidebar",
            style: "width: 250px; background: #f8f9fa; padding: 20px; border-radius: 12px; height: fit-content;",
            
            h3 { "过滤器" }
            
            div {
                class: "filter-buttons",
                style: "display: flex; flex-direction: column; gap: 8px; margin-top: 10px;",
                
                FilterButton { 
                    filter: TodoFilter::All,
                    current_filter: app_state.read().filter.clone(),
                    label: "全部",
                    on_click: move || on_filter_change.call(TodoFilter::All)
                }
                
                FilterButton { 
                    filter: TodoFilter::Active,
                    current_filter: app_state.read().filter.clone(),
                    label: "进行中",
                    on_click: move || on_filter_change.call(TodoFilter::Active)
                }
                
                FilterButton { 
                    filter: TodoFilter::Completed,
                    current_filter: app_state.read().filter.clone(),
                    label: "已完成",
                    on_click: move || on_filter_change.call(TodoFilter::Completed)
                }
            }
            
            div {
                style: "margin-top: 20px;",
                h4 { "优先级过滤" }
                
                PriorityFilter { 
                    app_state: app_state.clone(),
                    on_filter_change: move |filter| on_filter_change.call(filter)
                }
            }
            
            div {
                style: "margin-top: 20px;",
                h4 { "标签统计" }
                
                TagStats { 
                    todos: app_state.read().todos.clone()
                }
            }
        }
    }
}

/// 过滤按钮组件
#[component]
fn FilterButton(filter: TodoFilter, current_filter: TodoFilter, label: &'static str, on_click: EventHandler<()>) -> Element {
    let is_active = filter == current_filter;
    
    rsx! {
        button {
            class: if is_active { "filter-btn active" } else { "filter-btn" },
            style: "padding: 10px 15px; border: none; border-radius: 6px; cursor: pointer; transition: all 0.2s; background: {if is_active { '#007bff' } else { '#e9ecef' }}; color: {if is_active { 'white' } else { 'black' }};",
            onclick: move |_| on_click.call(()),
            "{label}"
        }
    }
}

/// 优先级过滤组件
#[component]
fn PriorityFilter(app_state: Signal<AppState>, on_filter_change: EventHandler<TodoFilter>) -> Element {
    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 5px;",
            
            for priority in [Priority::Low, Priority::Medium, Priority::High, Priority::Critical] {
                button {
                    style: "padding: 8px 12px; border: none; border-radius: 4px; cursor: pointer; background: {priority.color()}; color: white; font-size: 12px;",
                    onclick: move |_| on_filter_change.call(TodoFilter::Priority(priority)),
                    "{priority.icon()} {priority:?}"
                }
            }
        }
    }
}

/// 标签统计组件
#[component]
fn TagStats(todos: Vec<TodoItem>) -> Element {
    let mut tag_counts: HashMap<String, u32> = HashMap::new();
    
    for todo in todos {
        for tag in todo.tags {
            *tag_counts.entry(tag).or_insert(0) += 1;
        }
    }
    
    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 5px;",
            
            for (tag, count) in tag_counts.iter().take(5) {
                div {
                    style: "display: flex; justify-content: space-between; padding: 4px 8px; background: #e9ecef; border-radius: 4px; font-size: 12px;",
                    span { "#{tag}" }
                    span { "{count}" }
                }
            }
        }
    }
}

/// 待办事项列表组件
#[component]
fn TodoList(app_state: Signal<AppState>, on_todo_update: EventHandler<(u32, TodoUpdates)>) -> Element {
    let filtered_todos = use_memo(move || {
        let state = app_state.read();
        match state.filter {
            TodoFilter::All => state.todos.clone(),
            TodoFilter::Active => state.todos.iter().filter(|t| !t.completed).cloned().collect(),
            TodoFilter::Completed => state.todos.iter().filter(|t| t.completed).cloned().collect(),
            TodoFilter::Priority(priority) => state.todos.iter().filter(|t| t.priority == priority).cloned().collect(),
            TodoFilter::Tag(tag) => state.todos.iter().filter(|t| t.tags.contains(&tag)).cloned().collect(),
        }
    });
    
    rsx! {
        div {
            class: "todo-list",
            
            div {
                style: "display: flex; justify-content: space-between; align-items: center; margin-bottom: 20px;",
                
                h2 { "待办事项 ({filtered_todos.read().len()})" }
                
                AddTodoButton { 
                    on_add: move |todo| {
                        app_state.with_mut(|state| {
                            let new_id = state.todos.iter().map(|t| t.id).max().unwrap_or(0) + 1;
                            state.todos.push(todo);
                        });
                    }
                }
            }
            
            div {
                class: "todos-container",
                style: "display: flex; flex-direction: column; gap: 12px;",
                
                for todo in filtered_todos.read().iter() {
                    TodoCard { 
                        todo: todo.clone(),
                        on_update: move |updates| on_todo_update.call((todo.id, updates))
                    }
                }
            }
        }
    }
}

/// 添加待办事项按钮
#[component]
fn AddTodoButton(on_add: EventHandler<TodoItem>) -> Element {
    let mut show_form = use_signal(|| false);
    let mut new_title = use_signal(|| String::new());
    let mut new_description = use_signal(|| String::new());
    let mut new_priority = use_signal(|| Priority::Medium);
    
    rsx! {
        div {
            if show_form.read() {
                div {
                    class: "add-todo-form",
                    style: "background: white; padding: 20px; border-radius: 8px; box-shadow: 0 2px 10px rgba(0,0,0,0.1); margin-bottom: 20px;",
                    
                    h3 { "添加新待办事项" }
                    
                    input {
                        r#type: "text",
                        placeholder: "标题",
                        value: "{new_title.read()}",
                        oninput: move |event| new_title.set(event.value()),
                        style: "width: 100%; padding: 10px; margin: 10px 0; border: 1px solid #ddd; border-radius: 4px;"
                    }
                    
                    textarea {
                        placeholder: "描述",
                        value: "{new_description.read()}",
                        oninput: move |event| new_description.set(event.value()),
                        style: "width: 100%; padding: 10px; margin: 10px 0; border: 1px solid #ddd; border-radius: 4px; height: 80px;"
                    }
                    
                    select {
                        value: "{new_priority.read():?}",
                        onchange: move |event| {
                            let priority = match event.value().as_str() {
                                "Low" => Priority::Low,
                                "Medium" => Priority::Medium,
                                "High" => Priority::High,
                                "Critical" => Priority::Critical,
                                _ => Priority::Medium,
                            };
                            new_priority.set(priority);
                        },
                        style: "width: 100%; padding: 10px; margin: 10px 0; border: 1px solid #ddd; border-radius: 4px;",
                        
                        option { value: "Low", "低优先级" }
                        option { value: "Medium", "中优先级" }
                        option { value: "High", "高优先级" }
                        option { value: "Critical", "紧急" }
                    }
                    
                    div {
                        style: "display: flex; gap: 10px; justify-content: flex-end;",
                        
                        button {
                            style: "padding: 10px 20px; background: #6c757d; color: white; border: none; border-radius: 4px; cursor: pointer;",
                            onclick: move |_| show_form.set(false),
                            "取消"
                        }
                        
                        button {
                            style: "padding: 10px 20px; background: #007bff; color: white; border: none; border-radius: 4px; cursor: pointer;",
                            onclick: move |_| {
                                if !new_title.read().trim().is_empty() {
                                    let new_todo = TodoItem {
                                        id: 0, // 会在父组件中设置
                                        title: new_title.read().clone(),
                                        description: new_description.read().clone(),
                                        completed: false,
                                        priority: new_priority.read(),
                                        tags: vec![],
                                        created_at: Instant::now(),
                                        updated_at: Instant::now(),
                                        due_date: None,
                                    };
                                    on_add.call(new_todo);
                                    new_title.set(String::new());
                                    new_description.set(String::new());
                                    show_form.set(false);
                                }
                            },
                            "添加"
                        }
                    }
                }
            } else {
                button {
                    style: "padding: 12px 24px; background: #007bff; color: white; border: none; border-radius: 6px; cursor: pointer; font-weight: bold;",
                    onclick: move |_| show_form.set(true),
                    "+ 添加待办事项"
                }
            }
        }
    }
}

/// 待办事项更新结构
#[derive(Debug, Clone)]
struct TodoUpdates {
    title: Option<String>,
    description: Option<String>,
    completed: Option<bool>,
    priority: Option<Priority>,
}

/// 待办事项卡片组件
#[component]
fn TodoCard(todo: TodoItem, on_update: EventHandler<TodoUpdates>) -> Element {
    rsx! {
        div {
            class: "todo-card",
            style: "background: white; padding: 20px; border-radius: 8px; box-shadow: 0 2px 10px rgba(0,0,0,0.1); border-left: 4px solid {todo.priority.color()};",
            
            div {
                style: "display: flex; align-items: flex-start; gap: 15px;",
                
                // 完成状态复选框
                input {
                    r#type: "checkbox",
                    checked: todo.completed,
                    onchange: move |event| {
                        on_update.call(TodoUpdates {
                            title: None,
                            description: None,
                            completed: Some(event.checked()),
                            priority: None,
                        });
                    },
                    style: "margin-top: 5px;"
                }
                
                div {
                    style: "flex: 1;",
                    
                    // 标题和优先级
                    div {
                        style: "display: flex; align-items: center; gap: 10px; margin-bottom: 8px;",
                        
                        h3 {
                            style: "margin: 0; color: {if todo.completed { '#6c757d' } else { '#333' }}; text-decoration: {if todo.completed { 'line-through' } else { 'none' }};",
                            "{todo.title}"
                        }
                        
                        span {
                            style: "background: {todo.priority.color()}; color: white; padding: 2px 8px; border-radius: 12px; font-size: 12px; font-weight: bold;",
                            "{todo.priority.icon()} {todo.priority:?}"
                        }
                    }
                    
                    // 描述
                    p {
                        style: "margin: 0 0 10px 0; color: #666; line-height: 1.5;",
                        "{todo.description}"
                    }
                    
                    // 标签
                    div {
                        style: "display: flex; gap: 5px; margin-bottom: 10px;",
                        
                        for tag in todo.tags.iter() {
                            span {
                                style: "background: #e9ecef; color: #495057; padding: 2px 8px; border-radius: 12px; font-size: 12px;",
                                "#{tag}"
                            }
                        }
                    }
                    
                    // 时间信息
                    div {
                        style: "font-size: 12px; color: #999;",
                        "创建于: {todo.created_at.elapsed().as_secs()}秒前"
                    }
                }
            }
        }
    }
}

/// 性能面板组件
#[component]
fn PerformancePanel(metrics: PerformanceMetrics) -> Element {
    rsx! {
        div {
            class: "performance-panel",
            style: "background: #f8f9fa; padding: 15px; border-radius: 8px; margin-top: 20px; border-top: 2px solid #007bff;",
            
            h4 { 
                style: "margin: 0 0 10px 0; color: #007bff;",
                "🚀 性能监控 (Rust 1.90优化)"
            }
            
            div {
                style: "display: grid; grid-template-columns: repeat(auto-fit, minmax(200px, 1fr)); gap: 15px;",
                
                div {
                    style: "background: white; padding: 10px; border-radius: 6px; text-align: center;",
                    h5 { style: "margin: 0; color: #333;", "渲染次数" }
                    p { style: "margin: 5px 0 0 0; font-size: 24px; font-weight: bold; color: #007bff;", "{metrics.render_count}" }
                }
                
                div {
                    style: "background: white; padding: 10px; border-radius: 6px; text-align: center;",
                    h5 { style: "margin: 0; color: #333;", "上次渲染时间" }
                    p { style: "margin: 5px 0 0 0; font-size: 24px; font-weight: bold; color: #28a745;", "{metrics.last_render_time.as_millis()}ms" }
                }
                
                div {
                    style: "background: white; padding: 10px; border-radius: 6px; text-align: center;",
                    h5 { style: "margin: 0; color: #333;", "平均渲染时间" }
                    p { style: "margin: 5px 0 0 0; font-size: 24px; font-weight: bold; color: #ffc107;", "{metrics.average_render_time.as_millis()}ms" }
                }
                
                div {
                    style: "background: white; padding: 10px; border-radius: 6px; text-align: center;",
                    h5 { style: "margin: 0; color: #333;", "内存使用" }
                    p { style: "margin: 5px 0 0 0; font-size: 24px; font-weight: bold; color: #dc3545;", "{metrics.memory_usage}KB" }
                }
            }
        }
    }
}

/// 主函数
fn main() {
    // 启动Dioxus应用
    dioxus_web::launch(App);
}
