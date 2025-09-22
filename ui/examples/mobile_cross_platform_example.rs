//! 移动端跨平台UI示例 - Rust 1.90优化
//! 
//! 本示例展示了如何使用Rust构建跨移动平台的UI应用：
//! - iOS和Android原生支持
//! - 响应式设计
//! - 触摸交互优化
//! - 移动端性能优化
//! - Rust 1.90新特性集成

use dioxus::prelude::*;
use std::collections::HashMap;
use std::time::{Duration, Instant};

/// 移动应用状态
#[derive(Debug, Clone)]
struct MobileAppState {
    current_screen: Screen,
    user_profile: UserProfile,
    notifications: Vec<Notification>,
    settings: MobileSettings,
    performance_metrics: MobilePerformanceMetrics,
}

/// 用户配置文件
#[derive(Debug, Clone)]
struct UserProfile {
    name: String,
    avatar: String,
    email: String,
    preferences: UserPreferences,
    stats: UserStats,
}

/// 用户偏好设置
#[derive(Debug, Clone)]
struct UserPreferences {
    theme: MobileTheme,
    language: String,
    notifications_enabled: bool,
    biometric_auth: bool,
    haptic_feedback: bool,
}

/// 用户统计
#[derive(Debug, Clone)]
struct UserStats {
    total_sessions: u32,
    last_active: Instant,
    favorite_features: Vec<String>,
    achievements: Vec<Achievement>,
}

/// 成就系统
#[derive(Debug, Clone)]
struct Achievement {
    id: String,
    name: String,
    description: String,
    icon: String,
    unlocked_at: Instant,
    progress: u8,
}

/// 通知系统
#[derive(Debug, Clone)]
struct Notification {
    id: String,
    title: String,
    message: String,
    timestamp: Instant,
    read: bool,
    priority: NotificationPriority,
    action_type: NotificationAction,
}

/// 通知优先级
#[derive(Debug, Clone, PartialEq)]
enum NotificationPriority {
    Low,
    Medium,
    High,
    Critical,
}

/// 通知操作类型
#[derive(Debug, Clone)]
enum NotificationAction {
    Navigate(String),
    OpenUrl(String),
    Callback(String),
}

/// 移动端设置
#[derive(Debug, Clone)]
struct MobileSettings {
    screen_orientation: ScreenOrientation,
    battery_saver: bool,
    data_saver: bool,
    accessibility: AccessibilitySettings,
    privacy: PrivacySettings,
}

/// 屏幕方向
#[derive(Debug, Clone, PartialEq)]
enum ScreenOrientation {
    Portrait,
    Landscape,
    Auto,
}

/// 无障碍设置
#[derive(Debug, Clone)]
struct AccessibilitySettings {
    high_contrast: bool,
    large_text: bool,
    screen_reader: bool,
    voice_over: bool,
}

/// 隐私设置
#[derive(Debug, Clone)]
struct PrivacySettings {
    analytics_enabled: bool,
    crash_reporting: bool,
    personalization: bool,
}

/// 移动端主题
#[derive(Debug, Clone, PartialEq)]
enum MobileTheme {
    Light,
    Dark,
    Auto,
    HighContrast,
}

/// 屏幕枚举
#[derive(Debug, Clone, PartialEq)]
enum Screen {
    Home,
    Profile,
    Settings,
    Notifications,
    Achievements,
    Performance,
}

/// 移动端性能指标
#[derive(Debug, Clone)]
struct MobilePerformanceMetrics {
    fps: f32,
    memory_usage: u64,
    battery_usage: f32,
    network_usage: u64,
    cpu_usage: f32,
    render_time: Duration,
    touch_latency: Duration,
}

