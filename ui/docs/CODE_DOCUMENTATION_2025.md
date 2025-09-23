# Rust 1.90 跨平台UI框架代码文档

## 📋 概述

本文档提供了项目中所有代码模块的详细注释和解释，帮助开发者理解代码结构、设计理念和实现细节。

## 🏗️ 核心架构

### 状态管理系统

```rust
/// 应用状态管理器 - 基于Rust 1.90优化的状态管理
/// 
/// 设计理念:
/// 1. 类型安全: 利用Rust的类型系统保证状态一致性
/// 2. 性能优化: 使用Rc<RefCell<T>>避免不必要的克隆
/// 3. 响应式更新: 自动追踪状态变化并触发更新
/// 4. 内存安全: 防止数据竞争和内存泄漏
/// 
/// 使用示例:
/// ```rust
/// let mut state = AppState::new();
/// state.update(|s| s.counter += 1);
/// state.subscribe(|new_state| println!("状态更新: {:?}", new_state));
/// ```
pub struct AppState<T: Clone + 'static> {
    /// 当前状态值，使用Rc<RefCell<T>>实现内部可变性
    /// Rc: 引用计数智能指针，允许多个所有者
    /// RefCell: 运行时借用检查，允许可变借用
    data: Rc<RefCell<T>>,
    
    /// 状态变化订阅者列表
    /// 使用Weak引用避免循环引用导致的内存泄漏
    subscribers: Vec<Weak<dyn Fn(&T)>>,
    
    /// 状态历史记录，用于撤销/重做功能
    /// 使用VecDeque实现高效的前后插入和删除
    history: VecDeque<T>,
    
    /// 最大历史记录数量，防止内存无限增长
    max_history_size: usize,
    
    /// 状态变化计数器，用于调试和性能分析
    change_count: AtomicUsize,
}

impl<T: Clone + 'static> AppState<T> {
    /// 创建新的状态管理器
    /// 
    /// # 参数
    /// * `initial_state` - 初始状态值
    /// 
    /// # 返回值
    /// 返回新创建的AppState实例
    /// 
    /// # 示例
    /// ```rust
    /// let state = AppState::new(CounterState { value: 0 });
    /// ```
    pub fn new(initial_state: T) -> Self {
        Self {
            data: Rc::new(RefCell::new(initial_state)),
            subscribers: Vec::new(),
            history: VecDeque::new(),
            max_history_size: 50, // 默认保存50个历史状态
            change_count: AtomicUsize::new(0),
        }
    }
    
    /// 更新状态值
    /// 
    /// 使用闭包更新状态，确保类型安全和原子性
    /// 
    /// # 参数
    /// * `updater` - 状态更新函数
    /// 
    /// # 示例
    /// ```rust
    /// state.update(|s| {
    ///     s.counter += 1;
    ///     s.last_updated = Instant::now();
    /// });
    /// ```
    pub fn update<F>(&mut self, updater: F)
    where
        F: FnOnce(&mut T),
    {
        // 获取当前状态的可变引用
        let mut current_state = self.data.borrow_mut();
        
        // 保存当前状态到历史记录
        self.save_to_history(current_state.clone());
        
        // 应用更新
        updater(&mut current_state);
        
        // 增加变化计数
        self.change_count.fetch_add(1, Ordering::SeqCst);
        
        // 通知所有订阅者
        self.notify_subscribers(&current_state);
    }
    
    /// 订阅状态变化
    /// 
    /// 当状态发生变化时，会自动调用订阅者函数
    /// 
    /// # 参数
    /// * `callback` - 状态变化回调函数
    /// 
    /// # 示例
    /// ```rust
    /// state.subscribe(|new_state| {
    ///     println!("状态更新: {:?}", new_state);
    /// });
    /// ```
    pub fn subscribe<F>(&mut self, callback: F)
    where
        F: Fn(&T) + 'static,
    {
        // 将强引用转换为弱引用，避免循环引用
        let weak_callback = Rc::downgrade(&Rc::new(callback));
        self.subscribers.push(weak_callback);
    }
    
    /// 获取当前状态值的克隆
    /// 
    /// # 返回值
    /// 当前状态的克隆副本
    pub fn get_state(&self) -> T {
        self.data.borrow().clone()
    }
    
    /// 保存状态到历史记录
    /// 
    /// 私有方法，用于维护状态历史
    fn save_to_history(&mut self, state: T) {
        self.history.push_back(state);
        
        // 限制历史记录大小
        if self.history.len() > self.max_history_size {
            self.history.pop_front();
        }
    }
    
    /// 通知所有订阅者状态已变化
    /// 
    /// 私有方法，清理无效的订阅者引用
    fn notify_subscribers(&mut self, new_state: &T) {
        // 过滤掉已被释放的订阅者
        self.subscribers.retain(|weak_ref| {
            if let Some(callback) = weak_ref.upgrade() {
                callback(new_state);
                true // 保留有效的订阅者
            } else {
                false // 移除无效的订阅者
            }
        });
    }
}
```

### 事件处理系统

```rust
/// 事件类型枚举 - 定义所有可能的UI事件
/// 
/// 设计考虑:
/// 1. 类型安全: 使用枚举确保事件类型的正确性
/// 2. 性能优化: 使用Box<dyn Any>避免泛型膨胀
/// 3. 扩展性: 易于添加新的事件类型
/// 4. 调试友好: 实现Debug trait便于调试
#[derive(Debug, Clone)]
pub enum UIEvent {
    /// 鼠标点击事件
    /// 包含点击位置和按钮信息
    MouseClick {
        position: Point2D<f32>,
        button: MouseButton,
        modifiers: KeyModifiers,
    },
    
