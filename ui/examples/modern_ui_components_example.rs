//! 现代化UI组件库示例 - 展示Rust 1.90特性在UI开发中的应用
//! 
//! 本示例展示了如何使用Rust 1.90的新特性构建现代化的UI组件库，
//! 包括响应式状态管理、组件组合、性能优化等特性。

use std::collections::HashMap;
use std::cell::Cell;
use std::rc::Rc;
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use anyhow::Result;

/// 现代化UI组件库
pub struct ModernUIComponents {
    components: HashMap<String, Box<dyn Component>>,
    state_manager: Arc<StateManager>,
    event_bus: Arc<EventBus>,
}

/// 组件trait
pub trait Component: Send + Sync {
    fn render(&self, props: &ComponentProps) -> Result<ComponentElement>;
    fn update(&mut self, props: &ComponentProps, state: &mut ComponentState) -> Result<()>;
    fn get_id(&self) -> &str;
}

/// 组件属性
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentProps {
    pub id: String,
    pub class_name: Option<String>,
    pub style: Option<HashMap<String, String>>,
    pub children: Vec<ComponentElement>,
    pub data: HashMap<String, serde_json::Value>,
}

/// 组件状态
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentState {
    pub is_visible: bool,
    pub is_enabled: bool,
    pub data: HashMap<String, serde_json::Value>,
    pub last_updated: chrono::DateTime<chrono::Utc>,
}

/// 组件元素
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentElement {
    pub tag: String,
    pub attributes: HashMap<String, String>,
    pub content: String,
    pub children: Vec<ComponentElement>,
}

