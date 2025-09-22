//! 无障碍支持示例 - Rust 1.90优化
//! 
//! 本示例展示了如何在Rust UI应用中实现完整的无障碍支持：
//! - 屏幕阅读器支持
//! - 键盘导航
//! - 高对比度模式
//! - 大字体支持
//! - 语音控制
//! - 焦点管理
//! - ARIA标签支持
//! - 语义化HTML

use dioxus::prelude::*;
use std::collections::HashMap;

/// 无障碍设置
#[derive(Debug, Clone)]
struct AccessibilitySettings {
    screen_reader: bool,
    high_contrast: bool,
    large_text: bool,
    reduced_motion: bool,
    keyboard_navigation: bool,
    voice_control: bool,
    focus_indicators: bool,
    color_blind_friendly: bool,
}

impl Default for AccessibilitySettings {
    fn default() -> Self {
        Self {
            screen_reader: false,
            high_contrast: false,
            large_text: false,
            reduced_motion: false,
            keyboard_navigation: true,
            voice_control: false,
            focus_indicators: true,
            color_blind_friendly: false,
        }
    }
}

/// 应用状态
#[derive(Debug, Clone)]
struct AccessibilityAppState {
    settings: AccessibilitySettings,
    current_focus: Option<String>,
    announcements: Vec<Announcement>,
    user_preferences: UserPreferences,
    demo_content: DemoContent,
}

/// 用户偏好
#[derive(Debug, Clone)]
struct UserPreferences {
    preferred_language: String,
    font_size: FontSize,
    color_scheme: ColorScheme,
    input_method: InputMethod,
}

/// 字体大小
#[derive(Debug, Clone, PartialEq)]
enum FontSize {
    Small,
    Medium,
    Large,
    ExtraLarge,
}

/// 配色方案
#[derive(Debug, Clone, PartialEq)]
enum ColorScheme {
    Default,
    HighContrast,
    Dark,
    Light,
    ColorBlindFriendly,
}

/// 输入方法
#[derive(Debug, Clone, PartialEq)]
enum InputMethod {
    Mouse,
    Keyboard,
    Touch,
    Voice,
    Switch,
}

/// 公告
#[derive(Debug, Clone)]
struct Announcement {
    id: String,
    message: String,
    priority: AnnouncementPriority,
    timestamp: u64,
}

/// 公告优先级
#[derive(Debug, Clone, PartialEq)]
enum AnnouncementPriority {
    Low,
    Medium,
    High,
    Critical,
}

/// 演示内容
#[derive(Debug, Clone)]
struct DemoContent {
    articles: Vec<Article>,
    navigation_items: Vec<NavigationItem>,
    form_data: FormData,
    interactive_elements: Vec<InteractiveElement>,
}

/// 文章
#[derive(Debug, Clone)]
struct Article {
    id: String,
    title: String,
    content: String,
    author: String,
    publish_date: String,
    tags: Vec<String>,
}

/// 导航项
#[derive(Debug, Clone)]
struct NavigationItem {
    id: String,
    label: String,
    href: String,
    description: String,
    current: bool,
}

/// 表单数据
#[derive(Debug, Clone)]
struct FormData {
    name: String,
    email: String,
    message: String,
    newsletter: bool,
    accessibility_feedback: String,
}

/// 交互元素
#[derive(Debug, Clone)]
struct InteractiveElement {
    id: String,
    type_: ElementType,
    label: String,
    description: String,
    state: ElementState,
}

/// 元素类型
#[derive(Debug, Clone)]
enum ElementType {
    Button,
    Link,
    Input,
    Select,
    Checkbox,
    Radio,
    Tab,
    Accordion,
}

/// 元素状态
#[derive(Debug, Clone)]
enum ElementState {
    Normal,
    Focused,
    Selected,
    Disabled,
    Expanded,
    Collapsed,
}

