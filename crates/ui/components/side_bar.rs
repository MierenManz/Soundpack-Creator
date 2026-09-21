use freya::prelude::*;

use crate::Route;

fn sidebar_link(route: Route) -> impl IntoElement {
  let theme = use_theme();
  let name = route.pretty();

  Link::new(route).child(
    SideBarItem::new()
      .child(name)
      .background(theme.read().colors.active),
  )
}

#[derive(PartialEq)]
pub struct SideBar;

impl SideBar {
  pub fn new() -> Self {
    Self
  }
}

impl Component for SideBar {
  fn render(&self) -> impl IntoElement {
    let theme = use_theme();

    rect()
      .background(theme.read().colors.surface_tertiary)
      .width(Size::px(175.))
      .height(Size::fill())
      .spacing(16.)
      .padding((0., 8.))
      .child(sidebar_link(Route::MainPage))
      .child(sidebar_link(Route::PacksPage))
      .child(sidebar_link(Route::SfxPage))
      .child(sidebar_link(Route::TagsPage))
      .child(sidebar_link(Route::SettingsPage))
  }
}
