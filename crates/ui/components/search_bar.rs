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
    let mut visibility_override = use_state(|| false);
    let on_submit = self.on_submit.clone();
    let data = use_state(|| self.data.clone());
    let search_str = use_state(String::new);

    let filtered_data = use_memo::<Vec<String>>(move || {
      let search = search_str.read().as_str().to_lowercase();

      data
        .read()
        .clone()
        .into_iter()
        .filter_map(|x| {
          let formatted = format!("{x}");

          formatted
            .to_lowercase()
            .contains(&search)
            .then_some(formatted)
        })
        .collect()
    });

    let menu_visible =
      use_memo(move || visibility_override() && filtered_data.read().len() > 0);

    let submit_with_str = move |input_str: String| {
      if input_str.len() == 0 {
        return;
      }

      visibility_override.set(false);

      if let Some(func) = on_submit.as_ref() {
        let ret = data
          .read()
          .as_slice()
          .iter()
          .find(|x| input_str == format!("{x}"))
          .map(|x| Item::Existing(x.clone()))
          .unwrap_or_else(|| Item::New(input_str));

        func.call(ret);
      }
    };

    let submit_with_str2 = submit_with_str.clone();

    rect()
      .child(
        rect()
          .horizontal()
          .content(Content::Flex)
          .child(
            Input::new(search_str)
              .on_validate(move |x: InputValidator| {
                visibility_override.set(x.text().len() != 0);
              })
              .on_submit(submit_with_str)
              .width(Size::flex(1.))
              .height(Size::px(32.))
              .placeholder(self.placeholder.clone())
              .corner_radius((6., 0., 0., 6.)),
          )
          .child(
            Button::new()
              .on_press(move |_| {
                visibility_override.set(false);
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
            .on_close(move |_| visibility_override.set(false))
            .children(filtered_data.read().iter().map(|x| {
              let cpy = x.to_string();
              let mut submit = submit_with_str2.clone();
              MenuButton::new()
                .on_press(move |_| submit(cpy.to_string()))
                .child(x.as_str())
                .into_element()
            })),
        ),
      )
  }
}
