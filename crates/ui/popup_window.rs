use freya::prelude::*;

#[derive(PartialEq)]
pub struct PopupWindow {
  element: Element,
}

impl PopupWindow {
  pub fn new(element: impl IntoElement) -> Self {
    Self {
      element: element.into_element(),
    }
  }
}

impl App for PopupWindow {
  fn render(&self) -> impl IntoElement {
    let theme = use_init_theme(dark_theme);

    rect()
      .expanded()
      .background(theme.read().colors.background)
      .child(self.element.clone())
  }
}
