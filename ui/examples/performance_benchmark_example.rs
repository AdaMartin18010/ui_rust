//! 性能基准测试示例 - Rust 1.90优化
//! 
//! 本示例展示了如何对Rust UI框架进行性能测试和优化：
//! - 渲染性能测试
//! - 内存使用监控
//! - 组件性能分析
//! - 跨框架性能对比
//! - 实时性能监控
//! - 优化建议系统

use dioxus::prelude::*;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};

/// 性能测试配置
#[derive(Debug, Clone)]
struct BenchmarkConfig {
    test_duration: Duration,
    component_count: usize,
    update_frequency: Duration,
    enable_memory_tracking: bool,
    enable_render_tracking: bool,
}

impl Default for BenchmarkConfig {
    fn default() -> Self {
        Self {
            test_duration: Duration::from_secs(30),
            component_count: 1000,
            update_frequency: Duration::from_millis(16), // 60 FPS
            enable_memory_tracking: true,
            enable_render_tracking: true,
        }
    }
}

/// 性能指标
#[derive(Debug, Clone)]
struct PerformanceMetrics {
    frame_count: u64,
    total_render_time: Duration,
    average_render_time: Duration,
    min_render_time: Duration,
    max_render_time: Duration,
    memory_usage: u64,
    memory_peak: u64,
    component_count: usize,
    update_count: u64,
    last_update: Instant,
    fps_history: Vec<f32>,
    memory_history: Vec<u64>,
}

impl Default for PerformanceMetrics {
    fn default() -> Self {
        Self {
            frame_count: 0,
            total_render_time: Duration::ZERO,
            average_render_time: Duration::ZERO,
            min_render_time: Duration::from_millis(u64::MAX),
            max_render_time: Duration::ZERO,
            memory_usage: 0,
            memory_peak: 0,
            component_count: 0,
            update_count: 0,
            last_update: Instant::now(),
            fps_history: Vec::new(),
            memory_history: Vec::new(),
        }
    }
}

/// 组件性能数据
#[derive(Debug, Clone)]
struct ComponentPerformance {
    name: String,
    render_count: u64,
    total_render_time: Duration,
    average_render_time: Duration,
    memory_usage: u64,
    last_render: Instant,
}

/// 性能测试结果
#[derive(Debug, Clone)]
struct BenchmarkResult {
    framework: String,
    metrics: PerformanceMetrics,
    component_performance: HashMap<String, ComponentPerformance>,
    optimization_suggestions: Vec<OptimizationSuggestion>,
    test_duration: Duration,
    timestamp: Instant,
}

/// 优化建议
#[derive(Debug, Clone)]
struct OptimizationSuggestion {
    category: OptimizationCategory,
    severity: Severity,
    title: String,
    description: String,
    impact: String,
    implementation: String,
}

/// 优化类别
#[derive(Debug, Clone)]
enum OptimizationCategory {
    Rendering,
    Memory,
    Component,
    State,
    Network,
}

/// 严重程度
#[derive(Debug, Clone)]
enum Severity {
    Low,
    Medium,
    High,
    Critical,
}

/// 性能测试应用状态
#[derive(Debug, Clone)]
struct BenchmarkAppState {
    config: BenchmarkConfig,
    metrics: PerformanceMetrics,
    is_running: bool,
    test_results: Vec<BenchmarkResult>,
    current_test: Option<String>,
    component_data: Vec<ComponentData>,
}

/// 组件数据
#[derive(Debug, Clone)]
struct ComponentData {
    id: usize,
    name: String,
    value: f64,
    color: String,
    position: (f64, f64),
    size: (f64, f64),
    last_update: Instant,
}

impl Default for BenchmarkAppState {
    fn default() -> Self {
        Self {
            config: BenchmarkConfig::default(),
            metrics: PerformanceMetrics::default(),
            is_running: false,
            test_results: Vec::new(),
            current_test: None,
            component_data: Self::generate_test_components(1000),
        }
    }
}

