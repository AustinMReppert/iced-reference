use std::thread;
use iced::widget::{Button, Text};
use iced::{executor, Application, Command, Element, Settings, Theme};

pub fn button_example() -> iced::Result {
  HelloWorld::run(Settings::default())
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
  foo,
}

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

  fn update(&mut self, message: Message) -> Command<Self::Message> {
    match message {
      Message::foo => {
        thread::spawn(|| HelloWorld::run(Settings::default()));
        println!("Hello World!");
      }
    }
    Command::none()
  }

  fn view(&self) -> Element<Message> {
    Button::new("Hello World!")
      .on_press(Message::foo).into()
  }

  fn theme(&self) -> Theme {
    Theme::default()
  }
}
