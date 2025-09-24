# Rust 1.90 UI 全面指南 2025

> 版本: v1.0（Rust 1.90 对齐） 最后更新：2025-09

## 目录

1. 概览与目标
2. Rust 1.90 关键语言/库特性
3. UI 框架生态与选型原则
4. 推荐技术栈组合（Web/桌面/移动）
5. 架构模式与分层设计
6. 组件与组合式程序设计
7. 状态管理与数据流
8. 异步与并发模型（UI 场景）
9. 可访问性与国际化
10. 性能优化与基准
11. 测试策略与可观测性
12. 安全、权限与供应链
13. 构建与部署流水线
14. 进阶专题与最佳实践
15. 附录：代码片段与清单
16. 可运行示例与命令清单（新增）
17. 架构目录骨架与代码映射清单（新增）
18. Tauri 权限与安全清单（新增）
19. i18n 与 a11y 落地规范（新增）
20. 性能优化检查表与基准流程（新增）
21. 各UI框架快速模板与脚手架（新增）
22. 工程基线与 MSRV 策略（新增）
23. 错误处理与 UX 契约（新增）
24. WebAssembly/WASI 与 UI 互操作（新增）
25. 状态与数据同步（离线优先）（新增）
26. 架构落地清单与依赖方向守卫（新增）
27. 选型矩阵：成熟度/性能/生态（新增）
28. 迁移指南：升级到 Rust 1.90（新增）
29. 依赖与 features 建议（新增）
30. 常见陷阱与排障手册（新增）

---

## 1. 概览与目标

面向在 Rust 1.90 生态下构建 Web/桌面/移动 UI 的团队，提供成熟方案、工程化落地路径与可复制的组件设计方法，强调高性能、可维护、可观测与安全。

## 2. Rust 1.90 关键语言/库特性

- Cell::update（稳定）：更安全、简洁地原位更新内部值，适合 UI 局部可变状态。
- HashMap::extract_if（稳定）：高效筛选/迁移条目，适合 UI 缓存与索引维护。
- 模式匹配增强与 let-else：减少样板分支，提高可读性与错误处理一致性。
- 异步生态增强：结合 `tokio`/`futures` 构建后台任务与 I/O，不阻塞 UI 渲染线程。

示例：Cell::update 与 extract_if

```rust
use std::cell::Cell;
use std::collections::HashMap;

fn cell_update_example() {
    let clicks = Cell::new(0);
    clicks.update(|c| *c += 1);
}

fn extract_if_example() {
    let mut cache: HashMap<String, usize> = HashMap::from([
        ("btn".into(), 10),
        ("list".into(), 0),
    ]);
    let removed: Vec<_> = cache.extract_if(|_, v| *v == 0).collect();
    assert!(removed.iter().any(|(k, _)| k == "list"));
}
```

## 3. UI 框架生态与选型原则

- Web：Dioxus（DX 友好）、Leptos（极致性能）、Yew（成熟稳健）。
- 桌面：Tauri（轻量外壳/强安全模型）、Slint（原生渲染/低内存）、egui（即时模式）、Iced（声明式）。
- 移动：Dioxus Mobile（多端 UI）、Tauri Mobile（系统能力与分发通道）。

选型优先级：

- 需求贴合 > 团队经验 > 生态活跃度 > 可维护性 > 长期演进路径。

## 4. 推荐技术栈组合（Web/桌面/移动）

- Web（WASM）：Leptos + Axum + Vite/Trunk + `tracing` + OpenTelemetry
- 桌面：Tauri + 内嵌 Axum/IPC + `tracing` + Sentry SDK + 权限白名单
- 原生 GUI：Slint + tokio + anyhow/thiserror + `pprof-rs`
- 移动：Tauri Mobile 或 Dioxus Mobile + 后端 Axum + OTA 更新策略

## 5. 架构模式与分层设计

- 分层/六边形/洋葱架构：`domain`（业务）、`application`（用例）、`infrastructure`（外设）、`ui`（适配）。
- 依赖倒置：`ui` 依赖抽象接口，不直接耦合具体实现，便于多前端共用业务内核。
- 事件驱动：UI 发出意图 → 应用层处理 → 状态变更 → 视图渲染。

目录建议：

```text
src/
  ui/               # 视图与组件
  app/              # 用例服务（应用服务）
  domain/           # 领域模型与规则
  infra/            # 存储、网络、MQ 等适配
```

## 6. 组件与组合式程序设计

- 粒度：原子组件（按钮/输入）、复合组件（表单/对话框/列表）。
- 组合：以属性（props）与回调组合功能；避免深层继承，倾向组合优于继承。
- 不变式：组件输入保持纯净；副作用集中在 effect/hook；严格区分容器与展示组件。

Leptos 示例（简化）：