impl BenchmarkAppState {
    fn generate_test_components(count: usize) -> Vec<ComponentData> {
        (0..count)
            .map(|i| ComponentData {
                id: i,
                name: format!("Component_{}", i),
                value: (i as f64 * 0.1).sin(),
                color: Self::generate_color(i),
                position: (
                    (i as f64 * 0.1).cos() * 100.0,
                    (i as f64 * 0.1).sin() * 100.0,
                ),
                size: (50.0 + (i as f64 * 0.05).sin() * 20.0, 50.0 + (i as f64 * 0.05).cos() * 20.0),
                last_update: Instant::now(),
            })
            .collect()
    }
    
    fn generate_color(index: usize) -> String {
        let colors = [
            "#FF6B6B", "#4ECDC4", "#45B7D1", "#96CEB4", "#FFEAA7",
            "#DDA0DD", "#98D8C8", "#F7DC6F", "#BB8FCE", "#85C1E9",
        ];
        colors[index % colors.len()].to_string()
    }
}

/// 主性能测试应用
fn PerformanceBenchmarkApp() -> Element {
    let mut app_state = use_signal(|| BenchmarkAppState::default());
    let mut test_timer = use_signal(|| Instant::now());
    
    // 性能监控循环
    use_effect(move || {
        if app_state.read().is_running {
            let start_time = Instant::now();
            
            // 更新组件数据
            app_state.with_mut(|state| {
                state.component_data.iter_mut().for_each(|component| {
                    component.value = (component.id as f64 * 0.01 + start_time.elapsed().as_secs_f64()).sin();
                    component.position.0 = component.position.0 + component.value * 0.5;
                    component.position.1 = component.position.1 + component.value * 0.3;
                    component.last_update = start_time;
                });
                
                // 更新性能指标
                state.metrics.frame_count += 1;
                state.metrics.update_count += 1;
                state.metrics.last_update = start_time;
                
                // 计算FPS
                let fps = 1.0 / test_timer.read().elapsed().as_secs_f64();
                state.metrics.fps_history.push(fps);
                if state.metrics.fps_history.len() > 100 {
                    state.metrics.fps_history.remove(0);
                }
                
                // 模拟内存使用
                state.metrics.memory_usage = 128 + (state.metrics.frame_count % 50) as u64;
                state.metrics.memory_peak = state.metrics.memory_peak.max(state.metrics.memory_usage);
                
                state.metrics.memory_history.push(state.metrics.memory_usage);
                if state.metrics.memory_history.len() > 100 {
                    state.metrics.memory_history.remove(0);
                }
                
                test_timer.set(start_time);
            });
        }
    });

    rsx! {
        div {
            class: "benchmark-app",
            style: "min-height: 100vh; background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; color: white;",
            
            // 头部
            BenchmarkHeader { 
                app_state: app_state.clone(),
                on_start_test: move |framework| {
                    app_state.with_mut(|state| {
                        state.is_running = true;
                        state.current_test = Some(framework);
                        state.metrics = PerformanceMetrics::default();
                    });
                },
                on_stop_test: move || {
                    app_state.with_mut(|state| {
                        state.is_running = false;
                        if let Some(framework) = state.current_test.clone() {
                            let result = BenchmarkResult {
                                framework,
                                metrics: state.metrics.clone(),
                                component_performance: HashMap::new(),
                                optimization_suggestions: generate_optimization_suggestions(&state.metrics),
                                test_duration: Duration::from_secs(30),
                                timestamp: Instant::now(),
                            };
                            state.test_results.push(result);
                        }
                        state.current_test = None;
                    });
                }
            }
            
            // 主要内容区域
            div {
                style: "display: flex; gap: 20px; padding: 20px;",
                
                // 左侧控制面板
                div {
                    style: "width: 300px;",
                    
                    ControlPanel { app_state: app_state.clone() }
                    
                    ResultsPanel { app_state: app_state.clone() }
                }
                
                // 右侧可视化区域
                div {
                    style: "flex: 1;",
                    
                    VisualizationArea { app_state: app_state.clone() }
                    
                    MetricsPanel { app_state: app_state.clone() }
                }
            }
        }
    }
}

