use iced::{Element, Renderer, theme};
use iced::widget::{Button, Text};

#[derive(Debug, Clone, Copy)]
struct UrlButtonStyle {}

impl iced::widget::button::StyleSheet for UrlButtonStyle {
  type Style = iced::Theme;

  fn active(&self, style: &Self::Style) -> iced::widget::button::Appearance {
    iced::widget::button::Appearance {
      shadow_offset: Default::default(),
      background: None,
      border_radius: Default::default(),
      border_width: 0.0,
      border_color: Default::default(),
      text_color: style.palette().text,
    }
  }

  fn hovered(&self, style: &Self::Style) -> iced::widget::button::Appearance {
    iced::widget::button::Appearance {
      shadow_offset: Default::default(),
      background: None,
      border_radius: Default::default(),
      border_width: 0.0,
      border_color: Default::default(),
      text_color: style.palette().primary,
    }
  }

  fn pressed(&self, style: &Self::Style) -> iced::widget::button::Appearance {
    iced::widget::button::Appearance {
      shadow_offset: Default::default(),
      background: None,
      border_radius: Default::default(),
      border_width: 0.0,
      border_color: Default::default(),
      text_color: style.palette().primary,
    }
  }
}

pub fn docs_link<'a, Message>(label: &'a str, size: f32, message: Message) -> Element<'a, Message> where Message: Clone + 'a {
  Element::<Message>::new(Button::<Message, Renderer>::new(Text::new(label).size(size))
    .on_press(message)
    .style(theme::Button::Custom(Box::new(UrlButtonStyle {}))))
}