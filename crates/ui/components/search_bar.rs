use std::{borrow::Cow, fmt::Display};

use freya::prelude::*;

#[derive(PartialEq)]
pub struct Searchbar<T> {
  data: Vec<T>,
  placeholder: String,
  on_click: Option<EventHandler<T>>,
}

impl<T: PartialEq + Clone + Display> Searchbar<T> {
  pub fn new(data: Vec<T>) -> Self {
    Self {
      data,
      placeholder: String::new(),
      on_click: None,
    }
  }

  pub fn placeholder<'a>(mut self, placeholder: impl Into<Cow<'a, str>>) -> Self {
    self.placeholder = String::from(placeholder.into());
    self
  }

  pub fn on_click(mut self, on_click: impl Into<EventHandler<T>>) -> Self {
    self.on_click = Some(on_click.into());
    self
  }
}

impl<T: PartialEq + Clone + Display + 'static> Component for Searchbar<T> {
  fn render(&self) -> impl IntoElement {
    let on_click = self.on_click.clone();
    let search_str = use_state(String::new);

    rect()
      .horizontal()
      .content(Content::Flex)
      .child(
        Input::new(search_str)
          .width(Size::flex(1.))
          .height(Size::px(32.))
          .placeholder(self.placeholder.clone())
          .corner_radius((6., 0., 0., 6.)),
      )
      .child(
        Button::new()
          .width(Size::px(32.))
          .height(Size::px(32.))
          .corner_radius((0., 6., 6., 0.))
          .child("+"),
      )
      .child(Select::new().children())
  }
}