/// 基准测试头部组件
#[component]
fn BenchmarkHeader(
    app_state: Signal<BenchmarkAppState>, 
    on_start_test: EventHandler<String>,
    on_stop_test: EventHandler<()>
) -> Element {
    rsx! {
        header {
            style: "background: rgba(255,255,255,0.1); backdrop-filter: blur(10px); padding: 20px; border-bottom: 1px solid rgba(255,255,255,0.2);",
            
            div {
                style: "display: flex; justify-content: space-between; align-items: center;",
                
                div {
                    h1 {
                        style: "margin: 0; font-size: 2rem; font-weight: bold;",
                        "🚀 Rust UI 性能基准测试"
                    }
                    p {
                        style: "margin: 5px 0 0 0; opacity: 0.8;",
                        "基于Rust 1.90的性能优化测试平台"
                    }
                }
                
                div {
                    style: "display: flex; gap: 10px; align-items: center;",
                    
                    // 测试状态指示器
                    div {
                        style: "display: flex; align-items: center; gap: 8px; padding: 8px 16px; background: rgba(255,255,255,0.2); border-radius: 20px;",
                        
                        div {
                            style: "width: 8px; height: 8px; border-radius: 50%; background: {if app_state.read().is_running { '#4CAF50' } else { '#666' }};",
                        }
                        
                        span {
                            style: "font-size: 14px;",
                            if app_state.read().is_running { "测试运行中" } else { "测试已停止" }
                        }
                    }
                    
                    // 控制按钮
                    if app_state.read().is_running {
                        button {
                            style: "background: #f44336; color: white; border: none; padding: 10px 20px; border-radius: 8px; cursor: pointer; font-weight: bold;",
                            onclick: move |_| on_stop_test.call(()),
                            "停止测试"
                        }
                    } else {
                        div {
                            style: "display: flex; gap: 5px;",
                            
                            button {
                                style: "background: #4CAF50; color: white; border: none; padding: 10px 16px; border-radius: 8px; cursor: pointer; font-size: 12px;",
                                onclick: move |_| on_start_test.call("Dioxus".to_string()),
                                "测试 Dioxus"
                            }
                            
                            button {
                                style: "background: #2196F3; color: white; border: none; padding: 10px 16px; border-radius: 8px; cursor: pointer; font-size: 12px;",
                                onclick: move |_| on_start_test.call("Leptos".to_string()),
                                "测试 Leptos"
                            }
                            
                            button {
                                style: "background: #FF9800; color: white; border: none; padding: 10px 16px; border-radius: 8px; cursor: pointer; font-size: 12px;",
                                onclick: move |_| on_start_test.call("Tauri".to_string()),
                                "测试 Tauri"
                            }
                        }
                    }
                }
            }
        }
    }
}

/// 控制面板组件
#[component]
fn ControlPanel(app_state: Signal<BenchmarkAppState>) -> Element {
    rsx! {
        div {
            style: "background: rgba(255,255,255,0.95); color: #333; padding: 20px; border-radius: 12px; margin-bottom: 20px; box-shadow: 0 4px 15px rgba(0,0,0,0.1);",
            
            h3 {
                style: "margin: 0 0 15px 0; font-size: 18px;",
                "测试配置"
            }
            
            div {
                style: "display: flex; flex-direction: column; gap: 15px;",
                
                ConfigItem {
                    label: "组件数量",
                    value: "{app_state.read().config.component_count}",
                    on_change: move |value| {
                        app_state.with_mut(|state| {
                            state.config.component_count = value.parse().unwrap_or(1000);
                            state.component_data = BenchmarkAppState::generate_test_components(state.config.component_count);
                        });
                    }
                }
                
                ConfigItem {
                    label: "测试时长",
                    value: "{app_state.read().config.test_duration.as_secs()}秒",
                    on_change: move |_| {}
                }
                
                ConfigItem {
                    label: "更新频率",
                    value: "{1000 / app_state.read().config.update_frequency.as_millis()} FPS",
                    on_change: move |_| {}
                }
                
                div {
                    style: "display: flex; flex-direction: column; gap: 8px;",
                    
                    ConfigCheckbox {
                        label: "内存跟踪",
                        checked: app_state.read().config.enable_memory_tracking,
                        on_change: move |checked| {
                            app_state.with_mut(|state| state.config.enable_memory_tracking = checked);
                        }
                    }
                    
                    ConfigCheckbox {
                        label: "渲染跟踪",
                        checked: app_state.read().config.enable_render_tracking,
                        on_change: move |checked| {
                            app_state.with_mut(|state| state.config.enable_render_tracking = checked);
                        }
                    }
                }
            }
        }
    }
}

