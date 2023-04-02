use iced::widget::button::{Appearance, StyleSheet};
use iced::widget::Button;
use iced::{executor, Application, Color, Command, Element, Settings};

pub fn main() -> iced::Result {
  HelloWorld::run(Settings::default())
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
  Pressed,
}

struct HelloWorld {}

struct CustomButtonStylesheet {}

impl StyleSheet for CustomButtonStylesheet {
  type Style = iced::Theme;

  fn active(&self, style: &Self::Style) -> Appearance {
    Appearance {
      shadow_offset: Default::default(),
      background: None,
      border_radius: 0.0,
      border_width: 0.0,
      border_color: Default::default(),
      text_color: Color::new(1.0, 0.0, 0.0, 1.0),
    }
  }
}

impl Application for HelloWorld {
  type Executor = executor::Default;
  type Message = Message;
  type Theme = iced::Theme;
  type Flags = ();

  fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
    (HelloWorld {}, Command::none())
  }

  fn title(&self) -> String {
    String::from("Styling 3")
  }

  fn update(&mut self, _message: Message) -> Command<Self::Message> {
    Command::none()
  }

  fn view(&self) -> Element<Message> {
    Element::new(
      Button::new("Test")
        .on_press(Message::Pressed)
        .style(iced::theme::Button::Custom(Box::new(
          CustomButtonStylesheet {},
        ))),
    )
  }

  fn theme(&self) -> Self::Theme {
    Self::Theme::default()
  }
}
