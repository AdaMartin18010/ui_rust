# Rust 1.90 跨平台UI框架使用指南

## 🚀 快速开始

### 环境要求

- Rust 1.90+
- Cargo
- 目标平台相关依赖

### 安装依赖

```bash
# 克隆项目
git clone <your-repo>
cd ui_rust/ui

# 安装所有依赖
cargo build
```

## 🎯 运行示例

### 1. Dioxus 跨平台UI示例

```bash
# Web版本
cargo run --example dioxus_example --features dioxus

# Desktop版本 (如果支持)
cargo run --example dioxus_example --features dioxus-full
```

### 2. Leptos Web框架示例

```bash
# 客户端渲染
cargo run --example leptos_simple_example --features leptos

# 服务端渲染 (需要额外配置)
cargo run --example leptos_ssr_example --features leptos
```

### 3. Tauri 2.0 桌面应用示例

```bash
# 基础示例
cargo run --example tauri_basic_example --features tauri

# 完整示例
cargo run --example tauri_example --features tauri
```

### 4. Slint 1.5 原生GUI示例

```bash
cargo run --example slint_example --features slint
```

### 5. Iced 0.12 声明式GUI示例

```bash
cargo run --example iced_example --features iced
```

### 6. egui 0.27 即时模式GUI示例

```bash
cargo run --example egui_example --features egui
```

## 🔧 特性组合使用

### 现代UI框架组合

```bash
# 同时启用Dioxus和Leptos
cargo run --features modern-ui
```

### 桌面应用框架组合

```bash
# 启用所有桌面GUI框架
cargo run --features desktop-apps
```

### 跨平台UI框架组合

```bash
# 启用跨平台UI框架
cargo run --features cross-platform
```

### 高性能框架组合

```bash
# 启用高性能框架
cargo run --features high-performance
```

## 📱 平台特定构建

### Windows

```bash
# 确保安装了Windows SDK
cargo run --example slint_example --features slint --target x86_64-pc-windows-msvc
```

### macOS

```bash
# 确保安装了Xcode命令行工具
cargo run --example iced_example --features iced --target x86_64-apple-darwin
```

### Linux

```bash
# 确保安装了必要的系统库
sudo apt-get install libgtk-3-dev libwebkit2gtk-4.0-dev
cargo run --example tauri_example --features tauri
```

### WebAssembly

```bash
# 安装wasm-pack
cargo install wasm-pack

# 构建WebAssembly版本
wasm-pack build --target web --out-dir pkg
```

## 🛠️ 开发工具

### Dioxus开发工具

```bash
# 安装Dioxus CLI
cargo install dioxus-cli

# 启动开发服务器
dx serve

# 构建生产版本
dx build --release
```

### Leptos开发工具

```bash
# 安装Leptos CLI
cargo install cargo-leptos

# 热重载开发
cargo leptos watch

# 构建生产版本
cargo leptos build --release
```

### Tauri开发工具

```bash
# 安装Tauri CLI
cargo install tauri-cli

# 开发模式
tauri dev

# 构建生产版本
tauri build
```

## 🎨 自定义主题和样式

### Dioxus主题

```rust
use dioxus::prelude::*;

fn App() -> Element {
    rsx! {
        div { 
            class: "dark-theme",  // 自定义CSS类
            h1 { "我的应用" }
        }
    }
}
```

### Iced主题

```rust
use iced::Theme;

impl Application for MyApp {
    fn theme(&self) -> Theme {
        Theme::Dark  // 内置主题
    }
}
```

### Slint主题

```rust
slint::slint! {
    export component AppWindow inherits Window {
        title: "我的应用";
        // 使用内置主题
        default-font-family: "Arial";
    }
}
```

## 📊 性能优化

### 编译优化

```toml
# Cargo.toml
[profile.release]
opt-level = 3
lto = true
codegen-units = 1
panic = "abort"
```

### 运行时优化

```rust
// 使用Rust 1.90的新特性
use std::cell::Cell;

let cell = Cell::new(42);
cell.update(|x| x + 1);  // Rust 1.90新API
```

## 🐛 调试技巧

### 启用调试日志

```rust
use tracing::{info, debug, error};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    
    info!("应用启动");
    debug!("调试信息");
}
```

### 性能分析

```bash
# 使用cargo-flamegraph分析性能
cargo install flamegraph
cargo flamegraph --example my_example --features my_features
```

## 🔒 安全最佳实践

### 依赖安全

```bash
# 检查依赖安全漏洞
cargo audit

# 更新依赖
cargo update
```

### 代码安全

```rust
// 使用安全的字符串处理
use std::borrow::Cow;

fn safe_string(input: &str) -> Cow<str> {
    if input.len() > 100 {
        Cow::Owned(input[..100].to_string())
    } else {
        Cow::Borrowed(input)
    }
}
```

## 📚 学习资源

### 官方文档

- [Rust官方文档](https://doc.rust-lang.org/)
- [Dioxus文档](https://dioxuslabs.com/)
- [Leptos文档](https://leptos.dev/)
- [Tauri文档](https://tauri.app/)
- [Slint文档](https://slint-ui.com/)
- [Iced文档](https://docs.rs/iced/)
- [egui文档](https://docs.rs/egui/)

### 社区资源

- [Rust GUI工作组](https://github.com/rust-gui)
- [Are We GUI Yet](https://areweguiyet.com/)
- [Rust Web开发指南](https://rust-web-dev-guide.com/)

### 示例项目

- [Dioxus示例](https://github.com/dioxuslabs/dioxus/tree/master/examples)
- [Leptos示例](https://github.com/leptos-rs/leptos/tree/main/examples)
- [Tauri示例](https://github.com/tauri-apps/tauri/tree/dev/examples)

## ❓ 常见问题

### Q: 如何选择最适合的UI框架？

A: 参考 `CROSS_PLATFORM_UI_COMPARISON_2025.md` 文档中的详细对比和选择指南。

### Q: 如何在不同平台间共享代码？

A: 使用条件编译和平台特定模块：

```rust
#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "macos")]
mod macos;

#[cfg(target_arch = "wasm32")]
mod web;
```

### Q: 如何处理跨平台UI差异？

A: 使用平台特定的样式和组件：

```rust
#[cfg(target_os = "windows")]
const BUTTON_STYLE: &str = "windows-button";

#[cfg(target_os = "macos")]
const BUTTON_STYLE: &str = "macos-button";
```

### Q: 如何优化应用启动时间？

A:

1. 使用Rust 1.90的编译优化
2. 延迟加载非关键组件
3. 优化依赖关系
4. 使用链接时优化(LTO)

## 🤝 贡献指南

### 提交代码

1. Fork项目
2. 创建特性分支
3. 提交更改
4. 创建Pull Request

### 报告问题

1. 检查现有Issues
2. 提供详细的重现步骤
3. 包含系统信息和错误日志

### 改进文档

1. 更新相关文档
2. 添加代码示例
3. 改进说明的清晰度

---

**让Rust UI生态系统更加繁荣！** 🦀✨

*最后更新: 2025年1月*-
