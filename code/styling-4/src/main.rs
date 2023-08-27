use iced::widget::button::Appearance;
use iced::widget::{button, Button};
use iced::{executor, Application, Background, BorderRadius, Color, Command, Element, Settings};

pub fn main() -> iced::Result {
  HelloWorld::run(Settings::default())
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
  Pressed,
}

struct HelloWorld {}

struct CustomTheme {}

impl iced::application::StyleSheet for CustomTheme {
  type Style = ();

  fn appearance(&self, _style: &Self::Style) -> iced::application::Appearance {
    iced::application::Appearance {
      background_color: Default::default(),
      text_color: Default::default(),
    }
  }
}

impl iced::widget::text::StyleSheet for CustomTheme {
  type Style = ();

  fn appearance(&self, _style: Self::Style) -> iced::widget::text::Appearance {
    iced::widget::text::Appearance {
      color: Some(Color::new(0.0, 0.0, 0.0, 1.0)),
    }
  }
}

impl button::StyleSheet for CustomTheme {
  type Style = ();

  fn active(&self, _style: &Self::Style) -> Appearance {
    Appearance {
      shadow_offset: Default::default(),
      background: Some(Background::Color(Color::new(0.3, 0.4, 0.94, 1.0))),
      border_radius: BorderRadius::from(0.0),
      border_width: 0.0,
      border_color: Default::default(),
      text_color: Color::new(0.0, 0.0, 0.0, 1.0),
    }
  }
}

impl Default for CustomTheme {
  fn default() -> Self {
    CustomTheme {}
  }
}

impl Application for HelloWorld {
  type Executor = executor::Default;
  type Message = Message;
  type Theme = CustomTheme;
  type Flags = ();

  fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
    (HelloWorld {}, Command::none())
  }

  fn title(&self) -> String {
    String::from("Styling 4")
  }

  fn update(&mut self, _message: Message) -> Command<Self::Message> {
    Command::none()
  }

  fn view(&self) -> Element<Message, iced::Renderer<Self::Theme>> {
    Element::new(Button::new("Test").on_press(Message::Pressed).style(()))
  }

  fn theme(&self) -> Self::Theme {
    Self::Theme {}
  }
}
