use crate::elements::*;
use crate::open_or_focus_window;
use crate::windows::AddTag;
use freya::prelude::*;

#[derive(PartialEq)]
pub struct TagsPage;

impl Component for TagsPage {
  fn render(&self) -> impl IntoElement {
    let mut delete_mode = use_state(|| false);
    let window = use_state(|| None);

    let tmp_packs_data: [(); 0] = [];

    let add_button = add_button()
      .on_press(move |_| open_or_focus_window::<AddTag>(window.into_writable()))
      // TODO: Translate
      .child("Add Tag");

    let toggle_delete = move |_| {
      delete_mode.toggle();
    };

    let delete_button = delete_button().on_press(toggle_delete).child(
      delete_mode()
        .then_some("Disable Delete mode")
        .unwrap_or("Enable Delete mode"),
    );

    let button_bar = rect()
      .horizontal()
      .padding((0., 8.))
      .spacing(4.)
      .child(add_button)
      .child(delete_button);

    let tags_view = rect().expanded().padding(8.).child(
      ScrollView::new().expanded().children(
        tmp_packs_data
          .iter()
          // TODO: Render propper button
          .map(|_| Button::new().child("Thingychan")),
      ),
    );

    rect()
      .expanded()
      .color(Color::WHITE)
      .child("Please navigate using buttons on the left")
  }
}