    /// 键盘按键事件
    /// 包含按键码和修饰键状态
    KeyPress {
        key: KeyCode,
        modifiers: KeyModifiers,
        is_repeat: bool,
    },
    
    /// 窗口大小变化事件
    /// 包含新的窗口尺寸
    WindowResize {
        new_size: Size2D<f32>,
    },
    
    /// 自定义事件
    /// 允许用户定义特定的事件类型
    Custom {
        event_type: String,
        data: Box<dyn Any + Send + Sync>,
    },
}

/// 事件处理器trait
/// 
/// 定义了事件处理的标准接口
/// 所有事件处理器都必须实现此trait
pub trait EventHandler {
    /// 处理事件
    /// 
    /// # 参数
    /// * `event` - 要处理的事件
    /// 
    /// # 返回值
    /// * `Ok(())` - 事件处理成功
    /// * `Err(EventError)` - 事件处理失败
    fn handle(&self, event: &UIEvent) -> Result<(), EventError>;
    
    /// 获取事件处理器的优先级
    /// 
    /// 数值越小优先级越高
    fn priority(&self) -> u32 {
        100 // 默认优先级
    }
    
    /// 检查是否能够处理指定事件
    /// 
    /// # 参数
    /// * `event` - 要检查的事件
    /// 
    /// # 返回值
    /// 如果能够处理返回true，否则返回false
    fn can_handle(&self, event: &UIEvent) -> bool;
}

/// 事件分发器 - 负责将事件分发给相应的处理器
/// 
/// 设计特点:
/// 1. 优先级队列: 按优先级顺序处理事件
/// 2. 链式处理: 支持多个处理器处理同一事件
/// 3. 错误处理: 优雅处理处理器错误
/// 4. 性能优化: 使用HashMap快速查找处理器
pub struct EventDispatcher {
    /// 事件处理器映射表
    /// Key: 事件类型，Value: 处理器列表
    handlers: HashMap<TypeId, Vec<Box<dyn EventHandler>>>,
    
    /// 全局事件处理器列表
    /// 这些处理器会处理所有类型的事件
    global_handlers: Vec<Box<dyn EventHandler>>,
    
    /// 事件处理统计信息
    stats: EventStats,
}