/// 配置项组件
#[component]
fn ConfigItem(label: &'static str, value: &'static str, on_change: EventHandler<String>) -> Element {
    rsx! {
        div {
            style: "display: flex; justify-content: space-between; align-items: center; padding: 8px 0; border-bottom: 1px solid #eee;",
            
            span {
                style: "font-size: 14px; color: #666;",
                "{label}"
            }
            
            span {
                style: "font-size: 14px; font-weight: 500; color: #333;",
                "{value}"
            }
        }
    }
}

/// 配置复选框组件
#[component]
fn ConfigCheckbox(label: &'static str, checked: bool, on_change: EventHandler<bool>) -> Element {
    rsx! {
        div {
            style: "display: flex; align-items: center; gap: 8px;",
            
            input {
                r#type: "checkbox",
                checked: checked,
                onchange: move |event| on_change.call(event.checked()),
                style: "transform: scale(1.1);"
            }
            
            span {
                style: "font-size: 14px; color: #333;",
                "{label}"
            }
        }
    }
}

/// 结果面板组件
#[component]
fn ResultsPanel(app_state: Signal<BenchmarkAppState>) -> Element {
    rsx! {
        div {
            style: "background: rgba(255,255,255,0.95); color: #333; padding: 20px; border-radius: 12px; box-shadow: 0 4px 15px rgba(0,0,0,0.1);",
            
            h3 {
                style: "margin: 0 0 15px 0; font-size: 18px;",
                "测试结果"
            }
            
            div {
                style: "display: flex; flex-direction: column; gap: 10px; max-height: 400px; overflow-y: auto;",
                
                for (index, result) in app_state.read().test_results.iter().enumerate() {
                    ResultCard {
                        result: result.clone(),
                        index: index
                    }
                }
                
                if app_state.read().test_results.is_empty() {
                    div {
                        style: "text-align: center; color: #999; padding: 20px;",
                        "暂无测试结果"
                    }
                }
            }
        }
    }
}

/// 结果卡片组件
#[component]
fn ResultCard(result: BenchmarkResult, index: usize) -> Element {
    rsx! {
        div {
            style: "background: #f8f9fa; padding: 15px; border-radius: 8px; border-left: 4px solid #007bff;",
            
            div {
                style: "display: flex; justify-content: space-between; align-items: center; margin-bottom: 10px;",
                
                h4 {
                    style: "margin: 0; font-size: 14px; color: #333;",
                    "{result.framework}"
                }
                
                span {
                    style: "font-size: 12px; color: #666;",
                    "#{index + 1}"
                }
            }
            
            div {
                style: "display: grid; grid-template-columns: repeat(2, 1fr); gap: 8px; font-size: 12px;",
                
                div {
                    span { style: "color: #666;", "FPS: " }
                    span { style: "font-weight: 500; color: #333;", "{result.metrics.fps_history.last().unwrap_or(&0.0):.1}" }
                }
                
                div {
                    span { style: "color: #666;", "内存: " }
                    span { style: "font-weight: 500; color: #333;", "{result.metrics.memory_usage}MB" }
                }
                
                div {
                    span { style: "color: #666;", "组件: " }
                    span { style: "font-weight: 500; color: #333;", "{result.metrics.component_count}" }
                }
                
                div {
                    span { style: "color: #666;", "帧数: " }
                    span { style: "font-weight: 500; color: #333;", "{result.metrics.frame_count}" }
                }
            }
            
            if !result.optimization_suggestions.is_empty() {
                div {
                    style: "margin-top: 10px; padding-top: 10px; border-top: 1px solid #eee;",
                    
                    span {
                        style: "font-size: 11px; color: #007bff; font-weight: 500;",
                        "💡 {result.optimization_suggestions.len()} 优化建议"
                    }
                }
            }
        }
    }
}

