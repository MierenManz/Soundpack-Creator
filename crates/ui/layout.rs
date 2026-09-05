use crate::Route;
use crate::components::SideBar;
use crate::components::TopBar;
use freya::prelude::*;
use freya::router::*;

#[derive(PartialEq)]
pub struct PrimaryLayout;

impl Component for PrimaryLayout {
  fn render(&self) -> impl IntoElement {
    let theme = use_theme();

    let sidebar = SideBar::new();

    let page = rect()
      .background(theme.read().colors.background)
      .width(Size::flex(1.))
      .child(Outlet::<Route>::new());

    let main_container = rect()
      .horizontal()
      .expanded()
      .content(Content::Flex)
      .main_align(Alignment::End)
      .child(page)
      .child(sidebar);

    let top_bar = TopBar::new();

    rect().child(top_bar).child(main_container)
  }
}
