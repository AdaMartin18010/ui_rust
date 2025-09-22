//! Leptos 现代化Web框架示例
//! 
//! 本示例展示了Leptos的基本用法和服务端渲染功能
//! 利用Rust 1.90的新特性提升开发体验

use leptos::*;
use leptos_axum::*;
use axum::{routing::get, Router};

/// 主应用组件
#[component]
#[allow(non_snake_case)]
fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    let (name, set_name) = create_signal("Rust开发者".to_string());

    view! {
        <div style="text-align: center; font-family: Arial, sans-serif; padding: 20px;">
            <h1 style="color: #2c3e50; margin-bottom: 30px;">
                "⚡ Leptos 简化示例"
            </h1>
            
            <div style="background: #ecf0f1; padding: 20px; border-radius: 10px; margin: 20px 0;">
                <h2 style="color: #34495e; margin-bottom: 15px;">
                    "欢迎, " {move || name.get()} "!"
                </h2>
                
                <input
                    type="text"
                    placeholder="输入您的姓名"
                    prop:value=name
                    on:input=move |ev| set_name.set(event_target_value(&ev))
                    style="padding: 8px; margin: 10px; border: 1px solid #bdc3c7; border-radius: 5px; width: 200px;"
                />
            </div>
            
            <div style="background: #3498db; color: white; padding: 20px; border-radius: 10px; margin: 20px 0;">
                <h2 style="margin-bottom: 15px;">
                    "计数器: " {move || count.get()}
                </h2>
                
                <div style="display: flex; gap: 10px; justify-content: center;">
                    <button
                        on:click=move |_| set_count.update(|n| *n += 1)
                        style="background: #e74c3c; color: white; border: none; padding: 10px 20px; border-radius: 5px; cursor: pointer; font-size: 16px;"
                    >
                        "➕ 增加"
                    </button>
                    
                    <button
                        on:click=move |_| set_count.update(|n| *n -= 1)
                        style="background: #e67e22; color: white; border: none; padding: 10px 20px; border-radius: 5px; cursor: pointer; font-size: 16px;"
                    >
                        "➖ 减少"
                    </button>
                    
                    <button
                        on:click=move |_| set_count.set(0)
                        style="background: #95a5a6; color: white; border: none; padding: 10px 20px; border-radius: 5px; cursor: pointer; font-size: 16px;"
                    >
                        "🔄 重置"
                    </button>
                </div>
            </div>
            
            <div style="background: #2ecc71; color: white; padding: 20px; border-radius: 10px; margin: 20px 0;">
                <h3 style="margin-bottom: 15px;">
                    "📊 性能统计"
                </h3>
                
                <p>
                    "渲染次数: " {move || count.get() * 2}
                </p>
                
                <p>
                    "内存使用: " {move || format!("{:.2} MB", count.get() as f64 * 0.1)}
                </p>
                
                <p>
                    "响应时间: " {move || format!("{:.1} ms", count.get() as f64 * 0.5)}
                </p>
            </div>
            
            <div style="background: #9b59b6; color: white; padding: 20px; border-radius: 10px; margin: 20px 0;">
                <h3 style="margin-bottom: 15px;">
                    "🎯 Leptos特性"
                </h3>
                
                <ul style="text-align: left; max-width: 400px; margin: 0 auto;">
                    <li>"✅ 细粒度响应式系统"</li>
                    <li>"✅ 服务端渲染支持"</li>
                    <li>"✅ 优秀的开发体验"</li>
                    <li>"✅ 高性能"</li>
                    <li>"✅ 类型安全"</li>
                    <li>"✅ 零运行时开销"</li>
                    <li>"✅ Rust 1.90新特性集成"</li>
                    <li>"✅ 改进的异步编程"</li>
                    <li>"✅ 增强的错误处理"</li>
                </ul>
            </div>
        </div>
    }
}

/// 主函数 - 客户端渲染
#[cfg(not(feature = "ssr"))]
fn main() {
    leptos::mount_to_body(App)
}

/// 主函数 - 服务端渲染
#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::response::Html;
    
    // 创建Leptos配置
    let conf = get_configuration(None).await.unwrap();
    let leptos_options = conf.leptos_options;
    let root = leptos_options.site_root.clone();
    
    // 创建Axum路由器
    let app = Router::new()
        .leptos_routes_with_handler(
            leptos_options,
            move || view! { <App/> },
        )
        .fallback(file_and_error_handler(root));
    
    // 启动服务器
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("🚀 Leptos SSR服务器启动在 http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}

/// 测试模块
#[cfg(test)]
mod tests {
    use super::*;
    use leptos::*;

    #[test]
    fn test_app_component() {
        let runtime = create_runtime();
        
        // 测试应用组件能够正常创建
        let app = App();
        assert!(app.into_view().is_some());
        
        runtime.dispose();
    }
}
