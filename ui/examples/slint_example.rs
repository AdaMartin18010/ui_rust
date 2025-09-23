//! Slint 1.13 原生GUI框架示例
//! 
//! 本示例展示了如何使用Slint 1.13构建原生GUI应用程序
//! Slint是一个现代的、类型安全的GUI工具包，专为Rust设计

// 定义UI结构 - 使用Slint的声明式语法
slint::slint! {
    import { Button, VerticalBox, HorizontalBox, LineEdit } from "std-widgets.slint";

    export component AppWindow inherits Window {
        title: "Slint 1.13 示例 - Rust原生GUI";
        width: 400px;
        height: 300px;
        
        property <string> input-text: "Hello, Slint!";
        property <int> click-count: 0;
        
        VerticalBox {
            Text {
                text: "欢迎使用 Slint 1.13";
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
                        root.click-count = root.click-count + 1;
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
    
    // 运行应用程序
    app.run()
}