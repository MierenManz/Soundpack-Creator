use freya::prelude::*;

#[derive(PartialEq)]
pub struct TopBar;

impl TopBar {
  pub fn new() -> Self {
    Self
  }
}

impl Component for TopBar {
  fn render(&self) -> impl IntoElement {
    let theme = use_theme();
    // TODO: Make Global & then load initial from config
    let mut local_volume = use_state(|| 0.);
    let mut piped_volume = use_state(|| 0.);

    let slider_bars = rect()
      .padding(Gaps::new(0., 0., 0., 8.))
      .width(Size::flex(1.))
      .child(
        label()
          .color(Color::WHITE)
          // TODO: Translation
          .text(format!("Local Volume ({}%)", (*local_volume.read()))),
      )
      .child(Slider::new(move |x: f64| local_volume.set(x.ceil())).value(*local_volume.read()))
      .child(
        label()
          .color(Color::WHITE) // TODO: Translation
          .text(format!("Piped Volume ({}%)", (*piped_volume.read()))),
      )
      .child(Slider::new(move |x: f64| piped_volume.set(x.ceil())).value(*piped_volume.read()));
    // TODO: Make this linked together with a toggle

    rect()
      .background(theme.read().colors.surface_tertiary)
      .horizontal()
      .content(Content::Flex)
      .width(Size::fill())
      .padding(Gaps::new_all(8.))
      .child(slider_bars)
      .child(rect().background(Color::GREEN).width(Size::px(175.)))
  }
}