impl EventDispatcher {
    /// 注册事件处理器
    /// 
    /// # 参数
    /// * `event_type` - 事件类型
    /// * `handler` - 事件处理器
    /// 
    /// # 示例
    /// ```rust
    /// let mut dispatcher = EventDispatcher::new();
    /// dispatcher.register_handler(UIEvent::MouseClick, Box::new(MouseHandler::new()));
    /// ```
    pub fn register_handler<E, H>(&mut self, handler: H)
    where
        E: UIEvent + 'static,
        H: EventHandler + 'static,
    {
        let type_id = TypeId::of::<E>();
        let handlers = self.handlers.entry(type_id).or_insert_with(Vec::new);
        
        // 按优先级插入处理器
        let priority = handler.priority();
        let insert_pos = handlers
            .binary_search_by_key(&priority, |h| h.priority())
            .unwrap_or_else(|pos| pos);
        
        handlers.insert(insert_pos, Box::new(handler));
    }
    
    /// 分发事件
    /// 
    /// # 参数
    /// * `event` - 要分发的事件
    /// 
    /// # 返回值
    /// 事件分发结果统计
    pub fn dispatch<E: UIEvent>(&self, event: &E) -> DispatchResult {
        let start_time = Instant::now();
        let mut result = DispatchResult::default();
        
        // 获取特定类型的处理器
        let type_id = TypeId::of::<E>();
        let handlers = self.handlers.get(&type_id);
        
        // 处理特定类型的处理器
        if let Some(handlers) = handlers {
            for handler in handlers {
                if handler.can_handle(event) {
                    match handler.handle(event) {
                        Ok(()) => result.success_count += 1,
                        Err(e) => {
                            result.error_count += 1;
                            result.errors.push(e);
                        }
                    }
                }
            }
        }
        
        // 处理全局处理器
        for handler in &self.global_handlers {
            if handler.can_handle(event) {
                match handler.handle(event) {
                    Ok(()) => result.success_count += 1,
                    Err(e) => {
                        result.error_count += 1;
                        result.errors.push(e);
                    }
                }
            }
        }
        
        result.processing_time = start_time.elapsed();
        result
    }
}
```

### 渲染系统

```rust
/// 渲染器trait - 定义渲染接口
/// 
/// 设计目标:
/// 1. 抽象化: 支持多种渲染后端
/// 2. 性能: 批量渲染和缓存优化
/// 3. 扩展性: 支持自定义渲染效果
/// 4. 调试: 提供渲染统计信息
pub trait Renderer {
    /// 开始渲染帧
    /// 
    /// 设置渲染上下文，准备渲染资源
    fn begin_frame(&mut self, clear_color: Color) -> Result<(), RenderError>;
    
    /// 结束渲染帧
    /// 
    /// 提交渲染命令，交换缓冲区
    fn end_frame(&mut self) -> Result<(), RenderError>;
    
    /// 渲染几何体
    /// 
    /// # 参数
    /// * `geometry` - 要渲染的几何体
    /// * `transform` - 变换矩阵
    /// * `material` - 材质属性
    fn render_geometry(
        &mut self,
        geometry: &Geometry,
        transform: &Transform,
        material: &Material,
    ) -> Result<(), RenderError>;
    
    /// 渲染文本
    /// 
    /// # 参数
    /// * `text` - 要渲染的文本
    /// * `position` - 文本位置
    /// * `font` - 字体信息
    /// * `color` - 文本颜色
    fn render_text(
        &mut self,
        text: &str,
        position: Point2D<f32>,
        font: &Font,
        color: Color,
    ) -> Result<(), RenderError>;
    
    /// 获取渲染统计信息
    fn get_stats(&self) -> RenderStats;
}

/// 渲染统计信息
/// 
/// 用于性能监控和调试
#[derive(Debug, Clone, Default)]
pub struct RenderStats {
    /// 渲染的三角形数量
    pub triangles_rendered: usize,
    
    /// 渲染的绘制调用次数
    pub draw_calls: usize,
    
    /// 渲染帧时间
    pub frame_time: Duration,
    
    /// 内存使用量
    pub memory_usage: usize,
    
    /// 缓存命中率
    pub cache_hit_rate: f32,
}

