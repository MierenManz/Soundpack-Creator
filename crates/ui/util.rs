use crate::PopupWindow;
use crate::popup_window::Window;
use freya::prelude::*;
use freya::winit::window::WindowId;

pub fn open_or_focus_window<W: Window>(mut window: Writable<Option<WindowId>>) {
  let mut window_copy = window.clone();
  if let Some(window_id) = *window.read() {
    Platform::get().focus_window(window_id);
    return;
  }

  spawn(async move {
    let window_cfg = WindowConfig::new_app(PopupWindow::new(W::new()))
      .with_title(W::TITLE)
      .with_resizable(false)
      .with_on_close(move |_, _| {
        {
          *window.write() = None
        };
        CloseDecision::Close
      });

    let window_id = Platform::get().launch_window(window_cfg).await;
    *window_copy.write() = Some(window_id);
  });
}