/// 状态管理器 - 使用Rust 1.90的Cell::update
pub struct StateManager {
    global_state: Cell<GlobalState>,
    component_states: HashMap<String, Cell<ComponentState>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalState {
    pub theme: Theme,
    pub language: String,
    pub user_preferences: HashMap<String, serde_json::Value>,
    pub last_updated: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Theme {
    Light,
    Dark,
    Auto,
}

impl StateManager {
    pub fn new() -> Self {
        Self {
            global_state: Cell::new(GlobalState {
                theme: Theme::Light,
                language: "zh-CN".to_string(),
                user_preferences: HashMap::new(),
                last_updated: chrono::Utc::now(),
            }),
            component_states: HashMap::new(),
        }
    }
    
    /// 使用Cell::update进行原子状态更新
    pub fn update_global_state<F>(&self, updater: F) -> Result<GlobalState>
    where
        F: FnOnce(&GlobalState) -> Result<GlobalState>,
    {
        let new_state = self.global_state.update(|current_state| {
            updater(current_state)
        })?;
        
        Ok(new_state)
    }
    
    /// 更新组件状态
    pub fn update_component_state<F>(&self, component_id: &str, updater: F) -> Result<ComponentState>
    where
        F: FnOnce(&ComponentState) -> Result<ComponentState>,
    {
        let component_state = self.component_states
            .get(component_id)
            .ok_or_else(|| anyhow::anyhow!("组件状态不存在: {}", component_id))?;
        
        let new_state = component_state.update(|current_state| {
            updater(current_state)
        })?;
        
        Ok(new_state)
    }
    
    /// 获取全局状态
    pub fn get_global_state(&self) -> GlobalState {
        self.global_state.get()
    }
    
    /// 获取组件状态
    pub fn get_component_state(&self, component_id: &str) -> Option<ComponentState> {
        self.component_states.get(component_id).map(|cell| cell.get())
    }
}

/// 事件总线 - 使用Rust 1.90的增强模式匹配
pub struct EventBus {
    handlers: HashMap<String, Vec<Box<dyn EventHandler>>>,
    event_queue: tokio::sync::mpsc::UnboundedSender<UIEvent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UIEvent {
    ComponentMounted { component_id: String },
    ComponentUnmounted { component_id: String },
    StateChanged { component_id: String, state: ComponentState },
    ThemeChanged { theme: Theme },
    UserInteraction { component_id: String, action: String, data: HashMap<String, serde_json::Value> },
}

pub trait EventHandler: Send + Sync {
    fn handle(&self, event: &UIEvent) -> Result<()>;
    fn can_handle(&self, event_type: &str) -> bool;
}

impl EventBus {
    pub fn new() -> Self {
        let (sender, mut receiver) = tokio::sync::mpsc::unbounded_channel();
        
        // 启动事件处理循环
        let handlers = Arc::new(std::sync::RwLock::new(HashMap::new()));
        let handlers_clone = handlers.clone();
        
        tokio::spawn(async move {
            while let Some(event) = receiver.recv().await {
                let event_type = match &event {
                    UIEvent::ComponentMounted { .. } => "component.mounted",
                    UIEvent::ComponentUnmounted { .. } => "component.unmounted",
                    UIEvent::StateChanged { .. } => "state.changed",
                    UIEvent::ThemeChanged { .. } => "theme.changed",
                    UIEvent::UserInteraction { .. } => "user.interaction",
                };
                
                let handlers = {
                    let handlers = handlers_clone.read().unwrap();
                    handlers.get(event_type).cloned()
                };
                
                if let Some(handlers) = handlers {
                    for handler in handlers {
                        if let Err(e) = handler.handle(&event) {
                            eprintln!("事件处理失败: {}", e);
                        }
                    }
                }
            }
        });
        
        Self {
            handlers: HashMap::new(),
            event_queue: sender,
        }
    }
    
    /// 注册事件处理器
    pub fn register_handler(&mut self, event_type: String, handler: Box<dyn EventHandler>) {
        self.handlers.entry(event_type).or_insert_with(Vec::new).push(handler);
    }
    
    /// 发布事件
    pub fn publish(&self, event: UIEvent) -> Result<()> {
        self.event_queue.send(event)?;
        Ok(())
    }
}

/// 按钮组件
pub struct ButtonComponent {
    id: String,
    text: String,
    variant: ButtonVariant,
    size: ButtonSize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ButtonVariant {
    Primary,
    Secondary,
    Danger,
    Ghost,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ButtonSize {
    Small,
    Medium,
    Large,
}

impl ButtonComponent {
    pub fn new(id: String, text: String) -> Self {
        Self {
            id,
            text,
            variant: ButtonVariant::Primary,
            size: ButtonSize::Medium,
        }
    }
    
    pub fn with_variant(mut self, variant: ButtonVariant) -> Self {
        self.variant = variant;
        self
    }
    
    pub fn with_size(mut self, size: ButtonSize) -> Self {
        self.size = size;
        self
    }
}

impl Component for ButtonComponent {
    fn render(&self, props: &ComponentProps) -> Result<ComponentElement> {
        let mut attributes = HashMap::new();
        attributes.insert("id".to_string(), self.id.clone());
        attributes.insert("class".to_string(), format!("btn btn-{:?} btn-{:?}", self.variant, self.size));
        
        if let Some(class_name) = &props.class_name {
            attributes.insert("class".to_string(), 
                format!("{} {}", attributes.get("class").unwrap(), class_name));
        }
        
        if let Some(style) = &props.style {
            let style_str = style.iter()
                .map(|(k, v)| format!("{}: {}", k, v))
                .collect::<Vec<_>>()
                .join("; ");
            attributes.insert("style".to_string(), style_str);
        }
        
        Ok(ComponentElement {
            tag: "button".to_string(),
            attributes,
            content: self.text.clone(),
            children: Vec::new(),
        })
    }
    
    fn update(&mut self, props: &ComponentProps, state: &mut ComponentState) -> Result<()> {
        if let Some(data) = props.data.get("text") {
            if let Some(text) = data.as_str() {
                self.text = text.to_string();
            }
        }
        
        if let Some(data) = props.data.get("variant") {
            if let Some(variant_str) = data.as_str() {
                self.variant = match variant_str {
                    "primary" => ButtonVariant::Primary,
                    "secondary" => ButtonVariant::Secondary,
                    "danger" => ButtonVariant::Danger,
                    "ghost" => ButtonVariant::Ghost,
                    _ => ButtonVariant::Primary,
                };
            }
        }
        
        state.last_updated = chrono::Utc::now();
        Ok(())
    }
    
    fn get_id(&self) -> &str {
        &self.id
    }
}

/// 输入框组件
pub struct InputComponent {
    id: String,
    placeholder: String,
    input_type: InputType,
    value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InputType {
    Text,
    Password,
    Email,
    Number,
}

impl InputComponent {
    pub fn new(id: String, placeholder: String) -> Self {
        Self {
            id,
            placeholder,
            input_type: InputType::Text,
            value: String::new(),
        }
    }
    
    pub fn with_type(mut self, input_type: InputType) -> Self {
        self.input_type = input_type;
        self
    }
}

impl Component for InputComponent {
    fn render(&self, props: &ComponentProps) -> Result<ComponentElement> {
        let mut attributes = HashMap::new();
        attributes.insert("id".to_string(), self.id.clone());
        attributes.insert("type".to_string(), format!("{:?}", self.input_type).to_lowercase());
        attributes.insert("placeholder".to_string(), self.placeholder.clone());
        attributes.insert("value".to_string(), self.value.clone());
        attributes.insert("class".to_string(), "form-input".to_string());
        
        Ok(ComponentElement {
            tag: "input".to_string(),
            attributes,
            content: String::new(),
            children: Vec::new(),
        })
    }
    
    fn update(&mut self, props: &ComponentProps, state: &mut ComponentState) -> Result<()> {
        if let Some(data) = props.data.get("value") {
            if let Some(value) = data.as_str() {
                self.value = value.to_string();
            }
        }
        
        state.last_updated = chrono::Utc::now();
        Ok(())
    }
    
    fn get_id(&self) -> &str {
        &self.id
    }
}

/// 卡片组件
pub struct CardComponent {
    id: String,
    title: String,
    content: String,
    footer: Option<String>,
}

impl CardComponent {
    pub fn new(id: String, title: String, content: String) -> Self {
        Self {
            id,
            title,
            content,
            footer: None,
        }
    }
    
    pub fn with_footer(mut self, footer: String) -> Self {
        self.footer = Some(footer);
        self
    }
}

impl Component for CardComponent {
    fn render(&self, props: &ComponentProps) -> Result<ComponentElement> {
        let mut attributes = HashMap::new();
        attributes.insert("id".to_string(), self.id.clone());
        attributes.insert("class".to_string(), "card".to_string());
        
        let mut children = Vec::new();
        
        // 标题
        children.push(ComponentElement {
            tag: "div".to_string(),
            attributes: [("class".to_string(), "card-header".to_string())].iter().cloned().collect(),
            content: self.title.clone(),
            children: Vec::new(),
        });
        
        // 内容
        children.push(ComponentElement {
            tag: "div".to_string(),
            attributes: [("class".to_string(), "card-body".to_string())].iter().cloned().collect(),
            content: self.content.clone(),
            children: Vec::new(),
        });
        
        // 页脚
        if let Some(footer) = &self.footer {
            children.push(ComponentElement {
                tag: "div".to_string(),
                attributes: [("class".to_string(), "card-footer".to_string())].iter().cloned().collect(),
                content: footer.clone(),
                children: Vec::new(),
            });
        }
        
        Ok(ComponentElement {
            tag: "div".to_string(),
            attributes,
            content: String::new(),
            children,
        })
    }
    
    fn update(&mut self, props: &ComponentProps, state: &mut ComponentState) -> Result<()> {
        if let Some(data) = props.data.get("title") {
            if let Some(title) = data.as_str() {
                self.title = title.to_string();
            }
        }
        
        if let Some(data) = props.data.get("content") {
            if let Some(content) = data.as_str() {
                self.content = content.to_string();
            }
        }
        
        state.last_updated = chrono::Utc::now();
        Ok(())
    }
    
    fn get_id(&self) -> &str {
        &self.id
    }
}

impl ModernUIComponents {
    pub fn new() -> Self {
        Self {
            components: HashMap::new(),
            state_manager: Arc::new(StateManager::new()),
            event_bus: Arc::new(EventBus::new()),
        }
    }
    
    /// 注册组件
    pub fn register_component(&mut self, component: Box<dyn Component>) {
        let id = component.get_id().to_string();
        self.components.insert(id, component);
    }
    
    /// 渲染组件
    pub fn render_component(&self, component_id: &str, props: &ComponentProps) -> Result<ComponentElement> {
        let component = self.components
            .get(component_id)
            .ok_or_else(|| anyhow::anyhow!("组件不存在: {}", component_id))?;
        
        component.render(props)
    }
    
    /// 更新组件
    pub fn update_component(&mut self, component_id: &str, props: &ComponentProps) -> Result<()> {
        let component = self.components
            .get_mut(component_id)
            .ok_or_else(|| anyhow::anyhow!("组件不存在: {}", component_id))?;
        
        let mut state = self.state_manager
            .get_component_state(component_id)
            .unwrap_or_else(|| ComponentState {
                is_visible: true,
                is_enabled: true,
                data: HashMap::new(),
                last_updated: chrono::Utc::now(),
            });
        
        component.update(props, &mut state)?;
        
        // 发布状态变更事件
        self.event_bus.publish(UIEvent::StateChanged {
            component_id: component_id.to_string(),
            state,
        })?;
        
        Ok(())
    }
    
    /// 获取状态管理器
    pub fn get_state_manager(&self) -> Arc<StateManager> {
        self.state_manager.clone()
    }
    
    /// 获取事件总线
    pub fn get_event_bus(&self) -> Arc<EventBus> {
        self.event_bus.clone()
    }
}

/// 主函数 - 演示现代化UI组件库
#[tokio::main]
async fn main() -> Result<()> {
    println!("🎨 现代化UI组件库演示");
    
    // 创建组件库实例
    let mut ui_lib = ModernUIComponents::new();
    
    // 注册组件
    let button = ButtonComponent::new("btn1".to_string(), "点击我".to_string())
        .with_variant(ButtonVariant::Primary)
        .with_size(ButtonSize::Large);
    
    let input = InputComponent::new("input1".to_string(), "请输入内容".to_string())
        .with_type(InputType::Text);
    
    let card = CardComponent::new(
        "card1".to_string(),
        "示例卡片".to_string(),
        "这是一个示例卡片的内容".to_string(),
    ).with_footer("卡片页脚".to_string());
    
    ui_lib.register_component(Box::new(button));
    ui_lib.register_component(Box::new(input));
    ui_lib.register_component(Box::new(card));
    
    // 渲染组件
    println!("\n🎯 渲染组件:");
    
    let button_props = ComponentProps {
        id: "btn1".to_string(),
        class_name: Some("custom-button".to_string()),
        style: Some([("color".to_string(), "white".to_string())].iter().cloned().collect()),
        children: Vec::new(),
        data: HashMap::new(),
    };
    
    let button_element = ui_lib.render_component("btn1", &button_props)?;
    println!("按钮组件: {:?}", button_element);
    
    let input_props = ComponentProps {
        id: "input1".to_string(),
        class_name: None,
        style: None,
        children: Vec::new(),
        data: HashMap::new(),
    };
    
    let input_element = ui_lib.render_component("input1", &input_props)?;
    println!("输入框组件: {:?}", input_element);
    
    let card_props = ComponentProps {
        id: "card1".to_string(),
        class_name: None,
        style: None,
        children: Vec::new(),
        data: HashMap::new(),
    };
    
    let card_element = ui_lib.render_component("card1", &card_props)?;
    println!("卡片组件: {:?}", card_element);
    
    // 演示状态管理
    println!("\n📊 演示状态管理:");
    
    let state_manager = ui_lib.get_state_manager();
    let new_state = state_manager.update_global_state(|state| {
        Ok(GlobalState {
            theme: Theme::Dark,
            language: state.language.clone(),
            user_preferences: state.user_preferences.clone(),
            last_updated: chrono::Utc::now(),
        })
    })?;
    
    println!("全局状态已更新: {:?}", new_state);
    
    // 演示事件处理
    println!("\n🎪 演示事件处理:");
    
    let event_bus = ui_lib.get_event_bus();
    event_bus.publish(UIEvent::ComponentMounted {
        component_id: "btn1".to_string(),
    })?;
    
    event_bus.publish(UIEvent::UserInteraction {
        component_id: "btn1".to_string(),
        action: "click".to_string(),
        data: [("timestamp".to_string(), serde_json::Value::String(chrono::Utc::now().to_rfc3339()))].iter().cloned().collect(),
    })?;
    
    println!("事件已发布");
    
    // 演示组件更新
    println!("\n🔄 演示组件更新:");
    
    let mut update_props = button_props.clone();
    update_props.data.insert("text".to_string(), serde_json::Value::String("新文本".to_string()));
    update_props.data.insert("variant".to_string(), serde_json::Value::String("danger".to_string()));
    
    ui_lib.update_component("btn1", &update_props)?;
    
    let updated_element = ui_lib.render_component("btn1", &update_props)?;
    println!("更新后的按钮组件: {:?}", updated_element);
    
    println!("\n✅ 现代化UI组件库演示完成!");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_button_component() {
        let button = ButtonComponent::new("test_btn".to_string(), "测试按钮".to_string());
        let props = ComponentProps {
            id: "test_btn".to_string(),
            class_name: None,
            style: None,
            children: Vec::new(),
            data: HashMap::new(),
        };
        
        let element = button.render(&props).unwrap();
        assert_eq!(element.tag, "button");
        assert_eq!(element.content, "测试按钮");
    }
    
    #[test]
    fn test_input_component() {
        let input = InputComponent::new("test_input".to_string(), "测试输入".to_string());
        let props = ComponentProps {
            id: "test_input".to_string(),
            class_name: None,
            style: None,
            children: Vec::new(),
            data: HashMap::new(),
        };
        
        let element = input.render(&props).unwrap();
        assert_eq!(element.tag, "input");
        assert_eq!(element.attributes.get("placeholder"), Some(&"测试输入".to_string()));
    }
    
    #[test]
    fn test_card_component() {
        let card = CardComponent::new(
            "test_card".to_string(),
            "测试卡片".to_string(),
            "测试内容".to_string(),
        );
        
        let props = ComponentProps {
            id: "test_card".to_string(),
            class_name: None,
            style: None,
            children: Vec::new(),
            data: HashMap::new(),
        };
        
        let element = card.render(&props).unwrap();
        assert_eq!(element.tag, "div");
        assert_eq!(element.children.len(), 2); // 标题和内容
    }
    
    #[test]
    fn test_state_manager() {
        let state_manager = StateManager::new();
        
        let new_state = state_manager.update_global_state(|state| {
            Ok(GlobalState {
                theme: Theme::Dark,
                language: state.language.clone(),
                user_preferences: state.user_preferences.clone(),
                last_updated: chrono::Utc::now(),
            })
        }).unwrap();
        
        assert_eq!(new_state.theme, Theme::Dark);
    }
    
    #[tokio::test]
    async fn test_event_bus() {
        let mut event_bus = EventBus::new();
        
        let event = UIEvent::ComponentMounted {
            component_id: "test_component".to_string(),
        };
        
        let result = event_bus.publish(event);
        assert!(result.is_ok());
    }
}