```rust
use leptos::*;

#[component]
pub fn Counter() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    view! {
        <button on:click=move |_| set_count.update(|c| *c += 1)>
            {move || count.get()}
        </button>
    }
}
```

## 7. 状态管理与数据流

- 局部状态：`Cell::update`、`RefCell`、框架自带 `Signal`/`Atom`。
- 全局状态：`Arc<RwLock<T>>` 或框架提供的上下文注入（context/provider）。
- 单向数据流：意图 Intent → 动作 Action → 状态 State → 视图 View。
- 切片化更新：尽量缩小重渲染粒度，使用选择器或派生信号。

## 8. 异步与并发模型（UI 场景）

- 后台任务：`tokio::spawn`、`JoinSet` 聚合；UI 线程专注渲染。
- I/O 隔离：网络/磁盘 I/O 在 runtime，结果通过 channel/Signal 回传。
- 取消与超时：`tokio::time::timeout`、`tokio_util::sync::CancellationToken`。
- 去抖/节流：对快速输入与滚动交互进行节制，减少重渲染。

## 9. 可访问性与国际化

- a11y：语义化组件、键盘导航、颜色对比度、焦点环与可视顺序一致。
- i18n：抽象文案字典，运行时切换语言与区域格式；数字/日期本地化。

## 10. 性能优化与基准

- 编译：`lto=true`、`codegen-units=1`、`opt-level=z`、`panic=abort`。
- 运行时：避免不必要的重渲染；差分更新；虚拟列表与窗口化渲染。
- 基准：`criterion`、`cargo bench`；`-Zself-profile`、`pprof-rs` 取火焰图。

## 11. 测试策略与可观测性

- 单元测试：领域与应用服务优先；mock 外设。
- 组件测试：快照 + 交互事件；对关键交互写回归测试。
- 端到端：`tauri-driver`/浏览器驱动/`wasm-bindgen-test`。
- 可观测性：`tracing` + OpenTelemetry；链路追踪贯穿 UI 与后端。

## 12. 安全、权限与供应链

- Tauri：CSP、命令权限白名单、API 封装与最小授权。
- 供应链：`cargo deny`、`cargo audit`、依赖许可与漏洞扫描。
- 数据安全：敏感信息最小化存储，使用系统安全存储与加密。

## 13. 构建与部署流水线

- CI：fmt + clippy + test + audit + build + package。
- Web：WASM 构建（Trunk/Vite）+ 静态资源 CDN 发布。
- 桌面/移动：Tauri 打包与签名；渠道发布（MSIX/DMG/IPA/APK）。

## 14. 进阶专题与最佳实践

- 插件化体系：命令/事件作为扩展点；插件注册与沙箱隔离。
- 主题与外观：系统暗黑模式；Design Tokens 与多主题切换。
- 离线优先：本地缓存与冲突合并策略；增量同步。

## 15. 附录：代码片段与清单

- Rust 1.90 API 索引（Cell/HashMap 等）。
- VSCode/rust-analyzer 最佳配置样例。
- Cargo profiles 与 `.cargo/config.toml` 模板清单。

## 16. 可运行示例与命令清单（新增）

### 16.1 Leptos（WASM）

```bash
# 构建与运行（Trunk）
cargo install trunk wasm-bindgen-cli --locked
trunk serve --open

# 构建生产包
trunk build --release
```

关键片段（组件+路由示例）：

```rust
use leptos::*;
use leptos_router::*;

#[component]
fn Home() -> impl IntoView { view!{ <h1>"Home"</h1> } }

#[component]
fn App() -> impl IntoView {
    view! {
        <Router>
            <Routes>
                <Route path="/" view=Home />
            </Routes>
        </Router>
    }
}
```

### 16.2 Dioxus（Web/Desktop/Mobile）

```bash
# CLI
cargo install dioxus-cli --locked
# Web 开发
dx serve
# 构建
dx build --release
```

组件示例：

```rust
use dioxus::prelude::*;

fn app(cx: Scope) -> Element {
    let count = use_state(cx, || 0);
    cx.render(rsx!{
        button { onclick: move |_| count += 1, "{count}" }
    })
}
```

### 16.3 Tauri（桌面/移动）

```bash
# 安装 CLI
cargo install tauri-cli --locked
# 开发调试
yarn tauri dev  # 或 npm/pnpm 对应命令
# 桌面打包
cargo tauri build
# 移动构建（需平台SDK）
cargo tauri android build
cargo tauri ios build
```

安全要点：在 `tauri.conf.json` 中限制 `allowlist` 与 `protocols`，开启 CSP。

### 16.4 Slint（原生 GUI）

```bash
# 运行示例
cargo run --example slint_example --features slint
# 发布构建
cargo build --release --features slint
```

UI 片段：

