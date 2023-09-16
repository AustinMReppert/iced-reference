mod utils;

use iced::widget::{Column, Container, Row, Rule, Scrollable, Text};
use iced::{executor, Application, Command, Element, Settings, Theme, Color};
use iced::alignment::{Horizontal, Vertical};
use iced::widget::container::Id;

pub fn main() -> iced::Result {
  ContainerExample::run(Settings::default())
}

#[derive(Debug, Clone)]
pub enum Message {
  Open(String),
}

struct ContainerExample {}

impl Application for ContainerExample {
  type Executor = executor::Default;
  type Message = Message;
  type Theme = Theme;
  type Flags = ();

  fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
    (
      ContainerExample {
      },
      Command::none()
    )
  }

  fn title(&self) -> String {
    String::from("Button Example")
  }

  fn update(&mut self, message: Message) -> Command<Self::Message> {
    match message {
      Message::Open(url) => {
        opener::open(url).ok();
      }
    }
    Command::none()
  }

  fn view(&self) -> Element<Message> {
    Scrollable::new(
      Column::with_children(vec![
        utils::docs_link("Container", 36.0, Message::Open(String::from("https://docs.rs/iced/latest/iced/widget/struct.Container.html"))),
        Text::new("An element decorating some content. It is normally used for alignment purposes.").into(),

        Rule::horizontal(10).into(),

        utils::docs_link("new", 24.0, Message::Open(String::from("https://docs.rs/iced/latest/iced/widget/struct.Container.html#method.new"))),
        Element::new(Row::new()).explain(Color::BLACK),
        Text::new("Creates an empty Row. By default width and height will try to fit its children. An empty Row will be 0 by 0.").into(),

        Rule::horizontal(10).into(),

        utils::docs_link("height", 24.0, Message::Open(String::from("https://docs.rs/iced/latest/iced/widget/struct.Container.html#method.height"))),
        Element::new(Container::new(Text::new("Container")).width(50)).explain(Color::BLACK),
        Text::new("Sets the width of the Container.").into(),

        Rule::horizontal(10).into(),

        utils::docs_link("height", 24.0, Message::Open(String::from("https://docs.rs/iced/latest/iced/widget/struct.Container.html#method.height"))),
        Element::new(Container::new(Text::new("Container")).height(50)).explain(Color::BLACK),
        Text::new("Sets the height of the Row.").into(),

        Rule::horizontal(10).into(),

        utils::docs_link("max_width", 24.0, Message::Open(String::from("https://docs.rs/iced/latest/iced/widget/struct.Container.html#method.max_width"))),
        Element::new(Container::new(Text::new("Fooooooooooooooooooooooooooooooooooooooo")).max_width(50)).explain(Color::BLACK),
        Text::new("Sets the maximum width of the Container.").into(),

        Rule::horizontal(10).into(),

        utils::docs_link("max_height", 24.0, Message::Open(String::from("https://docs.rs/iced/latest/iced/widget/struct.Container.html#method.max_height"))),
        Element::new(Container::new(Container::new(Text::new("Container")).height(100)).max_height(50)).explain(Color::BLACK),
        Text::new("Sets the maximum height of the Container.").into(),

        Rule::horizontal(10).into(),

        utils::docs_link("padding", 24.0, Message::Open(String::from("https://docs.rs/iced/latest/iced/widget/struct.Container.html#method.padding"))),
        Element::new(Container::new(Text::new("Container"))
          .padding([5, 10, 15, 20])).explain(Color::BLACK),
        Text::new("Sets the Padding of the Row.").into(),

        Rule::horizontal(10).into(),

        utils::docs_link("align_x", 24.0, Message::Open(String::from("https://docs.rs/iced/latest/iced/widget/struct.Container.html#method.align_x"))),
        Element::new(Container::new(Text::new("Horizontal::Left")).align_x(Horizontal::Left).width(600).height(100)).explain(Color::BLACK),
        Element::new(Container::new(Text::new("Horizontal::Center")).align_x(Horizontal::Center).width(600).height(100)).explain(Color::BLACK),
        Element::new(Container::new(Text::new("Horizontal::Right")).align_x(Horizontal::Right).width(600).height(100)).explain(Color::BLACK),
        Text::new("Sets the content alignment for the horizontal axis of the Container.").into(),

        Rule::horizontal(10).into(),

        utils::docs_link("align_y", 24.0, Message::Open(String::from("https://docs.rs/iced/latest/iced/widget/struct.Container.html#method.align_y"))),
        Element::new(Container::new(Text::new("Vertical::Left")).align_y(Vertical::Top).width(600).height(100)).explain(Color::BLACK),
        Element::new(Container::new(Text::new("Vertical::Center")).align_y(Vertical::Center).width(600).height(100)).explain(Color::BLACK),
        Element::new(Container::new(Text::new("Vertical::Bottom")).align_y(Vertical::Bottom).width(600).height(100)).explain(Color::BLACK),
        Text::new("Sets the content alignment for the vertical axis of the Container.").into(),

        Rule::horizontal(10).into(),

        utils::docs_link("center_x", 24.0, Message::Open(String::from("https://docs.rs/iced/latest/iced/widget/struct.Container.html#method.center_x"))),
        Element::new(Container::new(Text::new("center_x")).center_x().width(600).height(100)).explain(Color::BLACK),
        Text::new("A shortcut for .align(Horizontal::Center).").into(),

        Rule::horizontal(10).into(),

        utils::docs_link("center_y", 24.0, Message::Open(String::from("https://docs.rs/iced/latest/iced/widget/struct.Container.html#method.center_y"))),
        Element::new(Container::new(Text::new("center_y")).center_y().width(600).height(100)).explain(Color::BLACK),
        Text::new("A shortcut for .align_y(Vertical::Center).").into(),

        Rule::horizontal(10).into(),

        utils::docs_link("id", 24.0, Message::Open(String::from("https://docs.rs/iced/latest/iced/widget/struct.Container.html#method.id"))),
        Element::new(Container::new(Text::new("Container")).id(Id::new("a"))).explain(Color::BLACK),
        Element::new(Container::new(Text::new("Container")).id(Id::new("a"))).explain(Color::BLACK),
        Text::new("A shortcut for .align_y(Vertical::Center).").into(),

      ]).padding([100, 100, 100, 100])
    ).into()
  }

  fn theme(&self) -> Theme {
    Theme::default()
  }
}