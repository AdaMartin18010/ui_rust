//! Iced 0.13 简化GUI示例 - 基于Rust 1.90优化
//! 
//! 这是一个简化的Iced示例，展示基本的GUI功能
//! 由于Iced 0.13 API变化较大，这里提供一个基础版本

use iced::{
    widget::{button, column, container, text, Column},
    Alignment, Element, Length, Settings, Theme,
};

#[derive(Debug, Clone)]
enum Message {
    ButtonPressed,
    ThemeChanged,
}

struct SimpleApp {
    counter: i32,
    theme: Theme,
}

impl Default for SimpleApp {
    fn default() -> Self {
        Self {
            counter: 0,
            theme: Theme::Light,
        }
    }
}

impl iced::Application for SimpleApp {
    type Message = Message;
    type Theme = Theme;
    type Executor = iced::executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Self, iced::Command<Message>) {
        (Self::default(), iced::Command::none())
    }

    fn title(&self) -> String {
        "Iced 0.13 简化示例".to_string()
    }

    fn update(&mut self, message: Message) -> iced::Command<Message> {
        match message {
            Message::ButtonPressed => {
                self.counter += 1;
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
        iced::Command::none()
    }

    fn view(&self) -> Element<Message> {
        let title = text("🦀 Iced 0.13 简化GUI示例")
            .size(32)
            .horizontal_alignment(iced::alignment::Horizontal::Center);

        let counter_text = text(format!("计数器: {}", self.counter))
            .size(24)
            .horizontal_alignment(iced::alignment::Horizontal::Center);

        let increment_button = button("点击增加")
            .on_press(Message::ButtonPressed)
            .padding(10)
            .style(iced::widget::Button::Primary);

        let theme_button = button(format!("切换主题: {:?}", self.theme))
            .on_press(Message::ThemeChanged)
            .padding(8)
            .style(iced::widget::Button::Secondary);

        let info_text = text("这是一个简化的Iced示例，展示基本的GUI功能")
            .size(14)
            .horizontal_alignment(iced::alignment::Horizontal::Center);

        let content = column![
            title,
            counter_text,
            increment_button,
            theme_button,
            info_text,
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
    SimpleApp::run(Settings::default())
}