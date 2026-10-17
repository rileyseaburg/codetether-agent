//! color from for portable theme configuration.

use crate::ColorDef;
use ratatui::style::Color;

impl From<Color> for ColorDef {
    /// Preserve a terminal color as serializable configuration.
    fn from(color: Color) -> Self {
        match color {
            Color::Black => ColorDef::Named("black".to_string()),
            Color::Red => ColorDef::Named("red".to_string()),
            Color::Green => ColorDef::Named("green".to_string()),
            Color::Yellow => ColorDef::Named("yellow".to_string()),
            Color::Blue => ColorDef::Named("blue".to_string()),
            Color::Magenta => ColorDef::Named("magenta".to_string()),
            Color::Cyan => ColorDef::Named("cyan".to_string()),
            Color::Gray => ColorDef::Named("gray".to_string()),
            Color::DarkGray => ColorDef::Named("darkgray".to_string()),
            Color::LightRed => ColorDef::Named("lightred".to_string()),
            Color::LightGreen => ColorDef::Named("lightgreen".to_string()),
            Color::LightYellow => ColorDef::Named("lightyellow".to_string()),
            Color::LightBlue => ColorDef::Named("lightblue".to_string()),
            Color::LightMagenta => ColorDef::Named("lightmagenta".to_string()),
            Color::LightCyan => ColorDef::Named("lightcyan".to_string()),
            Color::White => ColorDef::Named("white".to_string()),
            Color::Rgb(r, g, b) => ColorDef::Rgb(r, g, b),
            Color::Indexed(idx) => ColorDef::Indexed(idx),
            _ => ColorDef::Named("white".to_string()),
        }
    }
}
