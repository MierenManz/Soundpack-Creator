use crate::PopupWindow;
use crate::windows::AddSoundPack;
use freya::prelude::*;

#[derive(PartialEq)]
pub struct PacksPage;

impl Component for PacksPage {
  fn render(&self) -> impl IntoElement {
    let mut delete_mode = use_state(|| false);
    let mut window = use_state(|| None);

    let button_data: [u8; 0] = [];

    let open_window_cuz_stinky = move |_| {
      if let Some(window_id) = *window.read() {
        Platform::get().focus_window(window_id);
        return;
      }

      spawn(async move {
        let window_cfg = WindowConfig::new_app(PopupWindow::new(AddSoundPack))
          .with_title("Soundpack Creator - Packs")
          .with_resizable(false)
          .with_on_close(move |_, _| {
            *window.write() = None;
            CloseDecision::Close
          });

        let window_id = Platform::get().launch_window(window_cfg).await;
        *window.write() = Some(window_id);
      });
    };

    let add_button = Button::new()
      .background(Color::GREEN * 0.75)
      .hover_background(Color::GREEN * 0.75 * 0.75)
      .border_fill(Color::TRANSPARENT)
      .corner_radius((0., 4.))
      .on_press(open_window_cuz_stinky)
      // TODO: Translate
      .child("Add Soundpack");

    let toggle_delete = move |_| {
      delete_mode.toggle();
    };

    let delete_mode_button = Button::new()
      .background(delete_mode().then_some(Color::GRAY).unwrap_or(Color::RED))
      .hover_background(delete_mode().then_some(Color::GRAY).unwrap_or(Color::RED) * 0.75)
      .border_fill(Color::TRANSPARENT)
      .corner_radius((0., 4.))
      .on_press(toggle_delete)
      .child(
        delete_mode()
          // TODO: Translate
          .then_some("Disable Delete Mode")
          // TODO: Translate
          .unwrap_or("Enable Delete Mode"),
      );

    let button_bar = rect()
      .horizontal()
      .padding((0., 8.))
      .spacing(4.)
      .child(add_button)
      .child(delete_mode_button);

    let pack_window = rect().expanded().padding(8.).child(
      ScrollView::new().expanded().children(
        button_data
          .iter()
          // TODO: Render propper button
          .map(|_| Button::new().child("Thingychan")),
      ),
    );

    rect()
      .expanded()
      .color(Color::WHITE)
      .child(button_bar)
      .child(pack_window)
  }
}