impl Default for AccessibilityAppState {
    fn default() -> Self {
        Self {
            settings: AccessibilitySettings::default(),
            current_focus: None,
            announcements: vec![
                Announcement {
                    id: "welcome".to_string(),
                    message: "欢迎使用无障碍演示应用".to_string(),
                    priority: AnnouncementPriority::Medium,
                    timestamp: 0,
                },
            ],
            user_preferences: UserPreferences {
                preferred_language: "zh-CN".to_string(),
                font_size: FontSize::Medium,
                color_scheme: ColorScheme::Default,
                input_method: InputMethod::Mouse,
            },
            demo_content: DemoContent {
                articles: vec![
                    Article {
                        id: "article_1".to_string(),
                        title: "Rust 1.90无障碍开发指南".to_string(),
                        content: "本文介绍了如何在Rust 1.90中实现无障碍的用户界面开发...".to_string(),
                        author: "Rust无障碍团队".to_string(),
                        publish_date: "2025-01-15".to_string(),
                        tags: vec!["Rust".to_string(), "无障碍".to_string(), "UI".to_string()],
                    },
                ],
                navigation_items: vec![
                    NavigationItem {
                        id: "nav_home".to_string(),
                        label: "首页".to_string(),
                        href: "#home".to_string(),
                        description: "返回应用主页".to_string(),
                        current: true,
                    },
                    NavigationItem {
                        id: "nav_articles".to_string(),
                        label: "文章".to_string(),
                        href: "#articles".to_string(),
                        description: "浏览所有文章".to_string(),
                        current: false,
                    },
                    NavigationItem {
                        id: "nav_settings".to_string(),
                        label: "设置".to_string(),
                        href: "#settings".to_string(),
                        description: "访问应用设置".to_string(),
                        current: false,
                    },
                ],
                form_data: FormData {
                    name: String::new(),
                    email: String::new(),
                    message: String::new(),
                    newsletter: false,
                    accessibility_feedback: String::new(),
                },
                interactive_elements: vec![
                    InteractiveElement {
                        id: "demo_button".to_string(),
                        type_: ElementType::Button,
                        label: "演示按钮".to_string(),
                        description: "这是一个演示无障碍功能的按钮".to_string(),
                        state: ElementState::Normal,
                    },
                ],
            },
        }
    }
}

/// 主无障碍应用
fn AccessibilityApp() -> Element {
    let mut app_state = use_signal(|| AccessibilityAppState::default());
    
    // 键盘事件处理
    use_effect(move || {
        // 模拟键盘事件处理
        app_state.with_mut(|state| {
            // 更新焦点管理
            if state.current_focus.is_none() {
                state.current_focus = Some("nav_home".to_string());
            }
        });
    });

    rsx! {
        div {
            class: "accessibility-app",
            style: "min-height: 100vh; font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;",
            
            // 无障碍样式
            AccessibilityStyles { 
                settings: app_state.read().settings.clone(),
                preferences: app_state.read().user_preferences.clone()
            }
            
            // 跳过链接
            SkipLinks { app_state: app_state.clone() }
            
            // 公告区域
            AnnouncementsRegion { 
                announcements: app_state.read().announcements.clone()
            }
            
            // 主布局
            div {
                class: "main-layout",
                style: "display: flex; min-height: 100vh;",
                
                // 侧边导航
                Navigation { 
                    app_state: app_state.clone()
                }
                
                // 主内容区域
                main {
                    class: "main-content",
                    style: "flex: 1; display: flex; flex-direction: column;",
                    role: "main",
                    "aria-label": "主要内容",
                    
                    // 页面标题
                    h1 {
                        style: "margin: 20px; font-size: 2rem;",
                        "无障碍演示应用"
                    }
                    
                    // 内容标签页
                    TabContainer { 
                        app_state: app_state.clone()
                    }
                }
            }
            
            // 无障碍工具栏
            AccessibilityToolbar { 
                app_state: app_state.clone()
            }
        }
    }
}

