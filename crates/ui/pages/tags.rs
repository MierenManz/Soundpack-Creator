use freya::prelude::*;

#[derive(PartialEq)]
pub struct TagsPage;

impl Component for TagsPage {
  fn render(&self) -> impl IntoElement {
    rect()
      .expanded()
      .color(Color::WHITE)
      .child("Please navigate using buttons on the left")
  }
}
