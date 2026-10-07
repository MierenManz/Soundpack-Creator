use freya::prelude::*;

pub fn add_button() -> Button {
  Button::new()
    .background(Color::GREEN * 0.75)
    .hover_background(Color::GREEN * 0.75 * 0.75)
    .border_fill(Color::TRANSPARENT)
    .corner_radius((0., 4.))
}

pub fn delete_button() -> Button {
  Button::new()
    .background(Color::GRAY)
    .hover_background(Color::GRAY * 0.75)
    .border_fill(Color::TRANSPARENT)
    .corner_radius((0., 4.))
}