impl Default for MobileAppState {
    fn default() -> Self {
        Self {
            current_screen: Screen::Home,
            user_profile: UserProfile {
                name: "Rust开发者".to_string(),
                avatar: "👨‍💻".to_string(),
                email: "developer@rust-lang.org".to_string(),
                preferences: UserPreferences {
                    theme: MobileTheme::Auto,
                    language: "zh-CN".to_string(),
                    notifications_enabled: true,
                    biometric_auth: true,
                    haptic_feedback: true,
                },
                stats: UserStats {
                    total_sessions: 42,
                    last_active: Instant::now(),
                    favorite_features: vec!["跨平台开发".to_string(), "性能优化".to_string()],
                    achievements: vec![
                        Achievement {
                            id: "first_app".to_string(),
                            name: "首次应用".to_string(),
                            description: "创建了第一个Rust应用".to_string(),
                            icon: "🎉".to_string(),
                            unlocked_at: Instant::now(),
                            progress: 100,
                        },
                    ],
                },
            },
            notifications: vec![
                Notification {
                    id: "1".to_string(),
                    title: "欢迎使用Rust移动应用".to_string(),
                    message: "开始探索跨平台开发的强大功能".to_string(),
                    timestamp: Instant::now(),
                    read: false,
                    priority: NotificationPriority::High,
                    action_type: NotificationAction::Navigate("home".to_string()),
                },
            ],
            settings: MobileSettings {
                screen_orientation: ScreenOrientation::Auto,
                battery_saver: false,
                data_saver: false,
                accessibility: AccessibilitySettings {
                    high_contrast: false,
                    large_text: false,
                    screen_reader: false,
                    voice_over: false,
                },
                privacy: PrivacySettings {
                    analytics_enabled: true,
                    crash_reporting: true,
                    personalization: true,
                },
            },
            performance_metrics: MobilePerformanceMetrics {
                fps: 60.0,
                memory_usage: 128,
                battery_usage: 15.5,
                network_usage: 1024,
                cpu_usage: 25.0,
                render_time: Duration::from_millis(16),
                touch_latency: Duration::from_millis(8),
            },
        }
    }
}

/// 主移动应用组件
fn MobileApp() -> Element {
    let mut app_state = use_signal(|| MobileAppState::default());
    
    // 移动端性能监控
    use_effect(move || {
        // 模拟性能数据更新
        app_state.with_mut(|state| {
            state.performance_metrics.fps = 60.0 + (state.performance_metrics.fps * 0.1).sin() * 5.0;
            state.performance_metrics.memory_usage = 128 + (state.performance_metrics.memory_usage % 50);
        });
    });

    rsx! {
        div {
            class: "mobile-app",
            style: "min-height: 100vh; background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;",
            
            // 状态栏
            StatusBar { 
                app_state: app_state.clone()
            }
            
            // 主内容区域
            div {
                class: "main-content",
                style: "padding: 20px;",
                
                match app_state.read().current_screen {
                    Screen::Home => HomeScreen { app_state: app_state.clone() },
                    Screen::Profile => ProfileScreen { app_state: app_state.clone() },
                    Screen::Settings => SettingsScreen { app_state: app_state.clone() },
                    Screen::Notifications => NotificationsScreen { app_state: app_state.clone() },
                    Screen::Achievements => AchievementsScreen { app_state: app_state.clone() },
                    Screen::Performance => PerformanceScreen { app_state: app_state.clone() },
                }
            }
            
            // 底部导航栏
            BottomNavigation { 
                app_state: app_state.clone(),
                on_screen_change: move |screen| {
                    app_state.with_mut(|state| state.current_screen = screen);
                }
            }
        }
    }
}

/// 状态栏组件
#[component]
fn StatusBar(app_state: Signal<MobileAppState>) -> Element {
    rsx! {
        div {
            class: "status-bar",
            style: "display: flex; justify-content: space-between; align-items: center; padding: 10px 20px; background: rgba(0,0,0,0.1); color: white; font-size: 14px;",
            
            div {
                span { "🦀 Rust Mobile" }
            }
            
            div {
                style: "display: flex; gap: 15px; align-items: center;",
                span { "📶" }
                span { "🔋 {app_state.read().performance_metrics.battery_usage:.1}%" }
                span { "{chrono::Local::now().format('%H:%M')}" }
            }
        }
    }
}

