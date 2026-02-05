//! Theme utilities for color support detection and validation

use crate::tui::theme::{ColorDef, Theme};

/// Check terminal color support via environment detection
pub fn detect_color_support() -> ColorSupport {
    // Check COLORTERM for truecolor
    if let Ok(colorterm) = std::env::var("COLORTERM") {
        if colorterm == "truecolor" || colorterm == "24bit" {
            return ColorSupport::TrueColor;
        }
    }
    
    // Check TERM for 256 color support
    if let Ok(term) = std::env::var("TERM") {
        if term.contains("256color") || term.contains("256") {
            return ColorSupport::Ansi256;
        }
        if term.contains("color") || term.starts_with("xterm") || term.starts_with("screen") {
            return ColorSupport::Ansi8;
        }
    }
    
    // Default to 8 colors (most common minimum)
    ColorSupport::Ansi8
}

/// Color support levels
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorSupport {
    Monochrome,
    Ansi8,
    Ansi256,
    TrueColor,
}

impl ColorSupport {
    pub fn supports_rgb(&self) -> bool {
        matches!(self, ColorSupport::TrueColor)
    }

    pub fn supports_indexed(&self) -> bool {
        matches!(self, ColorSupport::Ansi256 | ColorSupport::TrueColor)
    }

    pub fn supports_named(&self) -> bool {
        !matches!(self, ColorSupport::Monochrome)
    }
}

/// Validate and adjust theme based on terminal capabilities
pub fn validate_theme(theme: &Theme) -> Theme {
    let support = detect_color_support();
    let mut validated = theme.clone();

    if !support.supports_rgb() {
        // Convert RGB colors to closest named colors
        validated.user_color = fallback_color(&theme.user_color, &support);
        validated.assistant_color = fallback_color(&theme.assistant_color, &support);
        validated.system_color = fallback_color(&theme.system_color, &support);
        validated.tool_color = fallback_color(&theme.tool_color, &support);
        validated.error_color = fallback_color(&theme.error_color, &support);
        validated.border_color = fallback_color(&theme.border_color, &support);
        validated.input_border_color = fallback_color(&theme.input_border_color, &support);
        validated.help_border_color = fallback_color(&theme.help_border_color, &support);
        validated.timestamp_color = fallback_color(&theme.timestamp_color, &support);
        validated.code_block_color = fallback_color(&theme.code_block_color, &support);
        validated.status_bar_foreground = fallback_color(&theme.status_bar_foreground, &support);
        validated.status_bar_background = fallback_color(&theme.status_bar_background, &support);
        
        if let Some(bg) = &theme.background {
            validated.background = Some(fallback_color(bg, &support));
        }
    }

    validated
}

/// Convert colors to fallback based on terminal support
fn fallback_color(color: &ColorDef, support: &ColorSupport) -> ColorDef {
    match color {
        ColorDef::Rgb(r, g, b) => {
            if support.supports_rgb() {
                ColorDef::Rgb(*r, *g, *b)
            } else if support.supports_indexed() {
                // Convert to closest indexed color
                ColorDef::Indexed(rgb_to_ansi256(*r, *g, *b))
            } else {
                // Convert to closest named color
                ColorDef::Named(rgb_to_named(*r, *g, *b))
            }
        }
        ColorDef::Indexed(idx) => {
            if support.supports_indexed() {
                ColorDef::Indexed(*idx)
            } else {
                // Convert to named color
                ColorDef::Named(indexed_to_named(*idx))
            }
        }
        ColorDef::Named(name) => {
            if support.supports_named() {
                ColorDef::Named(name.clone())
            } else {
                // Fallback to safe defaults
                ColorDef::Named("white".to_string())
            }
        }
    }
}

/// Convert RGB to closest ANSI 256-color index
fn rgb_to_ansi256(r: u8, g: u8, b: u8) -> u8 {
    // Standard 256-color conversion
    if r == g && g == b {
        // Grayscale
        if r < 8 {
            16
        } else if r > 248 {
            231
        } else {
            232 + ((r - 8) / 10)
        }
    } else {
        // Color
        16 + (36 * (r / 51)) + (6 * (g / 51)) + (b / 51)
    }
}

/// Convert RGB to closest named color
fn rgb_to_named(r: u8, g: u8, b: u8) -> String {
    // Simple mapping to standard 8 colors
    let colors = [
        ("black", (0, 0, 0)),
        ("red", (128, 0, 0)),
        ("green", (0, 128, 0)),
        ("yellow", (128, 128, 0)),
        ("blue", (0, 0, 128)),
        ("magenta", (128, 0, 128)),
        ("cyan", (0, 128, 128)),
        ("white", (192, 192, 192)),
    ];

    let mut closest = "white";
    let mut min_distance = u32::MAX;

    for (name, (cr, cg, cb)) in colors.iter() {
        let dr = (r as i32 - *cr as i32).unsigned_abs();
        let dg = (g as i32 - *cg as i32).unsigned_abs();
        let db = (b as i32 - *cb as i32).unsigned_abs();
        let distance = dr * dr + dg * dg + db * db;
        
        if distance < min_distance {
            min_distance = distance;
            closest = *name;
        }
    }

    closest.to_string()
}

/// Convert indexed color to named color
fn indexed_to_named(index: u8) -> String {
    match index {
        0..=15 => {
            // Standard 16 colors
            match index {
                0 => "black",
                1 => "red",
                2 => "green",
                3 => "yellow",
                4 => "blue",
                5 => "magenta",
                6 => "cyan",
                7 => "white",
                8 => "darkgray",
                9 => "lightred",
                10 => "lightgreen",
                11 => "lightyellow",
                12 => "lightblue",
                13 => "lightmagenta",
                14 => "lightcyan",
                15 => "lightgray",
                _ => "white",
            }
        }
        _ => "white",
    }.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_support_detection() {
        let support = detect_color_support();
        // Just ensure it doesn't panic
        assert!(matches!(support, ColorSupport::Monochrome | ColorSupport::Ansi8 | ColorSupport::Ansi256 | ColorSupport::TrueColor));
    }

    #[test]
    fn test_fallback_color() {
        let support = ColorSupport::Ansi8;
        let rgb_color = ColorDef::Rgb(255, 0, 0);
        let fallback = fallback_color(&rgb_color, &support);
        assert!(matches!(fallback, ColorDef::Named(_)));
    }
}