/// 渲染优化器 - 负责优化渲染性能
/// 
/// 优化策略:
/// 1. 批处理: 合并相同的渲染命令
/// 2. 剔除: 移除不可见的几何体
/// 3. 缓存: 缓存频繁使用的资源
/// 4. LOD: 根据距离调整细节级别
pub struct RenderOptimizer {
    /// 批处理器
    batcher: BatchRenderer,
    
    /// 剔除器
    culler: FrustumCuller,
    
    /// 资源缓存
    cache: RenderCache,
    
    /// 性能统计
    stats: RenderStats,
}

impl RenderOptimizer {
    /// 优化渲染命令列表
    /// 
    /// # 参数
    /// * `commands` - 原始渲染命令列表
    /// 
    /// # 返回值
    /// 优化后的渲染命令列表
    pub fn optimize_commands(&mut self, commands: Vec<RenderCommand>) -> Vec<RenderCommand> {
        let start_time = Instant::now();
        
        // 1. 剔除不可见的几何体
        let visible_commands = self.culler.cull_commands(commands);
        
        // 2. 批处理相同的渲染命令
        let batched_commands = self.batcher.batch_commands(visible_commands);
        
        // 3. 排序渲染命令以减少状态切换
        let sorted_commands = self.sort_commands(batched_commands);
        
        // 更新统计信息
        self.stats.frame_time = start_time.elapsed();
        
        sorted_commands
    }
    
    /// 排序渲染命令
    /// 
    /// 按材质和深度排序，减少GPU状态切换
    fn sort_commands(&self, mut commands: Vec<RenderCommand>) -> Vec<RenderCommand> {
        commands.sort_by(|a, b| {
            // 首先按材质排序
            let material_cmp = a.material.id().cmp(&b.material.id());
            if material_cmp != Ordering::Equal {
                return material_cmp;
            }
            
            // 然后按深度排序（从前往后）
            a.depth.partial_cmp(&b.depth).unwrap_or(Ordering::Equal)
        });
        
        commands
    }
}
```

## 🎨 UI组件系统

### 基础组件

```rust
/// 基础UI组件trait
/// 
/// 定义了所有UI组件必须实现的基本接口
/// 
/// 设计原则:
/// 1. 组合优于继承: 使用组合构建复杂组件
/// 2. 单一职责: 每个组件只负责一个功能
/// 3. 可测试性: 组件易于单元测试
/// 4. 可重用性: 组件可以在不同场景中重用
pub trait UIComponent {
    /// 组件ID，用于唯一标识组件
    fn id(&self) -> &ComponentId;
    
    /// 渲染组件
    /// 
    /// # 参数
    /// * `renderer` - 渲染器
    /// * `context` - 渲染上下文
    /// 
    /// # 返回值
    /// 渲染结果
    fn render(&self, renderer: &mut dyn Renderer, context: &RenderContext) -> Result<(), RenderError>;
    
    /// 处理事件
    /// 
    /// # 参数
    /// * `event` - UI事件
    /// 
    /// # 返回值
    /// 事件处理结果
    fn handle_event(&mut self, event: &UIEvent) -> EventResult;
    
    /// 获取组件边界框
    fn bounds(&self) -> Rect2D<f32>;
    
    /// 设置组件位置
    fn set_position(&mut self, position: Point2D<f32>);
    
    /// 设置组件大小
    fn set_size(&mut self, size: Size2D<f32>);
    
    /// 检查点是否在组件内
    fn contains_point(&self, point: Point2D<f32>) -> bool;
    
    /// 获取组件状态
    fn state(&self) -> ComponentState;
    
    /// 更新组件状态
    fn update_state(&mut self, state: ComponentState);
}

/// 按钮组件实现
/// 
/// 展示如何实现具体的UI组件
pub struct Button {
    /// 组件ID
    id: ComponentId,
    
    /// 组件位置和大小
    bounds: Rect2D<f32>,
    
    /// 按钮文本
    text: String,
    
    /// 按钮状态
    state: ButtonState,
    