/// 主屏幕组件
#[component]
fn HomeScreen(app_state: Signal<MobileAppState>) -> Element {
    rsx! {
        div {
            class: "home-screen",
            
            // 欢迎卡片
            div {
                class: "welcome-card",
                style: "background: rgba(255,255,255,0.95); padding: 25px; border-radius: 20px; margin-bottom: 20px; box-shadow: 0 8px 32px rgba(0,0,0,0.1);",
                
                div {
                    style: "display: flex; align-items: center; gap: 15px; margin-bottom: 15px;",
                    
                    div {
                        style: "font-size: 48px;",
                        "{app_state.read().user_profile.avatar}"
                    }
                    
                    div {
                        h2 {
                            style: "margin: 0; color: #333; font-size: 24px;",
                            "你好, {app_state.read().user_profile.name}!"
                        }
                        p {
                            style: "margin: 5px 0 0 0; color: #666;",
                            "欢迎使用Rust跨平台移动应用"
                        }
                    }
                }
                
                // 快速操作按钮
                div {
                    style: "display: grid; grid-template-columns: repeat(2, 1fr); gap: 15px;",
                    
                    QuickActionButton {
                        icon: "⚙️",
                        title: "设置",
                        on_click: move || {
                            app_state.with_mut(|state| state.current_screen = Screen::Settings);
                        }
                    }
                    
                    QuickActionButton {
                        icon: "🔔",
                        title: "通知",
                        on_click: move || {
                            app_state.with_mut(|state| state.current_screen = Screen::Notifications);
                        }
                    }
                    
                    QuickActionButton {
                        icon: "🏆",
                        title: "成就",
                        on_click: move || {
                            app_state.with_mut(|state| state.current_screen = Screen::Achievements);
                        }
                    }
                    
                    QuickActionButton {
                        icon: "📊",
                        title: "性能",
                        on_click: move || {
                            app_state.with_mut(|state| state.current_screen = Screen::Performance);
                        }
                    }
                }
            }
            
            // 功能卡片
            div {
                style: "display: flex; flex-direction: column; gap: 15px;",
                
                FeatureCard {
                    title: "跨平台开发",
                    description: "使用Rust构建原生性能的移动应用",
                    icon: "🚀",
                    progress: 85
                }
                
                FeatureCard {
                    title: "性能优化",
                    description: "利用Rust 1.90的性能特性",
                    icon: "⚡",
                    progress: 92
                }
                
                FeatureCard {
                    title: "安全可靠",
                    description: "内存安全和类型安全保证",
                    icon: "🛡️",
                    progress: 100
                }
            }
        }
    }
}

/// 快速操作按钮组件
#[component]
fn QuickActionButton(icon: &'static str, title: &'static str, on_click: EventHandler<()>) -> Element {
    rsx! {
        button {
            style: "background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); color: white; border: none; padding: 20px; border-radius: 15px; cursor: pointer; transition: transform 0.2s; box-shadow: 0 4px 15px rgba(0,0,0,0.2);",
            onclick: move |_| on_click.call(()),
            onmouseenter: move |_| {
                // 触摸反馈模拟
            },
            
            div {
                style: "font-size: 32px; margin-bottom: 8px;",
                "{icon}"
            }
            
            div {
                style: "font-size: 16px; font-weight: bold;",
                "{title}"
            }
        }
    }
}