/// 无障碍样式组件
#[component]
fn AccessibilityStyles(settings: AccessibilitySettings, preferences: UserPreferences) -> Element {
    let font_size = match preferences.font_size {
        FontSize::Small => "14px",
        FontSize::Medium => "16px",
        FontSize::Large => "18px",
        FontSize::ExtraLarge => "20px",
    };
    
    let color_scheme = match preferences.color_scheme {
        ColorScheme::Default => "normal",
        ColorScheme::HighContrast => "high-contrast",
        ColorScheme::Dark => "dark",
        ColorScheme::Light => "light",
        ColorScheme::ColorBlindFriendly => "color-blind-friendly",
    };

    rsx! {
        style { r#"
            :root {{
                --font-size-base: {font_size};
                --color-scheme: {color_scheme};
                --focus-color: {if settings.focus_indicators { '#007bff' } else { 'transparent' }};
                --high-contrast: {if settings.high_contrast { '1' } else { '0' }};
                --reduced-motion: {if settings.reduced_motion { '1' } else { '0' }};
            }}
            
            * {{
                box-sizing: border-box;
            }}
            
            body {{
                font-size: var(--font-size-base);
                line-height: 1.6;
            }}
            
            /* 焦点指示器 */
            *:focus {{
                outline: 2px solid var(--focus-color);
                outline-offset: 2px;
            }}
            
            /* 高对比度模式 */
            @media (prefers-contrast: high) {{
                .high-contrast {{
                    filter: contrast(2) brightness(1.2);
                }}
            }}
            
            /* 减少动画 */
            @media (prefers-reduced-motion: reduce) {{
                * {{
                    animation-duration: 0.01ms !important;
                    animation-iteration-count: 1 !important;
                    transition-duration: 0.01ms !important;
                }}
            }}
            
            /* 大字体支持 */
            .large-text {{
                font-size: calc(var(--font-size-base) * 1.25);
                line-height: 1.8;
            }}
            
            .extra-large-text {{
                font-size: calc(var(--font-size-base) * 1.5);
                line-height: 2.0;
            }}
            
            /* 键盘导航 */
            .keyboard-nav button:focus,
            .keyboard-nav a:focus {{
                background-color: #e3f2fd;
                border: 2px solid #2196f3;
            }}
            
            /* 屏幕阅读器专用内容 */
            .sr-only {{
                position: absolute;
                width: 1px;
                height: 1px;
                padding: 0;
                margin: -1px;
                overflow: hidden;
                clip: rect(0, 0, 0, 0);
                white-space: nowrap;
                border: 0;
            }}
            
            /* 色盲友好配色 */
            .color-blind-friendly {{
                --primary-color: #0066cc;
                --success-color: #009900;
                --warning-color: #ff6600;
                --error-color: #cc0000;
            }}
        "# }
    }
}

/// 跳过链接组件
#[component]
fn SkipLinks(app_state: Signal<AccessibilityAppState>) -> Element {
    rsx! {
        div {
            class: "skip-links",
            style: "position: absolute; top: -40px; left: 6px; z-index: 1000;",
            
            a {
                href: "#main-content",
                class: "skip-link",
                style: "position: absolute; top: -40px; left: 6px; background: #000; color: #fff; padding: 8px; text-decoration: none; border-radius: 4px; z-index: 1000; transition: top 0.2s;",
                onfocus: move |_| {
                    // 显示跳过链接
                },
                "跳过导航"
            }
            
            a {
                href: "#main-content",
                class: "skip-link",
                style: "position: absolute; top: -40px; left: 100px; background: #000; color: #fff; padding: 8px; text-decoration: none; border-radius: 4px; z-index: 1000; transition: top 0.2s;",
                "跳转到主要内容"
            }
        }
    }
}

/// 公告区域组件
#[component]
fn AnnouncementsRegion(announcements: Vec<Announcement>) -> Element {
    rsx! {
        div {
            "aria-live": "polite",
            "aria-atomic": "true",
            class: "sr-only",
            id: "announcements",
            
            for announcement in announcements.iter() {
                div {
                    key: "{announcement.id}",
                    "aria-label": "公告",
                    "{announcement.message}"
                }
            }
        }
    }
}

/// 导航组件
#[component]
fn Navigation(app_state: Signal<AccessibilityAppState>) -> Element {
    rsx! {
        nav {
            class: "sidebar-navigation",
            style: "width: 250px; background: #f8f9fa; border-right: 1px solid #dee2e6; padding: 20px;",
            role: "navigation",
            "aria-label": "主导航",
            
            ul {
                style: "list-style: none; padding: 0; margin: 0;",
                role: "menubar",
                
                for item in app_state.read().demo_content.navigation_items.iter() {
                    li {
                        key: "{item.id}",
                        role: "none",
                        
                        a {
                            href: "{item.href}",
                            role: "menuitem",
                            "aria-current": if item.current { "page" } else { "false" },
                            "aria-describedby": "{item.id}_desc",
                            style: "display: block; padding: 12px 16px; text-decoration: none; color: #333; border-radius: 6px; transition: background-color 0.2s; {if item.current { 'background-color: #e3f2fd; font-weight: bold;' } else { '' }}",
                            
                            "{item.label}"
                            
                            span {
                                id: "{item.id}_desc",
                                class: "sr-only",
                                " - {item.description}"
                            }
                        }
                    }
                }
            }
            
            // 无障碍设置快捷方式
            div {
                style: "margin-top: 30px; padding-top: 20px; border-top: 1px solid #dee2e6;",
                
                h3 {
                    style: "margin: 0 0 15px 0; font-size: 16px;",
                    "无障碍设置"
                }
                
                AccessibilitySettingsPanel { 
                    app_state: app_state.clone()
                }
            }
        }
    }
}

/// 无障碍设置面板组件
#[component]
fn AccessibilitySettingsPanel(app_state: Signal<AccessibilityAppState>) -> Element {
    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 10px;",
            
            SettingToggle {
                label: "屏幕阅读器模式",
                description: "启用屏幕阅读器优化",
                checked: app_state.read().settings.screen_reader,
                on_change: move |checked| {
                    app_state.with_mut(|state| state.settings.screen_reader = checked);
                }
            }
            
            SettingToggle {
                label: "高对比度模式",
                description: "提高颜色对比度",
                checked: app_state.read().settings.high_contrast,
                on_change: move |checked| {
                    app_state.with_mut(|state| state.settings.high_contrast = checked);
                }
            }
            
            SettingToggle {
                label: "大字体模式",
                description: "增大字体大小",
                checked: app_state.read().settings.large_text,
                on_change: move |checked| {
                    app_state.with_mut(|state| state.settings.large_text = checked);
                }
            }
            
            SettingToggle {
                label: "减少动画",
                description: "禁用动画效果",
                checked: app_state.read().settings.reduced_motion,
                on_change: move |checked| {
                    app_state.with_mut(|state| state.settings.reduced_motion = checked);
                }
            }
            
            SettingToggle {
                label: "键盘导航",
                description: "启用键盘导航支持",
                checked: app_state.read().settings.keyboard_navigation,
                on_change: move |checked| {
                    app_state.with_mut(|state| state.settings.keyboard_navigation = checked);
                }
            }
            
            SettingToggle {
                label: "语音控制",
                description: "启用语音命令支持",
                checked: app_state.read().settings.voice_control,
                on_change: move |checked| {
                    app_state.with_mut(|state| state.settings.voice_control = checked);
                }
            }
        }
    }
}

