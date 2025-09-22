//! Iced 0.12 跨平台GUI示例 - 基于Rust 1.90优化
//! 
//! Iced是一个受Elm启发的现代化GUI框架，提供：
//! - 声明式编程模型
//! - 跨平台支持 (Windows, macOS, Linux)
//! - 数据驱动的应用开发
//! - Rust 1.90新特性集成

use iced::{
    widget::{button, column, container, row, text, text_input, Column, Row},
    Alignment, Application, Command, Element, Length, Settings, Theme,
};

#[derive(Debug, Clone)]
enum Message {
    InputChanged(String),
    AddTodo,
    DeleteTodo(usize),
    ToggleTodo(usize),
    ThemeChanged,
}

#[derive(Debug, Clone)]
struct TodoItem {
    id: usize,
    text: String,
    completed: bool,
}

struct TodoApp {
    todos: Vec<TodoItem>,
    input_value: String,
    next_id: usize,
    theme: Theme,
}

impl Application for TodoApp {
    type Message = Message;
    type Theme = Theme;
    type Executor = iced::executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            Self {
                todos: vec![],
                input_value: String::new(),
                next_id: 0,
                theme: Theme::Light,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Iced 0.12 - Rust 1.90 跨平台示例")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::InputChanged(value) => {
                self.input_value = value;
            }
            Message::AddTodo => {
                if !self.input_value.trim().is_empty() {
                    let todo = TodoItem {
                        id: self.next_id,
                        text: self.input_value.clone(),
                        completed: false,
                    };
                    self.todos.push(todo);
                    self.next_id += 1;
                    self.input_value.clear();
                }
            }
            Message::DeleteTodo(id) => {
                self.todos.retain(|todo| todo.id != id);
            }
            Message::ToggleTodo(id) => {
                if let Some(todo) = self.todos.iter_mut().find(|todo| todo.id == id) {
                    todo.completed = !todo.completed;
                }
            }
            Message::ThemeChanged => {
                self.theme = match self.theme {
                    Theme::Light => Theme::Dark,
                    Theme::Dark => Theme::Auto,
                    Theme::Auto => Theme::Light,
                    _ => Theme::Light,
                };
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let title = text("🦀 Iced 0.12 跨平台GUI示例")
            .size(32)
            .horizontal_alignment(iced::alignment::Horizontal::Center);

        let subtitle = text("基于Rust 1.90优化的现代化GUI框架")
            .size(16)
            .horizontal_alignment(iced::alignment::Horizontal::Center);

        let input_row = row![
            text_input("输入新的待办事项...", &self.input_value)
                .on_input(Message::InputChanged)
                .on_submit(Message::AddTodo)
                .padding(10)
                .size(16),
            button("添加")
                .on_press(Message::AddTodo)
                .padding(10)
                .style(iced::theme::Button::Primary),
        ]
        .spacing(10)
        .align_items(Alignment::Center);

        let todo_list = Column::with_children(
            self.todos
                .iter()
                .map(|todo| {
                    let status_text = if todo.completed { "✅" } else { "⏳" };
                    let text_style = if todo.completed {
                        iced::theme::Text::Color(iced::Color::from_rgb(0.5, 0.5, 0.5))
                    } else {
                        iced::theme::Text::Default
                    };

                    row![
                        button(status_text)
                            .on_press(Message::ToggleTodo(todo.id))
                            .style(iced::theme::Button::Secondary),
                        text(&todo.text)
                            .style(text_style)
                            .width(Length::Fill),
                        button("删除")
                            .on_press(Message::DeleteTodo(todo.id))
                            .style(iced::theme::Button::Destructive),
                    ]
                    .spacing(10)
                    .align_items(Alignment::Center)
                    .into()
                })
                .collect(),
        )
        .spacing(5);

        let stats = text(format!(
            "总计: {} 项 | 已完成: {} 项 | 待完成: {} 项",
            self.todos.len(),
            self.todos.iter().filter(|t| t.completed).count(),
            self.todos.iter().filter(|t| !t.completed).count()
        ))
        .size(14)
        .horizontal_alignment(iced::alignment::Horizontal::Center);

        let theme_button = button(format!("主题: {:?}", self.theme))
            .on_press(Message::ThemeChanged)
            .padding(8)
            .style(iced::theme::Button::Secondary);

        let content = column![
            title,
            subtitle,
            input_row,
            todo_list,
            stats,
            theme_button,
        ]
        .spacing(20)
        .padding(20)
        .align_items(Alignment::Center);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }

    fn theme(&self) -> Theme {
        self.theme
    }
}

fn main() -> iced::Result {
    TodoApp::run(Settings {
        window: iced::window::Settings {
            size: iced::Size::new(600.0, 700.0),
            resizable: true,
            decorations: true,
            ..Default::default()
        },
        ..Default::default()
    })
}