/// 功能卡片组件
#[component]
fn FeatureCard(title: &'static str, description: &'static str, icon: &'static str, progress: u8) -> Element {
    rsx! {
        div {
            style: "background: rgba(255,255,255,0.95); padding: 20px; border-radius: 15px; box-shadow: 0 4px 20px rgba(0,0,0,0.1);",
            
            div {
                style: "display: flex; align-items: center; gap: 15px; margin-bottom: 15px;",
                
                div {
                    style: "font-size: 32px;",
                    "{icon}"
                }
                
                div {
                    style: "flex: 1;",
                    h3 {
                        style: "margin: 0 0 5px 0; color: #333; font-size: 18px;",
                        "{title}"
                    }
                    p {
                        style: "margin: 0; color: #666; font-size: 14px; line-height: 1.4;",
                        "{description}"
                    }
                }
            }
            
            // 进度条
            div {
                style: "background: #e9ecef; height: 6px; border-radius: 3px; overflow: hidden;",
                div {
                    style: "background: linear-gradient(90deg, #667eea 0%, #764ba2 100%); height: 100%; width: {progress}%; transition: width 0.3s ease;"
                }
            }
            
            div {
                style: "text-align: right; margin-top: 5px; font-size: 12px; color: #666;",
                "{progress}%"
            }
        }
    }
}

/// 个人资料屏幕
#[component]
fn ProfileScreen(app_state: Signal<MobileAppState>) -> Element {
    rsx! {
        div {
            class: "profile-screen",
            
            // 个人信息卡片
            div {
                style: "background: rgba(255,255,255,0.95); padding: 25px; border-radius: 20px; margin-bottom: 20px; text-align: center; box-shadow: 0 8px 32px rgba(0,0,0,0.1);",
                
                div {
                    style: "font-size: 80px; margin-bottom: 15px;",
                    "{app_state.read().user_profile.avatar}"
                }
                
                h2 {
                    style: "margin: 0 0 5px 0; color: #333; font-size: 28px;",
                    "{app_state.read().user_profile.name}"
                }
                
                p {
                    style: "margin: 0 0 20px 0; color: #666;",
                    "{app_state.read().user_profile.email}"
                }
                
                // 用户统计
                div {
                    style: "display: grid; grid-template-columns: repeat(3, 1fr); gap: 20px; margin-top: 20px;",
                    
                    StatItem {
                        label: "会话数",
                        value: "{app_state.read().user_profile.stats.total_sessions}"
                    }
                    
                    StatItem {
                        label: "成就",
                        value: "{app_state.read().user_profile.stats.achievements.len()}"
                    }
                    
                    StatItem {
                        label: "活跃度",
                        value: "高"
                    }
                }
            }
            
            // 偏好设置
            div {
                style: "background: rgba(255,255,255,0.95); padding: 20px; border-radius: 15px; box-shadow: 0 4px 20px rgba(0,0,0,0.1);",
                
                h3 {
                    style: "margin: 0 0 15px 0; color: #333;",
                    "偏好设置"
                }
                
                PreferenceItem {
                    label: "主题",
                    value: "{app_state.read().user_profile.preferences.theme:?}",
                    icon: "🎨"
                }
                
                PreferenceItem {
                    label: "语言",
                    value: "{app_state.read().user_profile.preferences.language}",
                    icon: "🌐"
                }
                
                PreferenceItem {
                    label: "通知",
                    value: if app_state.read().user_profile.preferences.notifications_enabled { "开启" } else { "关闭" },
                    icon: "🔔"
                }
                
                PreferenceItem {
                    label: "生物识别",
                    value: if app_state.read().user_profile.preferences.biometric_auth { "开启" } else { "关闭" },
                    icon: "🔐"
                }
            }
        }
    }
}

/// 统计项组件
#[component]
fn StatItem(label: &'static str, value: &'static str) -> Element {
    rsx! {
        div {
            style: "text-align: center;",
            div {
                style: "font-size: 24px; font-weight: bold; color: #667eea; margin-bottom: 5px;",
                "{value}"
            }
            div {
                style: "font-size: 12px; color: #666;",
                "{label}"
            }
        }
    }
}

/// 偏好设置项组件
#[component]
fn PreferenceItem(label: &'static str, value: &'static str, icon: &'static str) -> Element {
    rsx! {
        div {
            style: "display: flex; align-items: center; gap: 15px; padding: 12px 0; border-bottom: 1px solid #f0f0f0;",
            
            div {
                style: "font-size: 20px;",
                "{icon}"
            }
            
            div {
                style: "flex: 1;",
                div {
                    style: "font-weight: 500; color: #333; margin-bottom: 2px;",
                    "{label}"
                }
                div {
                    style: "font-size: 14px; color: #666;",
                    "{value}"
                }
            }
        }
    }
}