```rust
slint::slint! {
    export component App inherits Window {
        Text { text: "Hello, Slint"; }
    }
}
```

### 16.5 egui（即时模式）

```bash
# 运行示例
cargo run --example egui_example --features egui
```

片段：

```rust
fn ui(ui: &mut egui::Ui, count: &mut i32) {
    if ui.button("+1").clicked() { *count += 1; }
    ui.label(format!("count = {}", count));
}
```

### 16.6 测试与基准

```bash
# 单元/集成测试
cargo test
# 基准
cargo bench
```

### 16.7 可观测性 & 诊断

```rust
use tracing::{info, instrument};

#[instrument]
fn compute() { info!("start"); /* ... */ }
```

```bash
# 运行时查看日志
RUST_LOG=info cargo run
```

## 17. 架构目录骨架与代码映射清单（新增）

- 目标：让 UI、应用用例、领域模型、基础设施四层职责清晰、依赖方向单向。
- 约束：`ui -> app -> domain`（只向内依赖），`infra` 作为外设实现供 `app/domain` 注入。

目录骨架：

```text
src/
  ui/
    components/         # 纯视图组件（展示/受控）
    pages/              # 页面/路由
    services/           # UI 适配服务（请求组装、DTO 映射）
  app/
    usecases/           # 应用服务/用例（协调领域对象）
    ports/              # 端口接口（仓储/消息/网关抽象）
  domain/
    model/              # 领域实体/值对象/聚合
    services/           # 领域服务（纯业务规则）
  infra/
    adapters/           # 适配器（DB/HTTP/MQ 实现）
    repositories/       # 仓储实现
    telemetry/          # 日志/追踪/指标
```

代码映射：

- UI 调用 `app::usecases::*` 并只接触 DTO/Command，不直接引入领域实体。
- `app::ports::*` 定义仓储/网关接口；`infra::*` 提供实现并在组合根注入。
- 领域层不依赖任何框架，仅标准库与必要的无状态工具。

## 18. Tauri 权限与安全清单（新增）

- 配置 `tauri.conf.json`：
  - 限制 `allowlist`（fs、shell、dialog 等仅按需启用）。
  - 明确 `protocols` 自定义协议；禁用不必要的 `assetScope`。
  - 启用 CSP，默认拒绝内联脚本，使用 nonce/hash。
- 命令隔离：
  - Rust 侧 `tauri::command` 最小授权；校验输入与来源。
  - 前端仅通过安全 IPC 调用；避免直接暴露系统 API。
- 更新机制：
  - 校验签名与来源；使用 HTTPS 与签名校验的更新源。
- 依赖与构建：
  - `cargo deny`/`cargo audit` 持续检查；锁定版本与供应链来源。
- 数据保护：
  - 使用系统安全存储；持久化数据加密；最小化敏感信息驻留时间。

## 19. i18n 与 a11y 落地规范（新增）

- i18n：
  - 文案集中在字典（JSON/多语言资源）；避免硬编码。
  - 运行时语言切换；日期/货币/数字本地化格式。
  - 方向性（LTR/RTL）与区域特性适配。
- a11y：
  - 语义标签（role/aria-*）；键盘可达性（Tab/Enter/Esc）。
  - 焦点管理与可见焦点环；颜色对比度符合 WCAG AA。
  - 动画可减弱（prefers-reduced-motion）与可缩放文本。

## 20. 性能优化检查表与基准流程（新增）

检查表：

- 构建配置：release + `lto=true`、`codegen-units=1`、`opt-level=z`、`panic=abort`。
- 资源加载：按需加载/懒加载；静态资源指纹与缓存头。
- 渲染优化：避免不必要重渲染；虚拟列表/窗口化；大数据量分页/分块。
- 状态粒度：将热点状态切片化；使用派生信号减少依赖传播。
- 并发：`JoinSet`/批处理 I/O；限流与退避；避免阻塞 UI 线程。
- 观测：`tracing` 关键路径埋点；错误率/耗时/分位数指标。

基准流程：

```bash
# 1) 编写 criterion 基准
cargo add criterion --dev
# 2) 运行
cargo bench
# 3) 自分析与火焰图（本地）
cargo install flamegraph
cargo flamegraph --root
```

定位热点：

- 使用 `-Zself-profile`（夜间 rustc）或 `pprof-rs` 捕获 CPU/内存热点。
- 分析 UI 交互路径：事件 → 渲染 → 提交帧，找出最大瓶颈并针对性优化。

## 21. 各UI框架快速模板与脚手架（新增）

### 21.1 Leptos 模板

```bash
# 新建项目（推荐）
cargo generate --git https://github.com/leptos-rs/leptos --name my-leptos-app
# 开发
trunk serve --open
# 构建
trunk build --release
```

入口（main.rs）：