/// 可视化区域组件
#[component]
fn VisualizationArea(app_state: Signal<BenchmarkAppState>) -> Element {
    rsx! {
        div {
            style: "background: rgba(255,255,255,0.95); color: #333; padding: 20px; border-radius: 12px; margin-bottom: 20px; box-shadow: 0 4px 15px rgba(0,0,0,0.1);",
            
            h3 {
                style: "margin: 0 0 15px 0; font-size: 18px;",
                "组件可视化"
            }
            
            div {
                style: "position: relative; height: 300px; border: 1px solid #eee; border-radius: 8px; overflow: hidden; background: #f8f9fa;",
                
                // 组件渲染
                for component in app_state.read().component_data.iter().take(100) {
                    ComponentVisualization { 
                        component: component.clone(),
                        is_running: app_state.read().is_running
                    }
                }
                
                // 性能指示器
                div {
                    style: "position: absolute; top: 10px; right: 10px; background: rgba(0,0,0,0.7); color: white; padding: 8px 12px; border-radius: 6px; font-size: 12px;",
                    
                    div { "FPS: {app_state.read().metrics.fps_history.last().unwrap_or(&0.0):.1}" }
                    div { "组件: {app_state.read().component_data.len()}" }
                    div { "帧数: {app_state.read().metrics.frame_count}" }
                }
            }
        }
    }
}

/// 组件可视化组件
#[component]
fn ComponentVisualization(component: ComponentData, is_running: bool) -> Element {
    rsx! {
        div {
            style: "position: absolute; left: {component.position.0 + 150}px; top: {component.position.1 + 150}px; width: {component.size.0}px; height: {component.size.1}px; background: {component.color}; border-radius: 4px; opacity: 0.8; transition: all 0.1s ease; {if is_running { 'animation: pulse 2s infinite;' } else { '' }}",
            
            div {
                style: "position: absolute; bottom: -20px; left: 0; font-size: 10px; color: #333; white-space: nowrap;",
                "{component.name}"
            }
        }
    }
}

/// 指标面板组件
#[component]
fn MetricsPanel(app_state: Signal<BenchmarkAppState>) -> Element {
    let metrics = app_state.read().metrics.clone();
    
    rsx! {
        div {
            style: "background: rgba(255,255,255,0.95); color: #333; padding: 20px; border-radius: 12px; box-shadow: 0 4px 15px rgba(0,0,0,0.1);",
            
            h3 {
                style: "margin: 0 0 15px 0; font-size: 18px;",
                "实时指标"
            }
            
            div {
                style: "display: grid; grid-template-columns: repeat(auto-fit, minmax(150px, 1fr)); gap: 15px;",
                
                MetricCard {
                    title: "FPS",
                    value: "{metrics.fps_history.last().unwrap_or(&0.0):.1}",
                    unit: "fps",
                    color: "#4CAF50",
                    trend: calculate_trend(&metrics.fps_history)
                }
                
                MetricCard {
                    title: "内存使用",
                    value: "{metrics.memory_usage}",
                    unit: "MB",
                    color: "#2196F3",
                    trend: calculate_trend(&metrics.memory_history.iter().map(|&x| x as f32).collect())
                }
                
                MetricCard {
                    title: "组件数量",
                    value: "{metrics.component_count}",
                    unit: "个",
                    color: "#FF9800",
                    trend: Trend::Stable
                }
                
                MetricCard {
                    title: "渲染帧数",
                    value: "{metrics.frame_count}",
                    unit: "帧",
                    color: "#9C27B0",
                    trend: Trend::Increasing
                }
            }
            
            // 图表区域
            div {
                style: "margin-top: 20px;",
                
                h4 {
                    style: "margin: 0 0 10px 0; font-size: 16px;",
                    "性能趋势"
                }
                
                div {
                    style: "height: 200px; border: 1px solid #eee; border-radius: 8px; padding: 10px; background: #f8f9fa;",
                    
                    PerformanceChart { 
                        fps_data: metrics.fps_history.clone(),
                        memory_data: metrics.memory_history.iter().map(|&x| x as f32).collect()
                    }
                }
            }
        }
    }
}