/// 通知屏幕
#[component]
fn NotificationsScreen(app_state: Signal<MobileAppState>) -> Element {
    rsx! {
        div {
            class: "notifications-screen",
            
            h2 {
                style: "color: white; margin: 0 0 20px 0; font-size: 24px;",
                "通知中心"
            }
            
            div {
                style: "display: flex; flex-direction: column; gap: 15px;",
                
                for notification in app_state.read().notifications.iter() {
                    NotificationCard { 
                        notification: notification.clone(),
                        on_read: move |id| {
                            app_state.with_mut(|state| {
                                if let Some(notif) = state.notifications.iter_mut().find(|n| n.id == id) {
                                    notif.read = true;
                                }
                            });
                        }
                    }
                }
            }
        }
    }
}

/// 通知卡片组件
#[component]
fn NotificationCard(notification: Notification, on_read: EventHandler<String>) -> Element {
    rsx! {
        div {
            class: "notification-card",
            style: "background: rgba(255,255,255,0.95); padding: 20px; border-radius: 15px; box-shadow: 0 4px 20px rgba(0,0,0,0.1); {if !notification.read { 'border-left: 4px solid #007bff;' } else { '' }}",
            
            div {
                style: "display: flex; align-items: flex-start; gap: 15px;",
                
                div {
                    style: "font-size: 24px; margin-top: 5px;",
                    match notification.priority {
                        NotificationPriority::Low => "📢",
                        NotificationPriority::Medium => "🔔",
                        NotificationPriority::High => "⚠️",
                        NotificationPriority::Critical => "🚨",
                    }
                }
                
                div {
                    style: "flex: 1;",
                    
                    div {
                        style: "display: flex; justify-content: space-between; align-items: flex-start; margin-bottom: 8px;",
                        
                        h4 {
                            style: "margin: 0; color: #333; font-size: 16px; {if notification.read { 'opacity: 0.6;' } else { '' }}",
                            "{notification.title}"
                        }
                        
                        span {
                            style: "font-size: 12px; color: #999;",
                            "刚刚"
                        }
                    }
                    
                    p {
                        style: "margin: 0 0 10px 0; color: #666; line-height: 1.4; {if notification.read { 'opacity: 0.6;' } else { '' }}",
                        "{notification.message}"
                    }
                    
                    if !notification.read {
                        button {
                            style: "background: #007bff; color: white; border: none; padding: 8px 16px; border-radius: 6px; font-size: 12px; cursor: pointer;",
                            onclick: move |_| on_read.call(notification.id.clone()),
                            "标记为已读"
                        }
                    }
                }
            }
        }
    }
}

/// 成就屏幕
#[component]
fn AchievementsScreen(app_state: Signal<MobileAppState>) -> Element {
    rsx! {
        div {
            class: "achievements-screen",
            
            h2 {
                style: "color: white; margin: 0 0 20px 0; font-size: 24px;",
                "成就中心"
            }
            
            div {
                style: "display: grid; grid-template-columns: repeat(auto-fit, minmax(150px, 1fr)); gap: 15px;",
                
                for achievement in app_state.read().user_profile.stats.achievements.iter() {
                    AchievementCard { achievement: achievement.clone() }
                }
            }
        }
    }
}

