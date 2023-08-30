mod utils;

use iced::widget::{Column, Rule, Scrollable, Text};
use iced::{executor, Application, Command, Element, Settings, Theme, Color, Alignment};

pub fn main() -> iced::Result {
  ColumnExample::run(Settings::default())
}

#[derive(Debug, Clone)]
pub enum Message {
  Open(String),
}

struct ColumnExample {}

impl Application for ColumnExample {
  type Executor = executor::Default;
  type Message = Message;
  type Theme = Theme;
  type Flags = ();

  fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
    (
      ColumnExample {
      },
      Command::none(),
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
        utils::docs_link("Column", 36.0, Message::Open(String::from("https://docs.rs/iced/latest/iced/widget/struct.Column.html"))),
        Text::new("A container that distributes its contents vertically.").into(),

        Rule::horizontal(10).into(),

        utils::docs_link("new", 24.0, Message::Open(String::from("https://docs.rs/iced/latest/iced/widget/struct.Column.html#method.new"))),
        Element::new(Column::new()).explain(Color::BLACK),
        Text::new("Creates an empty Column. By default width and height will try to fit its children. An empty column will be 0 by 0.").into(),

        Rule::horizontal(10).into(),

        utils::docs_link("with_children", 24.0, Message::Open(String::from("https://docs.rs/iced/latest/iced/widget/struct.Column.html#method.with_children"))),
        Element::new(Column::with_children(vec![
          Text::new("Child 1").into(),
          Text::new("Child 2").into(),
          Text::new("Child 3").into(),
        ])).explain(Color::BLACK),
        Text::new("Creates a Column with the given elements.").into(),

        Rule::horizontal(10).into(),

        utils::docs_link("with_children", 24.0, Message::Open(String::from("https://docs.rs/iced/latest/iced/widget/struct.Column.html#method.push"))),
        Element::new(Column::new()
          .push(Text::new("Child 1"))
          .push(Text::new("Child 2"))
          .push(Text::new("Child 3"))
          ).explain(Color::BLACK),
        Text::new("Adds an element to the Column.").into(),

        Rule::horizontal(10).into(),

        utils::docs_link("width", 24.0, Message::Open(String::from("https://docs.rs/iced/latest/iced/widget/struct.Column.html#method.width"))),
        Element::new(Column::new()
          .push(Text::new("Child 1"))
          .push(Text::new("Child 2"))
          .push(Text::new("Child 3"))
          .width(100)).explain(Color::BLACK),
        Text::new("Sets the width of the Column.").into(),

        Rule::horizontal(10).into(),

        utils::docs_link("height", 24.0, Message::Open(String::from("https://docs.rs/iced/latest/iced/widget/struct.Column.html#method.height"))),
        Element::new(Column::new()
          .push(Text::new("Child 1"))
          .push(Text::new("Child 2"))
          .push(Text::new("Child 3"))
          .height(100)).explain(Color::BLACK),
        Text::new("Sets the height of the Column.").into(),

        Rule::horizontal(10).into(),

        utils::docs_link("max_width", 24.0, Message::Open(String::from("https://docs.rs/iced/latest/iced/widget/struct.Column.html#method.max_width"))),
        Element::new(Column::new()
          .push(Text::new("Child 1"))
          .push(Text::new("Child 22222222222222222222222222222222222222222"))
          .push(Text::new("Child 3"))
          .max_width(100)).explain(Color::BLACK),
        Text::new("Sets the maximum width of the Column.").into(),

        Rule::horizontal(10).into(),

        utils::docs_link("padding", 24.0, Message::Open(String::from("https://docs.rs/iced/latest/iced/widget/struct.Column.html#method.padding"))),
        Element::new(Column::new()
          .push(Text::new("Child 1"))
          .push(Text::new("Child 22222222222222222222222222222222222222222"))
          .push(Text::new("Child 3"))
          .padding([5, 10, 15, 20])).explain(Color::BLACK),
        Text::new("Sets the Padding of the Column.").into(),

        Rule::horizontal(10).into(),

        utils::docs_link("spacing", 24.0, Message::Open(String::from("https://docs.rs/iced/latest/iced/widget/struct.Column.html#method.spacing"))),
        Element::new(Column::new()
          .push(Text::new("Child 1"))
          .push(Text::new("Child 22222222222222222222222222222222222222222"))
          .push(Text::new("Child 3"))
          .spacing(20)).explain(Color::BLACK),
        Text::new("Sets the vertical spacing between elements.").into(),

        Rule::horizontal(10).into(),

        utils::docs_link("align_items", 24.0, Message::Open(String::from("https://docs.rs/iced/latest/iced/widget/struct.Column.html#method.align_items"))),
        Element::new(Column::new()
          .push(Text::new("Alignment::Start"))
          .push(Text::new("Alignment::Start"))
          .push(Text::new("Alignment::Start"))
          .align_items(Alignment::Start).width(300).height(300)).explain(Color::BLACK),
        Element::new(Column::new()
          .push(Text::new("Alignment::Center"))
          .push(Text::new("Alignment::Center"))
          .push(Text::new("Alignment::Center"))
          .align_items(Alignment::Center).width(300).height(300)).explain(Color::BLACK),
        Element::new(Column::new()
          .push(Text::new("Alignment::End"))
          .push(Text::new("Alignment::End"))
          .push(Text::new("Alignment::End"))
          .align_items(Alignment::End).width(300).height(300)).explain(Color::BLACK),
        Text::new("Sets the horizontal alignment of the contents of the Column.").into(),

      ]).padding([100, 100, 100, 100])
    ).into()
  }

  fn theme(&self) -> Theme {
    Theme::default()
  }
}