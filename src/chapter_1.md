# Chapter 1

## Warnings
* Iced is not production ready and is not yet 1.0. Some tinkering will be involved.
* For this tutorial, the web aspect of Iced will be ignored.
* Iced is not React/HTML/CSS/Swing/etc. and you should not treat it as such.
* This tutorial targets the latest commits.

## Setup

Dependencies:
```toml
# The core intefaces for the iced framework.
iced = { git = "https://github.com/iced-rs/iced" }
```

## Hello World.

```rust
use iced::widget::Text;
use iced::{Command, executor, Application, Element, Settings, Theme};

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

  fn new(flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
    (HelloWorld {}, Command::none())
  }

  fn title(&self) -> String {
    String::from("Hello World!")
  }

  fn update(&mut self, message: Message) -> iced::Command<Self::Message> {
    Command::none()
  }

  fn view(&self) -> Element<Message> {
    Element::new(Text::new("Hello World!"))
  }

  fn theme(&self) -> Theme {
    Theme::default()
  }
}
```

## Applications
All Iced GUIs start with an Application trait implemented on a struct. The Application trait requires a few associated types:
### Executor
Specifies which runtime will run async Iced code. This may change depending on your target platform i.e. desktop/web/etc.
### Message
Message is a type used by your application when any UI state changes occur. It must implement Debug and Send (indicates a type which can be sent across threads) traits.
Because this type will be used for application Messages, it should be fast and ergonomic to pass around. Deriving the Clone and Copy traits is the easiest way to do this.
Message will usually be an enum because it has to encode all possible UI events.
### Theme
A type used for styling information. This type must implement the Default and StyleSheet traits.
### Flags
Flags is a type for transferring information to your Iced Application at startup.

---

### Setting-up an Application:
#### 1. Setup Initial State with new()
```rust
fn new(flags: Self::Flags) -> (Self, Command<Self::Message>) {
  (HelloWorld {}, Command::none())
}
```
New is like any other "new" function in Rust. Its goal is to provide a valid starting state. The 2nd tuple value is used for async operations that may need to be performed at startup -- like authentication.
#### 2. Display your state with Widgets using view()
```rust
fn view(&self) -> Element<Message> {
  Element::new(Text::new("Hello World!"))
}
```
View takes a reference to self providing access to the current UI state and outputs Widgets.
Iced tries to minimize the amount of calls to view. It is typically called for initial renders or in response to UI Messages.
Widgets are structs implementing the Widget trait.
```rust
pub trait Widget<Message, Renderer>
where
Renderer: crate::Renderer
```
Widgets are the link between raw UI data and Iced. Widgets know what data they have, how to render it, and when changes to the data should emit Messages.
The Widget trait has a Renderer and a Message type parameter. Luckily, these are usually inferred.
Notice that view does not want just any widget. It wants an Element!

To get an element from a built-in widget you can use `Element::new()` or call `.into()`.
An Element is a generic Widget that holds a Box to another Widget. Elements abstract away implementation details and make Iced code easier to read.
#### 3. Update UI State in response to UI Messages With update()
Update is responsible for handling Messages and modifying UI state.
```rust
fn update(&mut self, message: Message) -> iced::Command<Self::Message> {
  Command::none()
}
```
Notice update has a mutable reference to the UI data allowing us to centralize modifications to our data.
This pattern plays well with the design pattern. Similar to new, update can return a Command for async operations like downloading a file.

---

## Starting Iced
We simply need to tell Iced to take over.
```rust
pub fn main() -> iced::Result {
  HelloWorld::run(Settings::default())
}
```
Note: run will not return until the Iced window has closed.