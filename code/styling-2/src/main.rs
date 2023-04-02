use iced::{executor, Application, Command, Element, Settings};

pub fn main() -> iced::Result {
  HelloWorld::run(Settings::default())
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
  Pressed,
}

struct HelloWorld {}

impl Application for HelloWorld {
  type Executor = executor::Default;
  type Message = Message;
  type Theme = iced::Theme;
  type Flags = ();

  fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
    (HelloWorld {}, Command::none())
  }

  fn title(&self) -> String {
    String::from("Styling 2")
  }

  fn update(&mut self, _message: Message) -> Command<Self::Message> {
    Command::none()
  }

  fn view(&self) -> Element<Message, iced::Renderer<Self::Theme>> {
    Element::<Message, iced::Renderer<Self::Theme>>::new(
      iced_native::widget::Button::<Message, iced::Renderer<Self::Theme>>::new("Test").on_press(Message::Pressed).style(
        // style: <Theme as iced::widget::button::StyleSheet>::Style
      )
    )
  }

  fn theme(&self) -> Self::Theme {
    Self::Theme::default()
  }
}