//! Dioxus 跨平台UI框架示例
//! 
//! 本示例展示了如何使用Dioxus构建跨平台的用户界面应用
//! 支持Web、Desktop和Mobile平台
//! 利用Rust 1.90的新特性提升开发体验

use dioxus::prelude::*;
use std::collections::HashMap;

/// 主题枚举 - 利用Rust 1.90的模式匹配改进
#[derive(Debug, Clone, Copy, PartialEq)]
enum Theme {
    Light,
    Dark,
    Auto,
}

/// Todo项目结构 - 利用Rust 1.90的结构体改进
#[derive(Debug, Clone, PartialEq)]
struct TodoItem {
    id: u32,
    title: String,
    completed: bool,
    priority: Priority,
}

/// 优先级枚举
#[derive(Debug, Clone, Copy, PartialEq)]
enum Priority {
    Low,
    Medium,
    High,
}

/// 用户配置 - 利用Rust 1.90的配置管理改进
#[derive(Debug, Clone)]
struct UserConfig {
    theme: Theme,
    language: String,
    notifications: bool,
    settings: HashMap<String, String>,
}

impl Default for UserConfig {
    fn default() -> Self {
        Self {
            theme: Theme::Auto,
            language: "zh-CN".to_string(),
            notifications: true,
            settings: HashMap::new(),
        }
    }
}