/// 设置开关组件
#[component]
fn SettingToggle(label: &'static str, description: &'static str, checked: bool, on_change: EventHandler<bool>) -> Element {
    rsx! {
        div {
            style: "display: flex; align-items: center; gap: 10px;",
            
            input {
                r#type: "checkbox",
                id: "setting_{label}",
                checked: checked,
                onchange: move |event| on_change.call(event.checked()),
                "aria-describedby": "desc_{label}",
                style: "transform: scale(1.2);"
            }
            
            label {
                r#for: "setting_{label}",
                style: "flex: 1; font-size: 14px; color: #333; cursor: pointer;",
                
                div {
                    style: "font-weight: 500;",
                    "{label}"
                }
                
                div {
                    id: "desc_{label}",
                    style: "font-size: 12px; color: #666; margin-top: 2px;",
                    "{description}"
                }
            }
        }
    }
}

/// 标签页容器组件
#[component]
fn TabContainer(app_state: Signal<AccessibilityAppState>) -> Element {
    rsx! {
        div {
            class: "tab-container",
            style: "flex: 1; padding: 20px;",
            
            div {
                role: "tablist",
                "aria-label": "内容标签页",
                style: "display: flex; border-bottom: 1px solid #dee2e6; margin-bottom: 20px;",
                
                TabButton {
                    id: "tab_articles",
                    label: "文章",
                    selected: true,
                    on_click: move || {}
                }
                
                TabButton {
                    id: "tab_form",
                    label: "表单演示",
                    selected: false,
                    on_click: move || {}
                }
                
                TabButton {
                    id: "tab_interactive",
                    label: "交互元素",
                    selected: false,
                    on_click: move || {}
                }
            }
            
            // 标签页内容
            div {
                role: "tabpanel",
                "aria-labelledby": "tab_articles",
                
                ArticlesList { 
                    articles: app_state.read().demo_content.articles.clone()
                }
                
                FormDemo { 
                    form_data: app_state.read().demo_content.form_data.clone(),
                    on_update: move |new_data| {
                        app_state.with_mut(|state| state.demo_content.form_data = new_data);
                    }
                }
                
                InteractiveElementsDemo { 
                    elements: app_state.read().demo_content.interactive_elements.clone(),
                    on_update: move |updated_elements| {
                        app_state.with_mut(|state| state.demo_content.interactive_elements = updated_elements);
                    }
                }
            }
        }
    }
}

