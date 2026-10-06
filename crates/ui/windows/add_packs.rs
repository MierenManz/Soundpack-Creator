use freya::prelude::*;

use crate::components::Searchbar;

#[derive(PartialEq)]
pub struct AddSoundPack;

impl Component for AddSoundPack {
  fn render(&self) -> impl IntoElement {
    let tmp_test_data = vec!["Max Verstappen", "Lewis Hamilton", "Haaland", "Steinway"];

    let theme = use_theme();
    let name = use_state(|| String::with_capacity(10));
    let picture: ImageSource =
      ("Placeholder", include_bytes!("../placeholder.jpg")).into();
    // let picture = use_state(|| Some());

    let metadata = rect()
      .width(Size::flex(2.))
      .height(Size::percent(100.))
      .direction(Direction::Vertical)
      .spacing(8.)
      .child(
        // Picture thing
        rect()
          .width(Size::px(112.))
          .height(Size::px(112.))
          .border(Border::new().width(1.).fill(theme.read().colors.border))
          .rounded()
          .child(ImageViewer::new(picture).corner_radius(CornerRadius::new_all(8.))),
      )
      .child(
        //
        rect()
          .width(Size::percent(100.))
          .height(Size::flex(1.))
          .border(Border::new().width(1.).fill(theme.read().colors.border))
          .rounded(),
      );

    let sound_effect_table = rect()
      .width(Size::flex(3.))
      .height(Size::percent(100.))
      .rounded()
      .border(Border::new().width(1.).fill(theme.read().colors.border))
      .child(Searchbar::new(tmp_test_data).placeholder("Add Sound effect"));

    let main_container = rect()
      .content(Content::Flex)
      .horizontal()
      .spacing(16.)
      .child(metadata)
      .child(sound_effect_table);

    let buttons = rect();

    rect().expanded().cross_align(Alignment::Center).child(
      rect()
        .padding(Gaps::new_symmetric(16., 0.))
        .width(Size::percent(80.))
        .height(Size::percent(100.))
        .spacing(8.)
        // TODO: Translate
        .child(
          Input::new(name)
            .width(Size::percent(100.))
            .placeholder("Name"),
        )
        .child(main_container)
        .child(buttons),
    )
  }
}
