use ratatui::style::{Color, Modifier, Style};
use serde::{Deserialize, Serialize};

/// Theme configuration for the TUI
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Theme {
    /// User message color
    pub user_color: ColorDef,
    /// Assistant message color
    pub assistant_color: ColorDef,
    /// System message color
    pub system_color: ColorDef,
    /// Tool message color
    pub tool_color: ColorDef,
    /// Error message color
    pub error_color: ColorDef,
    /// Border color for main areas
    pub border_color: ColorDef,
    /// Input border color
    pub input_border_color: ColorDef,
    /// Help border color
    pub help_border_color: ColorDef,
    /// Timestamp color
    pub timestamp_color: ColorDef,
    /// Code block border color
    pub code_block_color: ColorDef,
    /// Status bar colors
    pub status_bar_foreground: ColorDef,
    pub status_bar_background: ColorDef,
    /// Background color (terminal default if None)
    pub background: Option<ColorDef>,
}

impl Default for Theme {
    fn default() -> Self {
        Self::marketing()
    }
}

impl Theme {
    /// Dark theme (default)
    pub fn dark() -> Self {
        Self {
            user_color: ColorDef::Named("green".to_string()),
            assistant_color: ColorDef::Named("cyan".to_string()),
            system_color: ColorDef::Named("yellow".to_string()),
            tool_color: ColorDef::Named("magenta".to_string()),
            error_color: ColorDef::Named("red".to_string()),
            border_color: ColorDef::Named("cyan".to_string()),
            input_border_color: ColorDef::Named("white".to_string()),
            help_border_color: ColorDef::Named("yellow".to_string()),
            timestamp_color: ColorDef::Named("darkgray".to_string()),
            code_block_color: ColorDef::Named("darkgray".to_string()),
            status_bar_foreground: ColorDef::Named("black".to_string()),
            status_bar_background: ColorDef::Named("white".to_string()),
            background: None,
        }
    }

    /// Light theme
    pub fn light() -> Self {
        Self {
            user_color: ColorDef::Named("darkgreen".to_string()),
            assistant_color: ColorDef::Named("darkblue".to_string()),
            system_color: ColorDef::Named("darkyellow".to_string()),
            tool_color: ColorDef::Named("darkmagenta".to_string()),
            error_color: ColorDef::Named("darkred".to_string()),
            border_color: ColorDef::Named("darkgray".to_string()),
            input_border_color: ColorDef::Named("black".to_string()),
            help_border_color: ColorDef::Named("darkyellow".to_string()),
            timestamp_color: ColorDef::Named("gray".to_string()),
            code_block_color: ColorDef::Named("gray".to_string()),
            status_bar_foreground: ColorDef::Named("white".to_string()),
            status_bar_background: ColorDef::Named("black".to_string()),
            background: Some(ColorDef::Named("white".to_string())),
        }
    }

    /// Solarized dark theme
    pub fn solarized_dark() -> Self {
        Self {
            user_color: ColorDef::Rgb(133, 153, 0),              // green
            assistant_color: ColorDef::Rgb(42, 161, 152),        // cyan
            system_color: ColorDef::Rgb(181, 137, 0),            // yellow
            tool_color: ColorDef::Rgb(211, 54, 130),             // magenta
            error_color: ColorDef::Rgb(220, 50, 47),             // red
            border_color: ColorDef::Rgb(131, 148, 150),          // base1
            input_border_color: ColorDef::Rgb(238, 232, 213),    // base2
            help_border_color: ColorDef::Rgb(181, 137, 0),       // yellow
            timestamp_color: ColorDef::Rgb(101, 123, 131),       // base01
            code_block_color: ColorDef::Rgb(88, 110, 117),       // base03
            status_bar_foreground: ColorDef::Rgb(0, 43, 54),     // base03
            status_bar_background: ColorDef::Rgb(238, 232, 213), // base2
            background: Some(ColorDef::Rgb(0, 43, 54)),          // base03
        }
    }