    /// 样式配置
    style: ButtonStyle,
    
    /// 点击回调函数
    on_click: Option<Box<dyn Fn() + Send + Sync>>,
    
    /// 悬停回调函数
    on_hover: Option<Box<dyn Fn() + Send + Sync>>,
}

impl Button {
    /// 创建新按钮
    /// 
    /// # 参数
    /// * `id` - 组件ID
    /// * `text` - 按钮文本
    /// * `bounds` - 按钮边界
    /// 
    /// # 示例
    /// ```rust
    /// let button = Button::new(
    ///     ComponentId::new("submit_button"),
    ///     "提交",
    ///     Rect2D::new(0.0, 0.0, 100.0, 40.0)
    /// );
    /// ```
    pub fn new(id: ComponentId, text: String, bounds: Rect2D<f32>) -> Self {
        Self {
            id,
            bounds,
            text,
            state: ButtonState::Normal,
            style: ButtonStyle::default(),
            on_click: None,
            on_hover: None,
        }
    }
    
    /// 设置点击回调
    /// 
    /// # 参数
    /// * `callback` - 点击回调函数
    pub fn on_click<F>(mut self, callback: F) -> Self
    where
        F: Fn() + Send + Sync + 'static,
    {
        self.on_click = Some(Box::new(callback));
        self
    }
    
    /// 设置悬停回调
    /// 
    /// # 参数
    /// * `callback` - 悬停回调函数
    pub fn on_hover<F>(mut self, callback: F) -> Self
    where
        F: Fn() + Send + Sync + 'static,
    {
        self.on_hover = Some(Box::new(callback));
        self
    }
}

impl UIComponent for Button {
    fn id(&self) -> &ComponentId {
        &self.id
    }
    
    fn render(&self, renderer: &mut dyn Renderer, context: &RenderContext) -> Result<(), RenderError> {
        // 根据状态选择样式
        let style = match self.state {
            ButtonState::Normal => &self.style.normal,
            ButtonState::Hovered => &self.style.hovered,
            ButtonState::Pressed => &self.style.pressed,
            ButtonState::Disabled => &self.style.disabled,
        };
        
        // 渲染按钮背景
        let background_geometry = self.create_background_geometry();
        renderer.render_geometry(
            &background_geometry,
            &Transform::identity(),
            &style.background_material,
        )?;
        
        // 渲染按钮边框
        if style.border_width > 0.0 {
            let border_geometry = self.create_border_geometry();
            renderer.render_geometry(
                &border_geometry,
                &Transform::identity(),
                &style.border_material,
            )?;
        }
        
        // 渲染按钮文本
        let text_position = self.calculate_text_position();
        renderer.render_text(
            &self.text,
            text_position,
            &self.style.font,
            style.text_color,
        )?;
        
        Ok(())
    }
    
    fn handle_event(&mut self, event: &UIEvent) -> EventResult {
        match event {
            UIEvent::MouseClick { position, .. } => {
                if self.contains_point(*position) && self.state != ButtonState::Disabled {
                    self.state = ButtonState::Pressed;
                    
                    // 触发点击回调
                    if let Some(ref callback) = self.on_click {
                        callback();
                    }
                    
                    return EventResult::Handled;
                }
            }
            UIEvent::MouseMove { position } => {
                if self.contains_point(*position) && self.state != ButtonState::Disabled {
                    if self.state == ButtonState::Normal {
                        self.state = ButtonState::Hovered;
                        
                        // 触发悬停回调
                        if let Some(ref callback) = self.on_hover {
                            callback();
                        }
                    }
                    return EventResult::Handled;
                } else if self.state == ButtonState::Hovered {
                    self.state = ButtonState::Normal;
                }
            }
            _ => {}
        }
        
        EventResult::NotHandled
    }
    
    fn bounds(&self) -> Rect2D<f32> {
        self.bounds
    }
    
    fn set_position(&mut self, position: Point2D<f32>) {
        self.bounds.x = position.x;
        self.bounds.y = position.y;
    }
    
