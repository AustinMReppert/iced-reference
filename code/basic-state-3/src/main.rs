use iced::alignment::{Horizontal, Vertical};
use iced::widget::{Button, Column, Container, Text};
use iced::{executor, Alignment, Application, Command, Element, Length, Settings, Theme};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy)]
pub enum Error {
  APIError,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RandomQuoteApiResponse {
  content: String,
}

pub fn main() -> iced::Result {
  HelloWorld::run(Settings::default())
}

#[derive(Debug, Clone)]
pub enum Message {
  GetRandomQuote,
  GetRandomQuoteDone(Result<RandomQuoteApiResponse, Error>),
}

struct HelloWorld {
  quote: String,
}

impl Application for HelloWorld {
  type Executor = executor::Default;
  type Message = Message;
  type Theme = Theme;
  type Flags = ();

  fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
    (
      HelloWorld {
        quote: "???".to_string(),
      },
      Command::none(),
    )
  }

  fn title(&self) -> String {
    String::from("Basic State 3")
  }

  fn update(&mut self, message: Message) -> Command<Self::Message> {
    match message {
      Message::GetRandomQuote => Command::perform(get_random_quote(), Message::GetRandomQuoteDone),
      Message::GetRandomQuoteDone(res) => {
        match res {
          Ok(random_quote_api_response) => {
            self.quote = random_quote_api_response.content;
          }
          Err(_) => {}
        }

        Command::none()
      }
    }
  }

  fn view(&self) -> Element<Message> {
    // Remember .into() will wrap a built-in Widget inside of an Element.
    let quote = Text::new(format!("{}", self.quote)).into();

    let update = Button::new("Random Quote")
      .on_press(Message::GetRandomQuote)
      .into();

    let column = Column::with_children(vec![quote, update])
      .spacing(20)
      .align_items(Alignment::Center);

    Container::new(column)
      .align_x(Horizontal::Center)
      .align_y(Vertical::Center)
      .width(Length::Fill)
      .height(Length::Fill)
      .into()
  }

  fn theme(&self) -> Theme {
    Theme::default()
  }
}

async fn get_random_quote() -> Result<RandomQuoteApiResponse, Error> {
  Ok(
    reqwest::get("https://api.quotable.io/random")
      .await?
      .json::<RandomQuoteApiResponse>()
      .await?,
  )
}

impl From<reqwest::Error> for Error {
  fn from(_error: reqwest::Error) -> Error {
    Error::APIError
  }
}