/// 标签按钮组件
#[component]
fn TabButton(id: &'static str, label: &'static str, selected: bool, on_click: EventHandler<()>) -> Element {
    rsx! {
        button {
            role: "tab",
            id: "{id}",
            "aria-selected": "{selected}",
            "aria-controls": "panel_{id}",
            onclick: move |_| on_click.call(()),
            style: "background: none; border: none; padding: 12px 20px; cursor: pointer; border-bottom: 2px solid {if selected { '#007bff' } else { 'transparent' }}; color: {if selected { '#007bff' } else { '#666' }}; font-weight: {if selected { 'bold' } else { 'normal' }};",
            
            "{label}"
        }
    }
}

/// 文章列表组件
#[component]
fn ArticlesList(articles: Vec<Article>) -> Element {
    rsx! {
        div {
            class: "articles-list",
            
            for article in articles.iter() {
                article {
                    key: "{article.id}",
                    style: "background: white; padding: 20px; margin-bottom: 20px; border-radius: 8px; box-shadow: 0 2px 4px rgba(0,0,0,0.1);",
                    
                    header {
                        h2 {
                            style: "margin: 0 0 10px 0; font-size: 1.5rem;",
                            "{article.title}"
                        }
                        
                        div {
                            style: "display: flex; gap: 15px; font-size: 14px; color: #666; margin-bottom: 15px;",
                            
                            span {
                                "作者: {article.author}"
                            }
                            
                            time {
                                datetime: "{article.publish_date}",
                                "{article.publish_date}"
                            }
                        }
                        
                        div {
                            style: "display: flex; gap: 8px; flex-wrap: wrap;",
                            
                            for tag in article.tags.iter() {
                                span {
                                    style: "background: #e3f2fd; color: #1976d2; padding: 4px 8px; border-radius: 12px; font-size: 12px;",
                                    "aria-label": "标签: {tag}",
                                    "#{tag}"
                                }
                            }
                        }
                    }
                    
                    div {
                        style: "margin-top: 15px; line-height: 1.6;",
                        "{article.content}"
                    }
                    
                    button {
                        style: "margin-top: 15px; background: #007bff; color: white; border: none; padding: 8px 16px; border-radius: 4px; cursor: pointer;",
                        "aria-label": "阅读完整文章: {article.title}",
                        "阅读更多"
                    }
                }
            }
        }
    }
}

