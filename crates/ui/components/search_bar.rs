use freya::prelude::*;
use std::borrow::Cow;
use std::fmt::Display;

pub enum Item<T> {
  Existing(T),
  New(String),
}

#[derive(PartialEq)]
pub struct Searchbar<T> {
  data: Vec<T>,
  placeholder: String,
  on_submit: Option<EventHandler<Item<T>>>,
}

impl<T: PartialEq + Clone + Display> Searchbar<T> {
  pub fn new(data: Vec<T>) -> Self {
    Self {
      data,
      placeholder: String::new(),
      on_submit: None,
    }
  }

  pub fn placeholder<'a>(mut self, placeholder: impl Into<Cow<'a, str>>) -> Self {
    self.placeholder = String::from(placeholder.into());
    self
  }

  pub fn on_submit(mut self, on_submit: impl Into<EventHandler<Item<T>>>) -> Self {
    self.on_submit = Some(on_submit.into());
    self
  }
}

impl<T: PartialEq + Clone + Display + 'static> Component for Searchbar<T> {
  fn render(&self) -> impl IntoElement {
    let mut menu_visible = use_state(|| false);
    let on_submit = self.on_submit.clone();
    let on_submit2 = self.on_submit.clone();
    let data = use_state(|| self.data.clone());
    let search_str = use_state(String::new);

    let mut submit_with_str = move |input_str: &str| {
      println!("NAME={input_str}");
      menu_visible.set(false);

      if let Some(func) = on_submit.as_ref() {
        let ret = data
          .read()
          .as_slice()
          .iter()
          .find(|x| input_str == &format!("{x}"))
          .map(|x| Item::Existing(x.clone()))
          .unwrap_or_else(|| Item::New(input_str.to_string()));

        func.call(ret);
      }
    };

    let submit_with_idx = move |idx: usize| {
      println!("IDX={idx}, NAME={}", data.read()[idx].clone());
      menu_visible.set(false);

      if let Some(func) = on_submit2.as_ref() {
        func.call(Item::Existing(data.read()[idx].clone()));
      }
    };

    rect()
      .child(
        rect()
          .horizontal()
          .content(Content::Flex)
          .child(
            Input::new(search_str)
              .on_pre_key_down(move |_| {
                menu_visible.set(true);
                true
              })
              .on_submit(move |x: String| submit_with_str(&x))
              .width(Size::flex(1.))
              .height(Size::px(32.))
              .placeholder(self.placeholder.clone())
              .corner_radius((6., 0., 0., 6.)),
          )
          .child(
            Button::new()
              .on_press(move |_| {
                menu_visible.set(false);
              })
              .width(Size::px(32.))
              .height(Size::px(32.))
              .corner_radius((0., 6., 6., 0.))
              .child("+"),
          ),
      )
      .maybe_child(
        menu_visible().then_some(
          Menu::new()
            .on_close(move |_| menu_visible.set(false))
            .children(data.read().iter().enumerate().filter_map(move |(idx, x)| {
              let formatted = format!("{x}");
              let search = search_str.read();

              if !formatted
                .to_lowercase()
                .contains(&search.as_str().to_lowercase())
              {
                return None;
              }

              let mut submit = submit_with_idx.clone();

              Some(MenuButton::new().child(formatted).on_press(move |_| {
                submit(idx);
              }))
            })),
        ),
      )
  }
}