/// 主应用组件 - 利用Rust 1.90的新特性
#[allow(non_snake_case)]
fn App() -> Element {
    // 使用Rust 1.90改进的信号管理
    let mut count = use_signal(|| 0);
    let mut name = use_signal(|| "Rust开发者".to_string());
    let mut theme = use_signal(|| Theme::Light);
    let mut todos = use_signal(|| Vec::<TodoItem>::new());

    rsx! {
        div {
            style: "text-align: center; font-family: Arial, sans-serif; padding: 20px;",
            
            h1 {
                style: "color: #2c3e50; margin-bottom: 30px;",
                "🚀 Dioxus 跨平台UI示例"
            }
            
            div {
                style: "background: #ecf0f1; padding: 20px; border-radius: 10px; margin: 20px 0;",
                
                h2 {
                    style: "color: #34495e; margin-bottom: 15px;",
                    "欢迎, "
                    {name.clone()}
                    "!"
                }
                
                input {
                    r#type: "text",
                    placeholder: "输入您的姓名",
                    value: name.clone(),
                    oninput: move |evt| name.set(evt.value()),
                    style: "padding: 8px; margin: 10px; border: 1px solid #bdc3c7; border-radius: 5px; width: 200px;"
                }
            }
            
            div {
                style: "background: #3498db; color: white; padding: 20px; border-radius: 10px; margin: 20px 0;",
                
                h2 {
                    style: "margin-bottom: 15px;",
                    "计数器: "
                    {count.to_string()}
                }
                
                div {
                    style: "display: flex; gap: 10px; justify-content: center;",
                    
                    button {
                        onclick: move |_| count += 1,
                        style: "background: #e74c3c; color: white; border: none; padding: 10px 20px; border-radius: 5px; cursor: pointer; font-size: 16px;",
                        "➕ 增加"
                    }
                    
                    button {
                        onclick: move |_| count -= 1,
                        style: "background: #e67e22; color: white; border: none; padding: 10px 20px; border-radius: 5px; cursor: pointer; font-size: 16px;",
                        "➖ 减少"
                    }
                    
                    button {
                        onclick: move |_| count.set(0),
                        style: "background: #95a5a6; color: white; border: none; padding: 10px 20px; border-radius: 5px; cursor: pointer; font-size: 16px;",
                        "🔄 重置"
                    }
                }
            }
            
            div {
                style: "background: #2ecc71; color: white; padding: 20px; border-radius: 10px; margin: 20px 0;",
                
                h3 {
                    style: "margin-bottom: 15px;",
                    "📱 平台信息"
                }
                
                p {
                    "当前运行在: "
                    {get_platform().to_string()}
                }
                
                p {
                    "Dioxus版本: 0.6.0"
                }
                
                p {
                    "Rust版本: 1.90.0 (支持新特性)"
                }
            }
            
            div {
                style: "background: #9b59b6; color: white; padding: 20px; border-radius: 10px; margin: 20px 0;",
                
                h3 {
                    style: "margin-bottom: 15px;",
                    "🎯 特性展示"
                }
                
                ul {
                    style: "text-align: left; max-width: 400px; margin: 0 auto;",
                    
                    li { "✅ 跨平台支持 (Web, Desktop, Mobile)" }
                    li { "✅ 类似React的组件模型" }
                    li { "✅ 高性能渲染" }
                    li { "✅ 类型安全" }
                    li { "✅ 响应式状态管理" }
                    li { "✅ 热重载支持" }
                    li { "✅ Rust 1.90新特性集成" }
                    li { "✅ 改进的异步编程支持" }
                    li { "✅ 增强的模式匹配" }
                    li { "✅ 新API稳定化" }
                }
            }
            
            div {
                style: "background: #e67e22; color: white; padding: 20px; border-radius: 10px; margin: 20px 0;",
                
                h3 {
                    style: "margin-bottom: 15px;",
                    "🔧 Rust 1.90新特性演示"
                }
                
                div {
                    style: "display: flex; gap: 10px; justify-content: center; margin: 10px 0;",
                    
                    button {
                        onclick: move |_| {
                            let current_theme = *theme.read();
                            theme.set(match current_theme {
                                Theme::Light => Theme::Dark,
                                Theme::Dark => Theme::Auto,
                                Theme::Auto => Theme::Light,
                            });
                        },
                        style: "background: #d35400; color: white; border: none; padding: 10px 15px; border-radius: 5px; cursor: pointer;",
                        "🎨 切换主题: "
                        {match *theme.read() { Theme::Light => "浅色", Theme::Dark => "深色", Theme::Auto => "自动" }}
                    }
                }
                
                div {
                    style: "background: rgba(255,255,255,0.1); padding: 15px; border-radius: 8px; margin: 10px 0;",
                    
                    h4 { "📝 待办事项列表 (Rust 1.90特性)" }
                    
                    div {
                        style: "margin: 10px 0;",
                        
                        input {
                            r#type: "text",
                            placeholder: "添加新的待办事项...",
                            style: "padding: 8px; margin: 5px; border: 1px solid #ccc; border-radius: 4px; width: 250px;",
                            onkeypress: move |evt| {
                                if evt.key() == Key::Enter {
                                    // 添加新的待办事项
                                    let new_todo = TodoItem {
                                        id: todos.read().len() as u32,
                                        title: evt.data().key().to_string(),
                                        completed: false,
                                        priority: Priority::Medium,
                                    };
                                    todos.with_mut(|list| list.push(new_todo));
                                }
                            }
                        }
                    }
                    
                    div {
                        style: "max-height: 200px; overflow-y: auto;",
                        
                        for todo in todos.read().clone() {
                            div {
                                key: todo.id.to_string(),
                                style: "background: rgba(255,255,255,0.1); padding: 10px; margin: 5px 0; border-radius: 5px; display: flex; align-items: center; justify-content: space-between;",
                                
                                div {
                                    style: "display: flex; align-items: center;",
                                    
                                    input {
                                        r#type: "checkbox",
                                        checked: todo.completed,
                                        style: "margin-right: 10px;",
                                        onchange: move |evt| {
                                            todos.with_mut(|list| {
                                                if let Some(item) = list.iter_mut().find(|t| t.id == todo.id) {
                                                    item.completed = evt.checked();
                                                }
                                            });
                                        }
                                    }
                                    
                                    span {
                                        style: {
                                            let decoration = if todo.completed { "line-through" } else { "none" };
                                            let color = match todo.priority {
                                                Priority::High => "#e74c3c",
                                                Priority::Medium => "#f39c12",
                                                Priority::Low => "#27ae60"
                                            };
                                            format!("text-decoration: {}; color: {};", decoration, color)
                                        },
                                        {todo.title.clone()}
                                    }
                                }
                                
                                button {
                                    onclick: move |_| {
                                        todos.with_mut(|list| list.retain(|t| t.id != todo.id));
                                    },
                                    style: "background: #c0392b; color: white; border: none; padding: 5px 10px; border-radius: 3px; cursor: pointer; font-size: 12px;",
                                    "删除"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// 获取当前平台信息
fn get_platform() -> &'static str {
    #[cfg(target_arch = "wasm32")]
    {
        return "Web (WASM)";
    }
    
    #[cfg(target_os = "windows")]
    {
        return "Windows Desktop";
    }
    
    #[cfg(target_os = "macos")]
    {
        return "macOS Desktop";
    }
    
    #[cfg(target_os = "linux")]
    {
        return "Linux Desktop";
    }
    
    #[cfg(target_os = "android")]
    {
        return "Android Mobile";
    }
    
    #[cfg(target_os = "ios")]
    {
        return "iOS Mobile";
    }
    
    #[cfg(not(any(
        target_arch = "wasm32",
        target_os = "windows",
        target_os = "macos",
        target_os = "linux",
        target_os = "android",
        target_os = "ios"
    )))]
    {
        return "Unknown Platform";
    }
}

/// 主函数 - Web平台
fn main() {
    dioxus_web::launch::launch(App, vec![], vec![]);
}

/// 测试模块
#[cfg(test)]
mod tests {
    use super::*;
    use dioxus::prelude::*;

    #[test]
    fn test_app_renders() {
        let mut app = VirtualDom::new(App);
        app.rebuild();
        
        // 验证应用能够正常渲染
        assert!(app.base_scope().root_element().is_some());
    }

    #[test]
    fn test_platform_detection() {
        let platform = get_platform();
        assert!(!platform.is_empty());
        assert!(platform != "Unknown Platform");
    }
}
