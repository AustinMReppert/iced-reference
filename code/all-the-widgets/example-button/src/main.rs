mod utils;

use std::fmt::{Display, Formatter};
use iced::widget::{Button, Column, combo_box, ComboBox, Row, Rule, Scrollable, Space, Text};
use iced::{executor, Application, Command, Element, Settings, Theme, theme};

pub fn main() -> iced::Result {
  ButtonExample::run(Settings::default())
}

#[derive(Debug, Clone)]
pub enum Message {
  DoNothing,
  Open(String),
  SelectedTheme(ButtonStyle),
}

#[derive(Debug, Clone, Copy)]
pub enum ButtonStyle {
  Primary,
  Secondary,
  Destructive,
  Text,
}

impl Display for ButtonStyle {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      ButtonStyle::Primary => write!(f, "Primary"),
      ButtonStyle::Secondary => write!(f, "Secondary"),
      ButtonStyle::Destructive => write!(f, "Destructive"),
      ButtonStyle::Text => write!(f, "Text"),
    }
  }
}

struct ButtonExample {
  combo_box_state: combo_box::State<ButtonStyle>,
  selected_theme: ButtonStyle,
}

const ALL: [ButtonStyle; 4] = [
  ButtonStyle::Primary,
  ButtonStyle::Secondary,
  ButtonStyle::Destructive,
  ButtonStyle::Text,
];

impl Application for ButtonExample {
  type Executor = executor::Default;
  type Message = Message;
  type Theme = Theme;
  type Flags = ();

  fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
    (
      ButtonExample {
        combo_box_state: combo_box::State::new(ALL.to_vec()),
        selected_theme: ButtonStyle::Destructive,
      },
      Command::none(),
    )
  }

  fn title(&self) -> String {
    String::from("Button Example")
  }

  fn update(&mut self, message: Message) -> Command<Self::Message> {
    match message {
      Message::DoNothing => {}
      Message::Open(url) => {
        opener::open(url).ok();
      }
      Message::SelectedTheme(selected_theme) => {
        self.selected_theme = selected_theme;
        self.combo_box_state.unfocus();
      }
    }
    Command::none()
  }

  fn view(&self) -> Element<Message> {
    Scrollable::new(
      Column::with_children(vec![
        utils::docs_link("Button", 36.0, Message::Open(String::from("https://docs.rs/iced/latest/iced/widget/button/struct.Button.html"))),

        Rule::horizontal(10).into(),

        utils::docs_link("new", 24.0, Message::Open(String::from("https://docs.rs/iced/latest/iced/widget/button/struct.Button.html#method.new"))),
        Button::new("A button").into(),
        Text::new("Creates a button with a child. The child must implement Into<Element<'a, Message, Renderer>>. Usually you will give it text or a widget.").into(),

        Rule::horizontal(10).into(),

        Text::new("Attributes").size(32).into(),

        Rule::horizontal(10).into(),

        utils::docs_link("On Press", 24.0, Message::Open(String::from("https://docs.rs/iced/latest/iced/widget/button/struct.Button.html#method.on_press"))),
        Button::new("A button").on_press(Message::DoNothing).into(),
        Text::new("Sets the message that will be produced when the Button is pressed. Buttons that do not produce any messages are disabled.").into(),

        utils::docs_link("On Press Maybe", 24.0, Message::Open(String::from("https://docs.rs/iced/latest/iced/widget/button/struct.Button.html#method.on_press_maybe"))),
        Button::new("A button").on_press_maybe(Some(Message::DoNothing)).into(),
        Text::new("Sets the message that will be produced when the Button is pressed, if Some.").into(),

        Rule::horizontal(10).into(),

        utils::docs_link("Width", 24.0, Message::Open(String::from("https://docs.rs/iced/latest/iced/widget/button/struct.Button.html#method.width"))),
        Button::new("A button").width(100).on_press(Message::DoNothing).into(),
        Text::new("Sets the width of the Button.").into(),

        Rule::horizontal(10).into(),

        utils::docs_link("Height", 24.0, Message::Open(String::from("https://docs.rs/iced/latest/iced/widget/button/struct.Button.html#method.height"))),
        Button::new("A button").height(100).on_press(Message::DoNothing).into(),
        Text::new("Sets the width of the Button.").into(),

        Rule::horizontal(10).into(),

        utils::docs_link("Padding", 24.0, Message::Open(String::from("https://docs.rs/iced/latest/iced/widget/button/struct.Button.html#method.padding"))),
        Button::new("A button").padding([5, 10, 15, 20]).on_press(Message::DoNothing).into(),
        Text::new("Sets the Padding of the Button.").into(),

        Rule::horizontal(10).into(),

        utils::docs_link("Style", 24.0, Message::Open(String::from("https://docs.rs/iced/latest/iced/widget/button/struct.Button.html#method.style"))),
        Button::new("A button").padding([5, 10, 15, 20]).on_press(Message::DoNothing).style(match self.selected_theme {
          ButtonStyle::Primary => theme::Button::Primary,
          ButtonStyle::Secondary => theme::Button::Secondary,
          ButtonStyle::Destructive => theme::Button::Destructive,
          ButtonStyle::Text => theme::Button::Text,
        }).into(),
        Row::with_children(vec![
          Text::new("Style:").into(),
          Space::with_width(10).into(),
          ComboBox::new(&self.combo_box_state, "", Some(&self.selected_theme), Message::SelectedTheme).width(200).into(),
        ]).into(),
        Text::new("Sets the style variant of this Button.").into(),
      ]).padding([100, 100, 100, 100])
    ).into()
  }

  fn theme(&self) -> Theme {
    Theme::default()
  }
}