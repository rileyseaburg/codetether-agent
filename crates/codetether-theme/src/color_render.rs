//! color render for portable theme configuration.

use crate::ColorDef;
use ratatui::style::Color;

impl ColorDef {
    /// Convert the color definition into a terminal color.
    pub fn to_color(&self) -> Color {
        match self {
            ColorDef::Named(name) => match name.to_lowercase().as_str() {
                "black" => Color::Black,
                "red" => Color::Red,
                "green" => Color::Green,
                "yellow" => Color::Yellow,
                "blue" => Color::Blue,
                "magenta" => Color::Magenta,
                "cyan" => Color::Cyan,
                "gray" | "grey" => Color::Gray,
                "darkgray" | "dark_gray" | "darkgrey" => Color::DarkGray,
                "lightgray" | "light_gray" | "lightgrey" => Color::Gray,
                // Dark colors map to base colors (ratatui 0.30 doesn't have Dark* variants)
                "darkred" | "dark_red" => Color::Red,
                "darkgreen" | "dark_green" => Color::Green,
                "darkyellow" | "dark_yellow" => Color::Yellow,
                "darkblue" | "dark_blue" => Color::Blue,
                "darkmagenta" | "dark_magenta" => Color::Magenta,
                "darkcyan" | "dark_cyan" => Color::Cyan,
                // Light colors
                "lightred" | "light_red" => Color::LightRed,
                "lightgreen" | "light_green" => Color::LightGreen,
                "lightyellow" | "light_yellow" => Color::LightYellow,
                "lightblue" | "light_blue" => Color::LightBlue,
                "lightmagenta" | "light_magenta" => Color::LightMagenta,
                "lightcyan" | "light_cyan" => Color::LightCyan,
                "white" => Color::White,
                _ => Color::White,
            },
            ColorDef::Rgb(r, g, b) => Color::Rgb(*r, *g, *b),
            ColorDef::Indexed(idx) => Color::Indexed(*idx),
        }
    }
}
