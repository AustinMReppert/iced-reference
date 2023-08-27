# Styling

Styling changes the appearance of the Iced Application and its Widgets. Styling does not affect the layout.
To style a Widget call the Widget's `.style()` function and pass a Stylesheet. 

For this example we are focusing on a Button. It is similar for other `Widgets`.
```rust
/// Sets the style variant of this [`Button`].
pub fn style(
    mut self,
    style: <Renderer::Theme as StyleSheet>::Style,
) -> Self {
    self.style = style;
    self
}
```
When we try to call it:
```rust
use iced::widget::Button;
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
    String::from("Styling 1")
  }

  fn update(&mut self, _message: Message) -> Command<Self::Message> {
    Command::none()
  }

  fn view(&self) -> Element<Message> {
    Element::new(Button::new("Test").on_press(Message::Pressed).style(
      // ??????????
    ))
  }

  fn theme(&self) -> Self::Theme {
    Self::Theme::default()
  }
}
```
Now when we try to call `style()` our IDE will tell us we need a `::<::Theme as StyleSheet>::Style`.
The types are getting confusing again. Lets see if we can expand them to see what is going on.

```rust
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
      iced::widget::Button::<Message, iced::Renderer<Self::Theme>>::new("Test").on_press(Message::Pressed).style(
        // style: <Theme as iced::widget::button::StyleSheet>::Style
      )
    )
  }

  fn theme(&self) -> Self::Theme {
    Self::Theme::default()
  }
}
```

The first thing you should notice is that Button is aliased to a `iced::widget::Button`. We also see that Rust uses our Application's `Theme` type as a parameter to the `Renderer`. `Widgets` take in a `Renderer` type as a parameter. This seems fairly obvious when you think about it. However, `Renderer` requiring a `Theme` type seems quite strange. It seems like all we should need to do is store some style properties like color in the Button itself and let the Button give style information to the Renderer. In that scenario, `Button` doesn't really do much other than store the Styles. Instead, Iced delegates the task of styling `Widgets` to Stylesheets. Stylesheets are simply traits that have functions to return a struct with a generic set of style properties for a `Widget`. Lets look at the button's stylesheet(iced::widget::button::StyleSheet).

```rust
//! Change the appearance of a button.
use iced_core::{Background, Color, Vector};

/// The appearance of a button.
#[derive(Debug, Clone, Copy)]
pub struct Appearance {
    /// The amount of offset to apply to the shadow of the button.
    pub shadow_offset: Vector,
    /// The [`Background`] of the button.
    pub background: Option<Background>,
    /// The border radius of the button.
    pub border_radius: BorderRadius,
    /// The border width of the button.
    pub border_width: f32,
    /// The border [`Color`] of the button.
    pub border_color: Color,
    /// The text [`Color`] of the button.
    pub text_color: Color,
}

impl std::default::Default for Appearance {
    fn default() -> Self {
        Self {
            shadow_offset: Vector::default(),
            background: None,
            border_radius: 0.0.into(),
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
            text_color: Color::BLACK,
        }
    }
}

/// A set of rules that dictate the style of a button.
pub trait StyleSheet {
    /// The supported style of the [`StyleSheet`].
    type Style: Default;

    /// Produces the active [`Appearance`] of a button.
    fn active(&self, style: &Self::Style) -> Appearance;

    /// Produces the hovered [`Appearance`] of a button.
    fn hovered(&self, style: &Self::Style) -> Appearance {
        let active = self.active(style);

        Appearance {
            shadow_offset: active.shadow_offset + Vector::new(0.0, 1.0),
            ..active
        }
    }

    /// Produces the pressed [`Appearance`] of a button.
    fn pressed(&self, style: &Self::Style) -> Appearance {
        Appearance {
            shadow_offset: Vector::default(),
            ..self.active(style)
        }
    }

    /// Produces the disabled [`Appearance`] of a button.
    fn disabled(&self, style: &Self::Style) -> Appearance {
        let active = self.active(style);

        Appearance {
            shadow_offset: Vector::default(),
            background: active.background.map(|background| match background {
                Background::Color(color) => Background::Color(Color {
                    a: color.a * 0.5,
                    ..color
                }),
            }),
            text_color: Color {
                a: active.text_color.a * 0.5,
                ..active.text_color
            },
            ..active
        }
    }
}

```

There are functions for the different states of a `Button`, but they all return an `Appearance` struct. We also see there is an associated type: `type Style: Default`. This is the type that the `.style()` function requires. It is used to store additional information needed in the different stylesheet functions. Since it is an associated type we have to look at the actual value in `iced::Theme` to see what the `button.style()` function requires. A "theme" is simply a type with Stylsheet traits implemented on it.

```rust
/// A built-in theme.
#[derive(Debug, Clone, PartialEq, Default)]
pub enum Theme {
    /// The built-in light variant.
    #[default]
    Light,
    /// The built-in dark variant.
    Dark,
    /// A [`Theme`] that uses a [`Custom`] palette.
    Custom(Box<Custom>),
}
```

```rust
impl button::StyleSheet for Theme {
    type Style = Button;
    // ...
```

```rust
/// The style of a button.
#[derive(Default)]
pub enum Button {
    /// The primary style.
    #[default]
    Primary,
    /// The secondary style.
    Secondary,
    /// The positive style.
    Positive,
    /// The destructive style.
    Destructive,
    /// The text style.
    ///
    /// Useful for links!
    Text,
    /// A custom style.
    Custom(Box<dyn button::StyleSheet<Style = Theme>>),
}
```

So all `style()` really needs is a `Button` enum. There are some generic options in the enum or a Custom variant that simply needs a Box containing a type that implements the Button Stylesheet trait. Lets see how that is done.

```rust
use iced::widget::button::{Appearance, StyleSheet};
use iced::widget::Button;
use iced::{executor, Application, Color, Command, Element, Settings, BorderRadius};

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
      border_radius: BorderRadius::from(0.0),
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
```

Typically you will choose a theme so that you application looks similar to other applications on your OS. Unless you are using PopOS's Cosmic you will have to either create your own theme or use the default Iced theme.

To create your own theme you simply need to: 
1. Implement required Stylesheet traits + Default on your theme type
2. Provide your theme type to Application.
3. Provide your theme type to view's return type.

```rust
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
```

This would work but the easier solution is to copy the default Iced theme and modify as needed.