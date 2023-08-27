use iced::widget::Text;
use iced::{executor, Application, Command, Element, Settings, Theme};

pub fn main() -> iced::Result {
  HelloWorld::run(Settings::default())
}

#[derive(Debug, Clone, Copy)]
pub enum Message {}

struct HelloWorld {}

impl Application for HelloWorld {
  type Executor = executor::Default;
  type Message = Message;
  type Theme = Theme;
  type Flags = ();

  fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
    (HelloWorld {}, Command::none())
  }

  fn title(&self) -> String {
    String::from("Hello World!")
  }

  fn update(&mut self, _message: Message) -> Command<Self::Message> {
    Command::none()
  }

  fn view(&self) -> Element<Message> {
    Element::new(Text::new("Hello World!"))
  }

  fn theme(&self) -> Theme {
    Theme::default()
  }
}