    fn set_size(&mut self, size: Size2D<f32>) {
        self.bounds.width = size.width;
        self.bounds.height = size.height;
    }
    
    fn contains_point(&self, point: Point2D<f32>) -> bool {
        self.bounds.contains(point)
    }
    
    fn state(&self) -> ComponentState {
        ComponentState::Button(self.state.clone())
    }
    
    fn update_state(&mut self, state: ComponentState) {
        if let ComponentState::Button(button_state) = state {
            self.state = button_state;
        }
    }
}
```

## 🔧 工具函数和宏

### 性能分析宏

```rust
/// 性能分析宏 - 用于测量代码执行时间
/// 
/// 用法:
/// ```rust
/// time_it!(operation_name, {
///     // 要测量的代码
///     expensive_operation();
/// });
/// ```
#[macro_export]
macro_rules! time_it {
    ($name:expr, $code:block) => {{
        let start = std::time::Instant::now();
        let result = $code;
        let duration = start.elapsed();
        
        #[cfg(debug_assertions)]
        {
            println!("⏱️  {}: {:?}", $name, duration);
        }
        
        // 在发布模式下，可以选择记录到性能日志
        #[cfg(not(debug_assertions))]
        {
            performance_logger::record_operation($name, duration);
        }
        
        result
    }};
}

/// 内存使用分析宏
/// 
/// 用法:
/// ```rust
/// memory_check!(operation_name, {
///     // 要分析的代码
///     allocate_large_data();
/// });
/// ```
#[macro_export]
macro_rules! memory_check {
    ($name:expr, $code:block) => {{
        let before = get_memory_usage();
        let result = $code;
        let after = get_memory_usage();
        let diff = after - before;
        
        #[cfg(debug_assertions)]
        {
            println!("💾 {}: {} bytes", $name, diff);
        }
        
        result
    }};
}

/// 条件编译宏 - 根据特性标志编译不同代码
/// 
/// 用法:
/// ```rust
/// conditional_compile!(feature = "debug", {
///     println!("调试信息");
/// });
/// ```
#[macro_export]
macro_rules! conditional_compile {
    (feature = $feature:literal, $code:block) => {{
        #[cfg(feature = $feature)]
        $code
    }};
}
```

### 错误处理工具

```rust
/// 错误处理结果类型
/// 
/// 提供更丰富的错误信息，包括错误上下文和建议
pub type Result<T, E = UIError> = std::result::Result<T, E>;

/// 错误上下文结构体
/// 
/// 用于提供错误的详细上下文信息
#[derive(Debug, Clone)]
pub struct ErrorContext {
    /// 错误发生的位置
    pub location: String,
    
    /// 错误发生的时间
    pub timestamp: Instant,
    
    /// 错误的上下文数据
    pub context: HashMap<String, String>,
    
    /// 错误建议
    pub suggestions: Vec<String>,
}

/// 错误处理工具函数
pub mod error_utils {
    use super::*;
    
    /// 创建带上下文的错误
    /// 
    /// # 参数
    /// * `error` - 原始错误
    /// * `location` - 错误位置
    /// * `context` - 上下文信息
    /// 
    /// # 返回值
    /// 带上下文的错误
    pub fn with_context<E: std::error::Error + Send + Sync + 'static>(
        error: E,
        location: &str,
        context: HashMap<String, String>,
    ) -> UIError {
        UIError::ContextualError {
            source: Box::new(error),
            context: ErrorContext {
                location: location.to_string(),
                timestamp: Instant::now(),
                context,
                suggestions: Vec::new(),
            },
        }
    }
    
    /// 添加错误建议
    /// 
    /// # 参数
    /// * `error` - 错误
    /// * `suggestions` - 建议列表
    pub fn add_suggestions(mut error: UIError, suggestions: Vec<String>) -> UIError {
        if let UIError::ContextualError { ref mut context, .. } = error {
            context.suggestions.extend(suggestions);
        }
        error
    }
    
    /// 记录错误到日志
    /// 
    /// # 参数
    /// * `error` - 错误
    /// * `logger` - 日志记录器
    pub fn log_error(error: &UIError, logger: &dyn Logger) {
        match error {
            UIError::ContextualError { source, context } => {
                logger.error(&format!(
                    "错误发生在 {}: {} (上下文: {:?})",
                    context.location,
                    source,
                    context.context
                ));
                
                if !context.suggestions.is_empty() {
                    logger.info(&format!("建议: {}", context.suggestions.join(", ")));
                }
            }
            _ => {
                logger.error(&format!("错误: {}", error));
            }
        }
    }
}
```

## 📊 性能监控

### 性能指标收集

```rust
/// 性能指标收集器
/// 
/// 负责收集和聚合各种性能指标
pub struct MetricsCollector {
    /// 指标存储
    metrics: Arc<Mutex<HashMap<String, MetricValue>>>,
    