/// 成就卡片组件
#[component]
fn AchievementCard(achievement: Achievement) -> Element {
    rsx! {
        div {
            style: "background: rgba(255,255,255,0.95); padding: 20px; border-radius: 15px; text-align: center; box-shadow: 0 4px 20px rgba(0,0,0,0.1);",
            
            div {
                style: "font-size: 48px; margin-bottom: 10px;",
                "{achievement.icon}"
            }
            
            h4 {
                style: "margin: 0 0 8px 0; color: #333; font-size: 16px;",
                "{achievement.name}"
            }
            
            p {
                style: "margin: 0 0 15px 0; color: #666; font-size: 12px; line-height: 1.4;",
                "{achievement.description}"
            }
            
            // 进度条
            div {
                style: "background: #e9ecef; height: 4px; border-radius: 2px; overflow: hidden;",
                div {
                    style: "background: linear-gradient(90deg, #28a745 0%, #20c997 100%); height: 100%; width: {achievement.progress}%;"
                }
            }
            
            div {
                style: "text-align: right; margin-top: 5px; font-size: 10px; color: #666;",
                "{achievement.progress}%"
            }
        }
    }
}

/// 性能屏幕
#[component]
fn PerformanceScreen(app_state: Signal<MobileAppState>) -> Element {
    rsx! {
        div {
            class: "performance-screen",
            
            h2 {
                style: "color: white; margin: 0 0 20px 0; font-size: 24px;",
                "性能监控"
            }
            
            div {
                style: "display: grid; grid-template-columns: repeat(auto-fit, minmax(150px, 1fr)); gap: 15px;",
                
                PerformanceMetric {
                    label: "FPS",
                    value: "{app_state.read().performance_metrics.fps:.1}",
                    unit: "fps",
                    color: "#28a745"
                }
                
                PerformanceMetric {
                    label: "内存",
                    value: "{app_state.read().performance_metrics.memory_usage}",
                    unit: "MB",
                    color: "#007bff"
                }
                
                PerformanceMetric {
                    label: "电池",
                    value: "{app_state.read().performance_metrics.battery_usage:.1}",
                    unit: "%",
                    color: "#ffc107"
                }
                
                PerformanceMetric {
                    label: "CPU",
                    value: "{app_state.read().performance_metrics.cpu_usage:.1}",
                    unit: "%",
                    color: "#dc3545"
                }
                
                PerformanceMetric {
                    label: "渲染",
                    value: "{app_state.read().performance_metrics.render_time.as_millis()}",
                    unit: "ms",
                    color: "#6f42c1"
                }
                
                PerformanceMetric {
                    label: "触摸延迟",
                    value: "{app_state.read().performance_metrics.touch_latency.as_millis()}",
                    unit: "ms",
                    color: "#fd7e14"
                }
            }
        }
    }
}

/// 性能指标组件
#[component]
fn PerformanceMetric(label: &'static str, value: &'static str, unit: &'static str, color: &'static str) -> Element {
    rsx! {
        div {
            style: "background: rgba(255,255,255,0.95); padding: 20px; border-radius: 15px; text-align: center; box-shadow: 0 4px 20px rgba(0,0,0,0.1);",
            
            div {
                style: "font-size: 32px; font-weight: bold; color: {color}; margin-bottom: 8px;",
                "{value}"
            }
            
            div {
                style: "font-size: 14px; color: #666; margin-bottom: 4px;",
                "{unit}"
            }
            
            div {
                style: "font-size: 12px; color: #999;",
                "{label}"
            }
        }
    }
}

/// 设置屏幕
#[component]
fn SettingsScreen(app_state: Signal<MobileAppState>) -> Element {
    rsx! {
        div {
            class: "settings-screen",
            
            h2 {
                style: "color: white; margin: 0 0 20px 0; font-size: 24px;",
                "设置"
            }
            
            div {
                style: "background: rgba(255,255,255,0.95); padding: 20px; border-radius: 15px; box-shadow: 0 4px 20px rgba(0,0,0,0.1);",
                
                SettingItem {
                    label: "屏幕方向",
                    value: "{app_state.read().settings.screen_orientation:?}",
                    icon: "📱"
                }
                
                SettingItem {
                    label: "电池优化",
                    value: if app_state.read().settings.battery_saver { "开启" } else { "关闭" },
                    icon: "🔋"
                }
                
                SettingItem {
                    label: "数据节省",
                    value: if app_state.read().settings.data_saver { "开启" } else { "关闭" },
                    icon: "📊"
                }
                
                SettingItem {
                    label: "高对比度",
                    value: if app_state.read().settings.accessibility.high_contrast { "开启" } else { "关闭" },
                    icon: "👁️"
                }
                
                SettingItem {
                    label: "大字体",
                    value: if app_state.read().settings.accessibility.large_text { "开启" } else { "关闭" },
                    icon: "🔍"
                }
                
                SettingItem {
                    label: "屏幕阅读器",
                    value: if app_state.read().settings.accessibility.screen_reader { "开启" } else { "关闭" },
                    icon: "📢"
                }
            }
        }
    }
}

