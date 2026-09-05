use crate::PrimaryLayout;
use crate::pages::*;
use freya::prelude::*;
use freya::router::*;

#[derive(Routable, Clone, PartialEq)]
#[rustfmt::skip]
pub enum Route {
  #[layout(PrimaryLayout)]
    #[route("/")]
    MainPage,
    #[route("/tags")]
    TagsPage,
    #[route("/sfx")]
    SfxPage,
    #[route("/packs")]
    PacksPage,
    // #[route("/open_pack/:id")]
    // OpenSoundpackPage,
    #[route("/settings")]
    SettingsPage,
}

// TODO: TRANSLATE THISSSSSSS
impl Route {
  pub fn pretty(&self) -> &'static str {
    match self {
      Self::MainPage => "Main Page",
      Self::TagsPage => "Tags",
      Self::SfxPage => "Sound Effects",
      Self::PacksPage => "Sound Packs",
      Self::SettingsPage => "Settings",
    }
  }
}
