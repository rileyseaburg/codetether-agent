//! User bubble border color and dimming.
use codetether_theme::gradient::rgb_supported;
use ratatui::style::{Color, Modifier, Style};

const USER_RGB: (u8, u8, u8) = (80, 200, 120);

pub(super) fn box_color() -> Color {
    if rgb_supported() {
        Color::Rgb(USER_RGB.0, USER_RGB.1, USER_RGB.2)
    } else {
        Color::Green
    }
}

pub(super) fn dim(color: Color) -> Style {
    Style::default().fg(color).add_modifier(Modifier::DIM)
}
