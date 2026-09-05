use freya::prelude::*;

#[derive(PartialEq)]
pub struct SettingsPage;

impl Component for SettingsPage {
  fn render(&self) -> impl IntoElement {
    rect()
  }
}
