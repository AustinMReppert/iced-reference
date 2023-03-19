# State Management

Iced has a simple model for state. The type that implements the Application trait stores the entirety of your application's state.


## Cargo.toml
We will need to add a few things to Cargo.toml.
```toml
# ...
[dependencies]
# The core intefaces for the iced framework.
iced = { git = "https://github.com/iced-rs/iced", features = ["tokio"]}
# For time
chrono = "0.2.16"

# For an HTTP client:
reqwest = { version = "0.11", features = ["json"] }
tokio = { version = "1", features = ["full"] }

# For getting JSON out of the HTTP APIs:
serde = { version = "1.0.157", features = ["derive"] }
serde_json = "1.0.94"
```

## Handling Basic UI Events
New, view, and update presented in chapter 1 provide the basic blocks to manage state.

View tells Iced how you want your state to be displayed. But view is also important because the way the UI is displayed will also change the Messages your UI produces from button clicks or other UI events. Lets look at an example to see how easy it is to update our state in response to basic UI events.

```rust
use iced::alignment::{Horizontal, Vertical};
use iced::widget::{Button, Container, Row, Text};
use iced::{executor, Alignment, Application, Command, Element, Length, Settings, Theme};

extern crate chrono;

pub fn main() -> iced::Result {
  HelloWorld::run(Settings::default())
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
  UpdateTime,
}

struct HelloWorld {
  current_time: chrono::DateTime<chrono::Local>,
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
      },
      Command::none(),
    )
  }

  fn title(&self) -> String {
    String::from("Basic State 1")
  }

  fn update(&mut self, message: Message) -> Command<Self::Message> {
    match message {
      Message::UpdateTime => {
        self.current_time = chrono::Local::now();
      }
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

    let update = Button::new("Update").on_press(Message::UpdateTime).into();

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
}
```

We create a `current_time` field in our Application's state. It is initialized in `new()`. To render our state we convert it to a string and put it inside a Text Widget. We also create a Button that can update date the current time. The Message that our update button generates for on_press is specified by calling `.on_press(Message::UpdateTime)`. Notice how the `view()` isolates itself from `update()` logic by passing any necessary context into a Message. And in `update()` we check the type of Message and update the `current_time` state if the Message is of the UpdateTime variant.


This scenario is the simplest to handle. The Button Widget does the heavy lifting for handling events, we only need to tell it what type of message we want. Not all events are as simple to handle.

Consider how you could automatically produce an UpdateTime Message with only new, view, and update.

## Subscriptions

To handle more general events we will take a loot at `subscription()` from the Application trait.

```rust
    /// Returns the event [`Subscription`] for the current state of the
    /// application.
    ///
    /// A [`Subscription`] will be kept alive as long as you keep returning it,
    /// and the __messages__ produced will be handled by
    /// [`update`](#tymethod.update).
    ///
    /// By default, this method returns an empty [`Subscription`].
    fn subscription(&self) -> Subscription<Self::Message> {
        Subscription::none()
    }
```

Subscription() returns a `Subscription`. By default, it will return `Subscription::none()`. Subscription is called when a Message is produced or a non-empty Subscription is returned. Lets look closer at the type definition of Subscription:

```rust
/// A request to listen to external events.
///
/// Besides performing async actions on demand with [`Command`], most
/// applications also need to listen to external events passively.
///
/// A [`Subscription`] is normally provided to some runtime, like a [`Command`],
/// and it will generate events as long as the user keeps requesting it.
///
/// For instance, you can use a [`Subscription`] to listen to a WebSocket
/// connection, keyboard presses, mouse events, time ticks, etc.
///
/// [`Command`]: crate::Command
pub type Subscription<T> =
    iced_futures::Subscription<Hasher, (Event, event::Status), T>;
```

