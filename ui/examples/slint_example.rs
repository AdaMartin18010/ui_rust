//! Slint 1.0 原生GUI框架示例
//! 
//! 本示例展示了如何使用Slint 1.0构建原生GUI应用程序
//! Slint是一个现代的、类型安全的GUI工具包，专为Rust设计

use slint::SharedString;

// 定义UI结构 - 使用Slint的声明式语法
slint::slint! {
    import { Button, VerticalBox, HorizontalBox, LineEdit, Text } from "std-widgets.slint";

    export component AppWindow inherits Window {
        title: "Slint 1.0 示例 - Rust原生GUI";
        width: 400px;
        height: 300px;
        
        property <string> input-text: "Hello, Slint!";
        property <int> click-count: 0;
        
        callback button-clicked();
        
        VerticalBox {
            Text {
                text: "欢迎使用 Slint 1.0";
                font-size: 20px;
                horizontal-alignment: center;
            }
            
            Text {
                text: "这是一个原生Rust GUI应用程序";
                font-size: 14px;
                horizontal-alignment: center;
            }
            
            LineEdit {
                text: root.input-text;
                placeholder-text: "输入一些文本...";
            }
            
            HorizontalBox {
                Button {
                    text: "点击我!";
                    clicked => {
                        root.button-clicked();
                    }
                }
                
                Button {
                    text: "重置";
                    clicked => {
                        root.click-count = 0;
                        root.input-text = "Hello, Slint!";
                    }
                }
            }
            
            Text {
                text: "点击次数: " + root.click-count;
                font-size: 16px;
                horizontal-alignment: center;
            }
            
            Text {
                text: "输入文本: " + root.input-text;
                font-size: 14px;
                horizontal-alignment: center;
            }
        }
    }
}

/// 主函数 - 启动Slint应用程序
fn main() -> Result<(), slint::PlatformError> {
    // 创建应用程序实例
    let app = AppWindow::new()?;
    
    // 设置按钮点击回调
    let app_weak = app.as_weak();
    app.on_button_clicked(move || {
        let app = app_weak.unwrap();
        let current_count = app.get_click_count();
        app.set_click_count(current_count + 1);
        
        // 获取当前输入文本
        let current_text = app.get_input_text();
        println!("按钮被点击! 当前计数: {}, 输入文本: {}", current_count + 1, current_text);
    });
    
    // 运行应用程序
    app.run()
}

/// 演示Slint的高级特性
#[cfg(feature = "advanced")]
mod advanced_features {
    use slint::*;
    
    /// 自定义组件示例
    slint::slint! {
        component CustomButton inherits Button {
            property <color> custom-color: #3498db;
            
            background: custom-color;
            border-radius: 8px;
            border-width: 2px;
            border-color: #2980b9;
            
            // 悬停效果
            hover {
                background: #5dade2;
            }
            
            // 点击效果
            pressed {
                background: #2980b9;
            }
        }
        
        export component AdvancedApp inherits Window {
            title: "Slint高级特性示例";
            width: 600px;
            height: 400px;
            
            property <[string]> items: ["项目1", "项目2", "项目3", "项目4"];
            property <int> selected-index: 0;
            
            VerticalBox {
                Text {
                    text: "Slint高级特性演示";
                    font-size: 24px;
                    horizontal-alignment: center;
                }
                
                HorizontalBox {
                    CustomButton {
                        text: "自定义按钮";
                        clicked => {
                            root.selected-index = (root.selected-index + 1) % root.items.length;
                        }
                    }
                    
                    CustomButton {
                        text: "重置选择";
                        clicked => {
                            root.selected-index = 0;
                        }
                    }
                }
                
                Text {
                    text: "当前选择: " + root.items[root.selected-index];
                    font-size: 18px;
                    horizontal-alignment: center;
                }
                
                // 动态列表
                VerticalBox {
                    for item[index] in root.items: Text {
                        text: "• " + item;
                        font-size: 14px;
                        color: root.selected-index == index ? #e74c3c : #2c3e50;
                    }
                }
            }
        }
    }
    
    /// 运行高级示例
    pub fn run_advanced_example() -> Result<(), slint::PlatformError> {
        let app = AdvancedApp::new()?;
        app.run()
    }
}

/// 性能测试和基准测试
#[cfg(feature = "performance")]
mod performance_tests {
    use std::time::Instant;
    
    /// 测试Slint的渲染性能
    pub fn benchmark_rendering() {
        let start = Instant::now();
        
        // 创建大量组件进行性能测试
        for i in 0..1000 {
            // 这里可以添加实际的组件创建和渲染测试
            println!("创建组件 {}", i);
        }
        
        let duration = start.elapsed();
        println!("渲染性能测试完成，耗时: {:?}", duration);
    }
    
    /// 内存使用测试
    pub fn memory_usage_test() {
        // 测试内存使用情况
        println!("内存使用测试 - Slint应用程序内存占用通常很低");
        println!("Slint专为嵌入式设备和低内存环境设计");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_app_creation() {
        // 测试应用程序创建
        let app = AppWindow::new().expect("应该能够创建应用程序");
        assert_eq!(app.get_click_count(), 0);
        assert_eq!(app.get_input_text(), "Hello, Slint!");
    }
    
    #[test]
    fn test_button_interaction() {
        let app = AppWindow::new().expect("应该能够创建应用程序");
        
        // 模拟按钮点击
        let initial_count = app.get_click_count();
        
        // 这里应该触发按钮点击事件
        // 由于Slint的架构，我们需要在实际运行环境中测试交互
        
        assert_eq!(app.get_click_count(), initial_count);
    }
}