/// 表单演示组件
#[component]
fn FormDemo(form_data: FormData, on_update: EventHandler<FormData>) -> Element {
    rsx! {
        div {
            class: "form-demo",
            style: "background: white; padding: 30px; border-radius: 8px; box-shadow: 0 2px 4px rgba(0,0,0,0.1);",
            
            h2 {
                style: "margin: 0 0 20px 0; font-size: 1.5rem;",
                "无障碍表单演示"
            }
            
            p {
                style: "margin: 0 0 20px 0; color: #666;",
                "这个表单展示了如何创建无障碍的用户输入界面。"
            }
            
            form {
                style: "display: flex; flex-direction: column; gap: 20px;",
                
                // 姓名输入
                div {
                    label {
                        r#for: "name",
                        style: "display: block; margin-bottom: 5px; font-weight: 500;",
                        "姓名 *"
                    }
                    
                    input {
                        r#type: "text",
                        id: "name",
                        value: "{form_data.name}",
                        oninput: move |event| {
                            let mut new_data = form_data.clone();
                            new_data.name = event.value();
                            on_update.call(new_data);
                        },
                        required: true,
                        "aria-describedby": "name_help",
                        style: "width: 100%; padding: 10px; border: 1px solid #ddd; border-radius: 4px; font-size: 16px;",
                        placeholder: "请输入您的姓名"
                    }
                    
                    div {
                        id: "name_help",
                        style: "font-size: 14px; color: #666; margin-top: 5px;",
                        "请输入您的真实姓名"
                    }
                }
                
                // 邮箱输入
                div {
                    label {
                        r#for: "email",
                        style: "display: block; margin-bottom: 5px; font-weight: 500;",
                        "邮箱 *"
                    }
                    
                    input {
                        r#type: "email",
                        id: "email",
                        value: "{form_data.email}",
                        oninput: move |event| {
                            let mut new_data = form_data.clone();
                            new_data.email = event.value();
                            on_update.call(new_data);
                        },
                        required: true,
                        "aria-describedby": "email_help",
                        style: "width: 100%; padding: 10px; border: 1px solid #ddd; border-radius: 4px; font-size: 16px;",
                        placeholder: "请输入您的邮箱地址"
                    }
                    
                    div {
                        id: "email_help",
                        style: "font-size: 14px; color: #666; margin-top: 5px;",
                        "我们将使用此邮箱与您联系"
                    }
                }
                
                // 消息输入
                div {
                    label {
                        r#for: "message",
                        style: "display: block; margin-bottom: 5px; font-weight: 500;",
                        "消息"
                    }
                    
                    textarea {
                        id: "message",
                        value: "{form_data.message}",
                        oninput: move |event| {
                            let mut new_data = form_data.clone();
                            new_data.message = event.value();
                            on_update.call(new_data);
                        },
                        "aria-describedby": "message_help",
                        style: "width: 100%; padding: 10px; border: 1px solid #ddd; border-radius: 4px; font-size: 16px; min-height: 100px; resize: vertical;",
                        placeholder: "请输入您的消息..."
                    }
                    
                    div {
                        id: "message_help",
                        style: "font-size: 14px; color: #666; margin-top: 5px;",
                        "最多500个字符"
                    }
                }
                
                // 复选框
                div {
                    style: "display: flex; align-items: center; gap: 10px;",
                    
                    input {
                        r#type: "checkbox",
                        id: "newsletter",
                        checked: form_data.newsletter,
                        onchange: move |event| {
                            let mut new_data = form_data.clone();
                            new_data.newsletter = event.checked();
                            on_update.call(new_data);
                        },
                        "aria-describedby": "newsletter_help",
                        style: "transform: scale(1.2);"
                    }
                    
                    label {
                        r#for: "newsletter",
                        style: "cursor: pointer;",
                        "订阅我们的新闻通讯"
                    }
                    
                    div {
                        id: "newsletter_help",
                        style: "font-size: 14px; color: #666; margin-top: 5px;",
                        "我们将定期发送最新的无障碍开发资讯"
                    }
                }
                
                // 无障碍反馈
                div {
                    label {
                        r#for: "accessibility_feedback",
                        style: "display: block; margin-bottom: 5px; font-weight: 500;",
                        "无障碍反馈"
                    }
                    
                    textarea {
                        id: "accessibility_feedback",
                        value: "{form_data.accessibility_feedback}",
                        oninput: move |event| {
                            let mut new_data = form_data.clone();
                            new_data.accessibility_feedback = event.value();
                            on_update.call(new_data);
                        },
                        "aria-describedby": "feedback_help",
                        style: "width: 100%; padding: 10px; border: 1px solid #ddd; border-radius: 4px; font-size: 16px; min-height: 80px; resize: vertical;",
                        placeholder: "请告诉我们您在使用此应用时的无障碍体验..."
                    }
                    
                    div {
                        id: "feedback_help",
                        style: "font-size: 14px; color: #666; margin-top: 5px;",
                        "您的反馈将帮助我们改进应用的无障碍功能"
                    }
                }
                
                // 提交按钮
                div {
                    style: "display: flex; gap: 10px; justify-content: flex-end;",
                    
                    button {
                        r#type: "button",
                        style: "background: #6c757d; color: white; border: none; padding: 10px 20px; border-radius: 4px; cursor: pointer;",
                        onclick: move |_| {
                            // 重置表单
                        },
                        "重置"
                    }
                    
                    button {
                        r#type: "submit",
                        style: "background: #007bff; color: white; border: none; padding: 10px 20px; border-radius: 4px; cursor: pointer;",
                        "提交表单"
                    }
                }
            }
        }
    }
}

/// 交互元素演示组件
#[component]
fn InteractiveElementsDemo(elements: Vec<InteractiveElement>, on_update: EventHandler<Vec<InteractiveElement>>) -> Element {
    rsx! {
        div {
            class: "interactive-demo",
            style: "background: white; padding: 30px; border-radius: 8px; box-shadow: 0 2px 4px rgba(0,0,0,0.1);",
            
            h2 {
                style: "margin: 0 0 20px 0; font-size: 1.5rem;",
                "交互元素演示"
            }
            
            p {
                style: "margin: 0 0 20px 0; color: #666;",
                "这些交互元素展示了如何正确实现无障碍的用户界面组件。"
            }
            
            div {
                style: "display: flex; flex-direction: column; gap: 20px;",
                
                for element in elements.iter() {
                    InteractiveElementDemo { 
                        element: element.clone(),
                        on_update: move |updated_element| {
                            let mut updated_elements = elements.clone();
                            if let Some(elem) = updated_elements.iter_mut().find(|e| e.id == updated_element.id) {
                                *elem = updated_element;
                            }
                            on_update.call(updated_elements);
                        }
                    }
                }
            }
        }
    }
}