    /// Solarized light theme
    pub fn solarized_light() -> Self {
        Self {
            user_color: ColorDef::Rgb(133, 153, 0),              // green
            assistant_color: ColorDef::Rgb(42, 161, 152),        // cyan
            system_color: ColorDef::Rgb(181, 137, 0),            // yellow
            tool_color: ColorDef::Rgb(211, 54, 130),             // magenta
            error_color: ColorDef::Rgb(220, 50, 47),             // red
            border_color: ColorDef::Rgb(88, 110, 117),           // base00
            input_border_color: ColorDef::Rgb(147, 161, 161),    // base1
            help_border_color: ColorDef::Rgb(181, 137, 0),       // yellow
            timestamp_color: ColorDef::Rgb(131, 148, 150),       // base1
            code_block_color: ColorDef::Rgb(147, 161, 161),      // base1
            status_bar_foreground: ColorDef::Rgb(238, 232, 213), // base2
            status_bar_background: ColorDef::Rgb(88, 110, 117),  // base00
            background: Some(ColorDef::Rgb(253, 246, 227)),      // base3
        }
    }

    /// Marketing site inspired theme - dark with cyan accents
    /// Matches the CodeTether marketing site design
    pub fn marketing() -> Self {
        Self {
            // Cyan accent for user (matches marketing site buttons/highlights)
            user_color: ColorDef::Rgb(6, 182, 212),              // cyan-400
            // Soft cyan for assistant
            assistant_color: ColorDef::Rgb(34, 211, 238),        // cyan-300
            // Yellow for system (warnings/notifications)
            system_color: ColorDef::Rgb(250, 204, 21),           // yellow-400
            // Magenta for tools
            tool_color: ColorDef::Rgb(232, 121, 249),            // fuchsia-400
            // Red for errors
            error_color: ColorDef::Rgb(248, 113, 113),           // red-400
            // Gray-800 for borders (subtle, matches marketing cards)
            border_color: ColorDef::Rgb(31, 41, 55),             // gray-800
            // Gray-700 for input borders
            input_border_color: ColorDef::Rgb(55, 65, 81),       // gray-700
            // Cyan for help borders
            help_border_color: ColorDef::Rgb(6, 182, 212),       // cyan-400
            // Gray-500 for timestamps
            timestamp_color: ColorDef::Rgb(107, 114, 128),       // gray-500
            // Gray-600 for code blocks
            code_block_color: ColorDef::Rgb(75, 85, 99),         // gray-600
            // Dark foreground for status bar
            status_bar_foreground: ColorDef::Rgb(17, 24, 39),    // gray-900
            // Cyan background for status bar
            status_bar_background: ColorDef::Rgb(6, 182, 212),   // cyan-400
            // Gray-950 background (near black, matches marketing site)
            background: Some(ColorDef::Rgb(3, 7, 18)),           // gray-950
        }
    }

    /// Get a style for a specific role
    pub fn get_role_style(&self, role: &str) -> Style {
        let color = match role {
            "user" => self.user_color.to_color(),
            "assistant" => self.assistant_color.to_color(),
            "system" => self.system_color.to_color(),
            "tool" => self.tool_color.to_color(),
            "error" => self.error_color.to_color(),
            _ => Color::White,
        };
        Style::default().fg(color).add_modifier(Modifier::BOLD)
    }
}

/// Color definition that can be serialized/deserialized
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ColorDef {
    /// Named color
    Named(String),
    /// RGB color (r, g, b)
    Rgb(u8, u8, u8),
    /// Indexed color (for 256-color terminals)
    Indexed(u8),
}

impl ColorDef {
    /// Convert to ratatui Color
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

impl From<Color> for ColorDef {
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

// Helper functions for creating common colors
impl ColorDef {
    pub fn black() -> Self {
        ColorDef::Named("black".to_string())
    }
    pub fn red() -> Self {
        ColorDef::Named("red".to_string())
    }
    pub fn green() -> Self {
        ColorDef::Named("green".to_string())
    }
    pub fn yellow() -> Self {
        ColorDef::Named("yellow".to_string())
    }
    pub fn blue() -> Self {
        ColorDef::Named("blue".to_string())
    }
    pub fn magenta() -> Self {
        ColorDef::Named("magenta".to_string())
    }
    pub fn cyan() -> Self {
        ColorDef::Named("cyan".to_string())
    }
    pub fn white() -> Self {
        ColorDef::Named("white".to_string())
    }
    pub fn gray() -> Self {
        ColorDef::Named("gray".to_string())
    }
    pub fn dark_gray() -> Self {
        ColorDef::Named("darkgray".to_string())
    }
    pub fn light_gray() -> Self {
        ColorDef::Named("gray".to_string())
    }
}
