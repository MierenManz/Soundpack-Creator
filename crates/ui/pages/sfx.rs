use freya::prelude::*;

#[derive(PartialEq)]
pub struct SfxPage;

impl Component for SfxPage {
  fn render(&self) -> impl IntoElement {
    rect()
  }
}