```rust
use leptos::*;
use my_leptos_app::app::App;

#[tokio::main]
async fn main() {
    leptos::mount_to_body(|| view! { <App/> })
}
```

### 21.2 Dioxus 模板

```bash
# 新建
cargo install dioxus-cli --locked
dx new my-dx-app --template fullstack
# 开发
dx serve
# 构建
dx build --release
```

入口：

```rust
use dioxus::prelude::*;
fn main() { dioxus::desktop::launch(app); }
fn app(cx: Scope) -> Element { cx.render(rsx!{ div{"Hello"} }) }
```

### 21.3 Tauri 模板（前端可选）

```bash
# 新建（使用官方脚手架）
cargo install create-tauri-app --locked
create-tauri-app my-tauri --manager pnpm --template vanilla
# 开发
yarn tauri dev
# 打包
cargo tauri build
```

Rust 入口：

```rust
#[tauri::command]
fn ping() -> String { "pong".into() }

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![ping])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
```

### 21.4 Slint 模板

```bash
cargo add slint
# 运行
cargo run --features slint
```

入口：

```rust
slint::slint! { export component App inherits Window { Text { text: "Hello"; } } }
fn main(){ App::new().unwrap().run().unwrap(); }
```

### 21.5 egui 模板（eframe）

```bash
cargo add eframe egui --features eframe/native
cargo run
```

入口：

```rust
fn main() -> eframe::Result<()> {
  eframe::run_native(
    "egui-app",
    eframe::NativeOptions::default(),
    Box::new(|_| Box::<MyApp>::default()),
  )
}

#[derive(Default)]
struct MyApp { count: i32 }
impl eframe::App for MyApp {
  fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    egui::CentralPanel::default().show(ctx, |ui| {
      if ui.button("+1").clicked() { self.count += 1; }
      ui.label(format!("count = {}", self.count));
    });
  }
}
```

## 22. 工程基线与 MSRV 策略（新增）

目标：统一工程下限（MSRV = Minimum Supported Rust Version）与团队工程基线，确保在 Rust 1.90 版本对齐的同时，允许有节奏地前向升级。

- 建议 MSRV：1.74+（如需使用 1.90 特性作为硬要求，则 MSRV=1.90）。
- 原则：构建链路固定 MSRV，开发链路可使用更高 stable；CI 强制校验。

建议的 Cargo 配置（`Cargo.toml` 片段）：

```toml
[package]
rust-version = "1.90"

[profile.release]
lto = true
codegen-units = 1
opt-level = "z"
panic = "abort"
strip = true

[profile.dev]
incremental = true

[workspace.metadata.msrv]
msrv = "1.90"
```

建议的 `.cargo/config.toml`（统一编译与 RUSTFLAGS）：

```toml
[build]
target-dir = "target"

[target.x86_64-pc-windows-msvc]
rustflags = ["-C", "target-cpu=native"]

[target.x86_64-unknown-linux-gnu]
rustflags = ["-C", "target-cpu=native"]

[target.aarch64-apple-darwin]
rustflags = ["-C", "target-cpu=apple-m1"]
```

CI（GitHub Actions）建议：

```yaml
name: ci
on: [push, pull_request]
jobs:
  build-test:
    runs-on: ubuntu-latest
    strategy:
      matrix:
        rust: [1.90.0]
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
        with:
          toolchain: ${{ matrix.rust }}
          components: clippy,rustfmt
      - name: Cache cargo
        uses: Swatinem/rust-cache@v2
      - run: cargo fetch --locked
      - run: cargo fmt --all -- --check
      - run: cargo clippy --all-targets --all-features -- -D warnings
      - run: cargo test --all --locked --features ""
      - run: cargo build --all --release --locked
  supply-chain:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - run: cargo install cargo-deny --locked
      - run: cargo install cargo-audit --locked
      - run: cargo deny check licenses bans advisories sources
      - run: cargo audit --db ~/advisory-db
```

依赖与版本锁定：

- 使用 `cargo update -w -Zminimal-versions`（在可控场景）验证下界。
- 生产分支固定 `Cargo.lock`，分阶段升级依赖并跑回归。

Windows/WSL/跨平台注意：

- Tauri/Slint/egui 在 Windows 下使用 MSVC 工具链；CI 额外覆盖 macOS 与 Linux。
- WASM 目标建议启用 `wasm32-unknown-unknown` 并通过 Trunk/Vite 构建。

可视化基线验证：

- 在应用首页暴露 `About/Diagnostics` 面板，显示 Rust 版本、构建 commit、feature 开关。

落地检查表：

- 已设置 `rust-version`、CI 锁定 1.90、Clippy 零警告、audit/deny 通过。
- release 配置 LTO/codegen-units=1，二进制开启 strip。
- 关键目标平台能成功构建与运行。
