use iced::alignment::{Horizontal, Vertical};
use iced::widget::{Button, Container, Row, Text};
use iced::{executor, Alignment, Application, Command, Element, Length, Settings, Subscription, Theme};

extern crate chrono;

pub fn main() -> iced::Result {
  HelloWorld::run(Settings::default())
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
  UpdateTime,
  TogglePauseTimer,
}

struct HelloWorld {
  current_time: chrono::DateTime<chrono::Local>,
  is_timer_paused: bool,
}

impl Application for HelloWorld {
  type Executor = executor::Default;
  type Message = Message;
  type Theme = Theme;
  type Flags = ();

  fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
    (
      HelloWorld {
        current_time: chrono::Local::now(),
        is_timer_paused: false,
      },
      Command::none(),
    )
  }

  fn title(&self) -> String {
    String::from("Basic State 2")
  }

  fn update(&mut self, message: Message) -> Command<Self::Message> {
    match message {
      Message::UpdateTime => {
        self.current_time = chrono::Local::now();
      }
      Message::TogglePauseTimer => self.is_timer_paused = !self.is_timer_paused,
    }

    Command::none()
  }

  fn view(&self) -> Element<Message> {
    // Remember .into() will wrap a built-in Widget inside of an Element.
    let timestamp = Text::new(
      self
        .current_time
        .format("%Y-%m-%d %H:%M:%S%.3f")
        .to_string(),
    )
    .into();

    let update = Button::new("Toggle")
      .on_press(Message::TogglePauseTimer)
      .into();

    let row = Row::with_children(vec![timestamp, update])
      .spacing(20)
      .align_items(Alignment::Center);

    Container::new(row)
      .align_x(Horizontal::Center)
      .align_y(Vertical::Center)
      .width(Length::Fill)
      .height(Length::Fill)
      .into()
  }

  fn theme(&self) -> Theme {
    Theme::default()
  }

  fn subscription(&self) -> Subscription<Self::Message> {
    if self.is_timer_paused {
      return Subscription::none();
    }

    iced::time::every(std::time::Duration::from_millis(17)).map(|_| Message::UpdateTime)
  }
}