Not super helpful -- lets dig further into the type.
```rust
/// A request to listen to external events.
///
/// Besides performing async actions on demand with [`Command`], most
/// applications also need to listen to external events passively.
///
/// A [`Subscription`] is normally provided to some runtime, like a [`Command`],
/// and it will generate events as long as the user keeps requesting it.
///
/// For instance, you can use a [`Subscription`] to listen to a WebSocket
/// connection, keyboard presses, mouse events, time ticks, etc.
///
/// This type is normally aliased by runtimes with a specific `Event` and/or
/// `Hasher`.
///
/// [`Command`]: crate::Command
#[must_use = "`Subscription` must be returned to runtime to take effect"]
pub struct Subscription<Hasher, Event, Output> {
    recipes: Vec<Box<dyn Recipe<Hasher, Event, Output = Output>>>,
}
```
And into Recipe...

```rust
/// The description of a [`Subscription`].
///
/// A [`Recipe`] is the internal definition of a [`Subscription`]. It is used
/// by runtimes to run and identify subscriptions. You can use it to create your
/// own!
//...
pub trait Recipe<Hasher: std::hash::Hasher, Event> {
    /// The events that will be produced by a [`Subscription`] with this
    /// [`Recipe`].
    type Output;

    /// Hashes the [`Recipe`].
    ///
    /// This is used by runtimes to uniquely identify a [`Subscription`].
    fn hash(&self, state: &mut Hasher);

    /// Executes the [`Recipe`] and produces the stream of events of its
    /// [`Subscription`].
    ///
    /// It receives some stream of generic events, which is normally defined by
    /// shells.
    fn stream(
        self: Box<Self>,
        input: BoxStream<Event>,
    ) -> BoxStream<Self::Output>;
}
```

An Iced `Recipe` is similar to Tokio's Subscription. A `Subscription` is Vec of `Recipes`. `Recipes` are like Tokio's `Streams`. As long as they are kept alive, they can keep producing values or in this case Messages. Subscription() allows us to run async code to continuously provide UI state changes. 

Lets incorporate subscription() into our timer to automatically update the time. We will let the button remain as a pause/unpause mechanism.

```rust
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
```

Creating custom Subscriptions will be covered in a later tutorial.

## Subscriptions vs Commands

That sounds a lot like Command, what's the difference?

Subscriptions are for listening to outside events and creating messages to represent the external changes. These external events may include changes to a file, receiving network requests, etc.

Commands on the other hand are for responding to user interactions. When the user clicks a button, `update()` will receive a message. If a button needs to perform a task such as an HTTP request this may take a long time. If we try to keep `update()` simple by using a synchronous HTTP library we will freeze all other UI events until our request is completed. In short we need to run async code in response to a Message directly triggered by a user. Returning a `Command` from `update()` will let us do exactly that.

## Commands

Commands are wrappers around futures. Iced can run Commands returned from `new()` or `update()`.

```rust
/// A set of asynchronous actions to be performed by some runtime.
#[must_use = "`Command` must be returned to runtime to take effect"]
pub struct Command<T>(iced_futures::Command<Action<T>>);
```
We can have commands that do nothing, a single thing, or 1 or more things.
```rust
/// A set of asynchronous actions to be performed by some runtime.
#[must_use = "`Command` must be returned to runtime to take effect"]
#[derive(Debug)]
pub struct Command<T>(Internal<T>);

#[derive(Debug)]
enum Internal<T> {
    None,
    Single(T),
    Batch(Vec<T>),
}
```

To create Commands there are helper functions.

```rust
    /// Creates a [`Command`] that performs the action of the given future.
    pub fn perform<A>(
        future: impl Future<Output = T> + 'static + MaybeSend,
        f: impl FnOnce(T) -> A + 'static + MaybeSend,
    ) -> Command<A> {
        use iced_futures::futures::FutureExt;

        Command::single(Action::Future(Box::pin(future.map(f))))
    }
```

Lets look at an example creating a Command::Single with `perform()`. It's quite simple. We need a future/(block of async code) and a Message to send when the async code is complete. Note the return type of the async code needs to match the Message contents.

```rust
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
```