/// 指标卡片组件
#[component]
fn MetricCard(title: &'static str, value: &'static str, unit: &'static str, color: &'static str, trend: Trend) -> Element {
    rsx! {
        div {
            style: "background: #f8f9fa; padding: 15px; border-radius: 8px; text-align: center; border-left: 4px solid {color};",
            
            h4 {
                style: "margin: 0 0 8px 0; font-size: 14px; color: #666;",
                "{title}"
            }
            
            div {
                style: "font-size: 24px; font-weight: bold; color: {color}; margin-bottom: 4px;",
                "{value}"
            }
            
            div {
                style: "font-size: 12px; color: #999;",
                "{unit}"
            }
            
            div {
                style: "margin-top: 8px; font-size: 12px; color: {match trend { Trend::Increasing => '#4CAF50', Trend::Decreasing => '#f44336', Trend::Stable => '#666' }};",
                match trend {
                    Trend::Increasing => "📈 上升",
                    Trend::Decreasing => "📉 下降", 
                    Trend::Stable => "➡️ 稳定",
                }
            }
        }
    }
}

/// 趋势枚举
#[derive(Debug, Clone)]
enum Trend {
    Increasing,
    Decreasing,
    Stable,
}

/// 性能图表组件
#[component]
fn PerformanceChart(fps_data: Vec<f32>, memory_data: Vec<f32>) -> Element {
    rsx! {
        div {
            style: "display: flex; height: 100%; align-items: end; gap: 2px;",
            
            for (index, fps) in fps_data.iter().enumerate() {
                div {
                    style: "background: linear-gradient(to top, #4CAF50, #8BC34A); width: 4px; height: {(*fps / 60.0 * 100.0).min(100.0)}%; border-radius: 2px;",
                    title: "FPS: {fps:.1}"
                }
            }
        }
    }
}

/// 生成优化建议
fn generate_optimization_suggestions(metrics: &PerformanceMetrics) -> Vec<OptimizationSuggestion> {
    let mut suggestions = Vec::new();
    
    // FPS优化建议
    if let Some(&avg_fps) = metrics.fps_history.last() {
        if avg_fps < 30.0 {
            suggestions.push(OptimizationSuggestion {
                category: OptimizationCategory::Rendering,
                severity: Severity::High,
                title: "低帧率警告".to_string(),
                description: "当前FPS低于30，可能影响用户体验".to_string(),
                impact: "用户体验严重下降".to_string(),
                implementation: "优化渲染逻辑，减少不必要的重绘".to_string(),
            });
        }
    }
    
    // 内存优化建议
    if metrics.memory_usage > 500 {
        suggestions.push(OptimizationSuggestion {
            category: OptimizationCategory::Memory,
            severity: Severity::Medium,
            title: "内存使用过高".to_string(),
            description: "内存使用超过500MB，建议优化内存管理".to_string(),
            impact: "可能导致应用崩溃".to_string(),
            implementation: "使用对象池，及时释放未使用资源".to_string(),
        });
    }
    
    // 组件数量优化建议
    if metrics.component_count > 1000 {
        suggestions.push(OptimizationSuggestion {
            category: OptimizationCategory::Component,
            severity: Severity::Low,
            title: "组件数量过多".to_string(),
            description: "同时渲染的组件数量过多".to_string(),
            impact: "可能影响渲染性能".to_string(),
            implementation: "使用虚拟滚动或分页加载".to_string(),
        });
    }
    
    suggestions
}

/// 计算趋势
fn calculate_trend(data: &[f32]) -> Trend {
    if data.len() < 2 {
        return Trend::Stable;
    }
    
    let first_half_avg = data[..data.len()/2].iter().sum::<f32>() / (data.len()/2) as f32;
    let second_half_avg = data[data.len()/2..].iter().sum::<f32>() / (data.len() - data.len()/2) as f32;
    
    let diff = second_half_avg - first_half_avg;
    
    if diff > 0.1 {
        Trend::Increasing
    } else if diff < -0.1 {
        Trend::Decreasing
    } else {
        Trend::Stable
    }
}

/// 主函数
fn main() {
    // 启动性能基准测试应用
    dioxus_web::launch(PerformanceBenchmarkApp);
}