/// 交互元素演示组件
#[component]
fn InteractiveElementDemo(element: InteractiveElement, on_update: EventHandler<InteractiveElement>) -> Element {
    rsx! {
        div {
            style: "border: 1px solid #dee2e6; border-radius: 8px; padding: 20px;",
            
            h3 {
                style: "margin: 0 0 10px 0; font-size: 1.2rem;",
                "{element.label}"
            }
            
            p {
                style: "margin: 0 0 15px 0; color: #666;",
                "{element.description}"
            }
            
            match element.type_ {
                ElementType::Button => {
                    button {
                        style: "background: #007bff; color: white; border: none; padding: 10px 20px; border-radius: 4px; cursor: pointer;",
                        "aria-describedby": "{element.id}_desc",
                        onclick: move |_| {
                            let mut updated = element.clone();
                            updated.state = match updated.state {
                                ElementState::Normal => ElementState::Selected,
                                _ => ElementState::Normal,
                            };
                            on_update.call(updated);
                        },
                        
                        "{element.label}"
                        
                        span {
                            id: "{element.id}_desc",
                            class: "sr-only",
                            " - 点击以切换状态"
                        }
                    }
                }
                
                ElementType::Link => {
                    a {
                        href: "#",
                        style: "color: #007bff; text-decoration: underline;",
                        "aria-describedby": "{element.id}_desc",
                        
                        "{element.label}"
                        
                        span {
                            id: "{element.id}_desc",
                            class: "sr-only",
                            " - 外部链接，将在新窗口中打开"
                        }
                    }
                }
                
                _ => {
                    div {
                        style: "padding: 10px; background: #f8f9fa; border-radius: 4px; color: #666;",
                        "其他类型的交互元素: {element.type_:?}"
                    }
                }
            }
            
            div {
                style: "margin-top: 10px; font-size: 14px; color: #666;",
                "状态: {element.state:?}"
            }
        }
    }
}

/// 无障碍工具栏组件
#[component]
fn AccessibilityToolbar(app_state: Signal<AccessibilityAppState>) -> Element {
    rsx! {
        div {
            class: "accessibility-toolbar",
            style: "position: fixed; bottom: 20px; right: 20px; background: rgba(0,0,0,0.8); color: white; padding: 10px; border-radius: 8px; display: flex; gap: 10px; z-index: 1000;",
            role: "toolbar",
            "aria-label": "无障碍工具栏",
            
            button {
                style: "background: none; border: 1px solid white; color: white; padding: 8px; border-radius: 4px; cursor: pointer;",
                "aria-label": "切换高对比度模式",
                onclick: move |_| {
                    app_state.with_mut(|state| {
                        state.settings.high_contrast = !state.settings.high_contrast;
                    });
                },
                "🎨"
            }
            
            button {
                style: "background: none; border: 1px solid white; color: white; padding: 8px; border-radius: 4px; cursor: pointer;",
                "aria-label": "切换大字体模式",
                onclick: move |_| {
                    app_state.with_mut(|state| {
                        state.settings.large_text = !state.settings.large_text;
                    });
                },
                "🔍"
            }
            
            button {
                style: "background: none; border: 1px solid white; color: white; padding: 8px; border-radius: 4px; cursor: pointer;",
                "aria-label": "切换减少动画模式",
                onclick: move |_| {
                    app_state.with_mut(|state| {
                        state.settings.reduced_motion = !state.settings.reduced_motion;
                    });
                },
                "⏸️"
            }
            
            button {
                style: "background: none; border: 1px solid white; color: white; padding: 8px; border-radius: 4px; cursor: pointer;",
                "aria-label": "显示焦点指示器",
                onclick: move |_| {
                    app_state.with_mut(|state| {
                        state.settings.focus_indicators = !state.settings.focus_indicators;
                    });
                },
                "🎯"
            }
        }
    }
}

/// 主函数
fn main() {
    // 启动无障碍演示应用
    dioxus_web::launch(AccessibilityApp);
}
