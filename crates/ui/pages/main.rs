use freya::prelude::*;

#[derive(PartialEq)]
pub struct MainPage;

impl Component for MainPage {
  fn render(&self) -> impl IntoElement {
    rect()
      .expanded()
      .color(Color::WHITE)
      // TODO: Translate
      .child("Please navigate using buttons on the left")
  }
}