/// 设置项组件
#[component]
fn SettingItem(label: &'static str, value: &'static str, icon: &'static str) -> Element {
    rsx! {
        div {
            style: "display: flex; align-items: center; gap: 15px; padding: 15px 0; border-bottom: 1px solid #f0f0f0;",
            
            div {
                style: "font-size: 20px;",
                "{icon}"
            }
            
            div {
                style: "flex: 1;",
                div {
                    style: "font-weight: 500; color: #333; margin-bottom: 2px;",
                    "{label}"
                }
                div {
                    style: "font-size: 14px; color: #666;",
                    "{value}"
                }
            }
            
            div {
                style: "color: #007bff;",
                "›"
            }
        }
    }
}

/// 底部导航栏组件
#[component]
fn BottomNavigation(app_state: Signal<MobileAppState>, on_screen_change: EventHandler<Screen>) -> Element {
    rsx! {
        nav {
            class: "bottom-navigation",
            style: "position: fixed; bottom: 0; left: 0; right: 0; background: rgba(255,255,255,0.95); backdrop-filter: blur(10px); padding: 10px 0; display: flex; justify-content: space-around; box-shadow: 0 -2px 20px rgba(0,0,0,0.1);",
            
            NavItem {
                icon: "🏠",
                label: "首页",
                screen: Screen::Home,
                current_screen: app_state.read().current_screen.clone(),
                on_click: move || on_screen_change.call(Screen::Home)
            }
            
            NavItem {
                icon: "👤",
                label: "个人",
                screen: Screen::Profile,
                current_screen: app_state.read().current_screen.clone(),
                on_click: move || on_screen_change.call(Screen::Profile)
            }
            
            NavItem {
                icon: "🔔",
                label: "通知",
                screen: Screen::Notifications,
                current_screen: app_state.read().current_screen.clone(),
                on_click: move || on_screen_change.call(Screen::Notifications)
            }
            
            NavItem {
                icon: "🏆",
                label: "成就",
                screen: Screen::Achievements,
                current_screen: app_state.read().current_screen.clone(),
                on_click: move || on_screen_change.call(Screen::Achievements)
            }
            
            NavItem {
                icon: "⚙️",
                label: "设置",
                screen: Screen::Settings,
                current_screen: app_state.read().current_screen.clone(),
                on_click: move || on_screen_change.call(Screen::Settings)
            }
        }
    }
}

/// 导航项组件
#[component]
fn NavItem(icon: &'static str, label: &'static str, screen: Screen, current_screen: Screen, on_click: EventHandler<()>) -> Element {
    let is_active = screen == current_screen;
    
    rsx! {
        button {
            style: "background: none; border: none; display: flex; flex-direction: column; align-items: center; gap: 4px; padding: 8px; cursor: pointer; color: {if is_active { '#007bff' } else { '#666' }}; transition: color 0.2s;",
            onclick: move |_| on_click.call(()),
            
            div {
                style: "font-size: 20px;",
                "{icon}"
            }
            
            div {
                style: "font-size: 10px; font-weight: {if is_active { 'bold' } else { 'normal' }};",
                "{label}"
            }
        }
    }
}

/// 主函数
fn main() {
    // 启动移动端Dioxus应用
    dioxus_web::launch(MobileApp);
}
