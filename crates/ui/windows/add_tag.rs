use crate::popup_window::Window;
use freya::prelude::*;

#[derive(PartialEq)]
pub struct AddTag;

impl Component for AddTag {
  fn render(&self) -> impl IntoElement {
    let theme = use_theme();
    let name = use_state(|| String::with_capacity(10));
    let color = use_state::<Color>(|| (127, 127, 127).into());

    rect()
  }
}

impl Window for AddTag {
  const TITLE: &'static str = "Soundpack Creator - Add new Tag";
  fn new() -> Self {
    Self
  }
}