    /// 收集器配置
    config: CollectorConfig,
    
    /// 指标处理器
    processors: Vec<Box<dyn MetricProcessor>>,
}

impl MetricsCollector {
    /// 记录指标值
    /// 
    /// # 参数
    /// * `name` - 指标名称
    /// * `value` - 指标值
    /// * `tags` - 指标标签
    pub fn record_metric(&self, name: &str, value: MetricValue, tags: HashMap<String, String>) {
        let key = self.create_metric_key(name, &tags);
        
        if let Ok(mut metrics) = self.metrics.lock() {
            metrics.insert(key, value);
        }
        
        // 通知所有处理器
        for processor in &self.processors {
            processor.process_metric(name, &value, &tags);
        }
    }
    
    /// 记录时间指标
    /// 
    /// # 参数
    /// * `name` - 指标名称
    /// * `duration` - 持续时间
    pub fn record_timing(&self, name: &str, duration: Duration) {
        self.record_metric(name, MetricValue::Timing(duration), HashMap::new());
    }
    
    /// 记录计数指标
    /// 
    /// # 参数
    /// * `name` - 指标名称
    /// * `count` - 计数值
    pub fn record_count(&self, name: &str, count: i64) {
        self.record_metric(name, MetricValue::Count(count), HashMap::new());
    }
    
    /// 获取指标值
    /// 
    /// # 参数
    /// * `name` - 指标名称
    /// * `tags` - 指标标签
    /// 
    /// # 返回值
    /// 指标值，如果不存在返回None
    pub fn get_metric(&self, name: &str, tags: &HashMap<String, String>) -> Option<MetricValue> {
        let key = self.create_metric_key(name, tags);
        
        if let Ok(metrics) = self.metrics.lock() {
            metrics.get(&key).cloned()
        } else {
            None
        }
    }
    
    /// 获取所有指标
    /// 
    /// # 返回值
    /// 所有指标的副本
    pub fn get_all_metrics(&self) -> HashMap<String, MetricValue> {
        if let Ok(metrics) = self.metrics.lock() {
            metrics.clone()
        } else {
            HashMap::new()
        }
    }
    
    /// 创建指标键
    /// 
    /// 私有方法，用于生成唯一的指标键
    fn create_metric_key(&self, name: &str, tags: &HashMap<String, String>) -> String {
        let mut key = name.to_string();
        
        if !tags.is_empty() {
            let mut sorted_tags: Vec<_> = tags.iter().collect();
            sorted_tags.sort_by_key(|(k, _)| *k);
            
            let tag_string = sorted_tags
                .iter()
                .map(|(k, v)| format!("{}={}", k, v))
                .collect::<Vec<_>>()
                .join(",");
            
            key.push_str(&format!("{{{}}}", tag_string));
        }
        
        key
    }
}

/// 性能监控宏
/// 
/// 自动记录函数执行时间
#[macro_export]
macro_rules! monitor_performance {
    ($metrics_collector:expr, $operation_name:expr, $code:block) => {{
        let start = std::time::Instant::now();
        let result = $code;
        let duration = start.elapsed();
        
        $metrics_collector.record_timing($operation_name, duration);
        
        result
    }};
}
```

---

**本文档将持续更新，提供更多代码示例和详细解释。**
