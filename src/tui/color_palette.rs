use ratatui::style::Color;

/// Centralized semantic color system for consistent theming across TUI components
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ColorPalette {
    /// Color for user messages
    pub user_message: Color,
    /// Color for assistant messages
    pub assistant_message: Color,
    /// Color for system messages
    pub system_message: Color,
    /// Color for error states
    pub error: Color,
    /// Color for timestamps
    pub timestamp: Color,
    /// Color for borders and dividers
    pub border: Color,
    /// Color for code blocks
    pub code_block: Color,
    /// Color for tool messages
    pub tool_message: Color,
    /// Default text color
    pub text: Color,
    /// Background color
    pub background: Color,
}

impl ColorPalette {
    /// Create a new dark theme color palette with semantic colors
    pub fn dark() -> Self {
        Self {
            user_message: Color::Cyan,
            assistant_message: Color::Green,
            system_message: Color::Yellow,
            error: Color::Red,
            timestamp: Color::DarkGray,
            border: Color::Cyan,
            code_block: Color::DarkGray,
            tool_message: Color::Magenta,
            text: Color::White,
            background: Color::Black,
        }
    }

    /// Create a new light theme color palette with semantic colors
    pub fn light() -> Self {
        Self {
            user_message: Color::DarkBlue,
            assistant_message: Color::DarkGreen,
            system_message: Color::DarkYellow,
            error: Color::DarkRed,
            timestamp: Color::Gray,
            border: Color::DarkGray,
            code_block: Color::Gray,
            tool_message: Color::DarkMagenta,
            text: Color::Black,
            background: Color::White,
        }
    }

    /// Create a solarized dark theme color palette
    pub fn solarized_dark() -> Self {
        Self {
            user_message: Color::Rgb(133, 153, 0),      // green
            assistant_message: Color::Rgb(42, 161, 152), // cyan
            system_message: Color::Rgb(181, 137, 0),    // yellow
            error: Color::Rgb(220, 50, 47),             // red
            timestamp: Color::Rgb(101, 123, 131),       // base01
            border: Color::Rgb(131, 148, 150),          // base1
            code_block: Color::Rgb(88, 110, 117),       // base03
            tool_message: Color::Rgb(211, 54, 130),     // magenta
            text: Color::Rgb(238, 232, 213),            // base2
            background: Color::Rgb(0, 43, 54),          // base03
        }
    }

    /// Create a solarized light theme color palette
    pub fn solarized_light() -> Self {
        Self {
            user_message: Color::Rgb(133, 153, 0),      // green
            assistant_message: Color::Rgb(42, 161, 152), // cyan
            system_message: Color::Rgb(181, 137, 0),    // yellow
            error: Color::Rgb(220, 50, 47),             // red
            timestamp: Color::Rgb(131, 148, 150),       // base1
            border: Color::Rgb(88, 110, 117),           // base00
            code_block: Color::Rgb(147, 161, 161),      // base1
            tool_message: Color::Rgb(211, 54, 130),     // magenta
            text: Color::Rgb(88, 110, 117),             // base00
            background: Color::Rgb(253, 246, 227),      // base3
        }
    }

    /// Marketing site inspired theme - dark with cyan accents
    pub fn marketing() -> Self {
        Self {
            user_message: Color::Rgb(6, 182, 212),      // cyan-400
            assistant_message: Color::Rgb(34, 211, 238), // cyan-300
            system_message: Color::Rgb(250, 204, 21),   // yellow-400
            error: Color::Rgb(248, 113, 113),           // red-400
            timestamp: Color::Rgb(107, 114, 128),       // gray-500
            border: Color::Rgb(31, 41, 55),             // gray-800
            code_block: Color::Rgb(75, 85, 99),         // gray-600
            tool_message: Color::Rgb(232, 121, 249),    // fuchsia-400
            text: Color::Rgb(229, 231, 235),            // gray-200
            background: Color::Rgb(3, 7, 18),           // gray-950
        }
    }

    /// Get color based on message role
    pub fn get_message_color(&self, role: &str) -> Color {
        match role.to_lowercase().as_str() {
            "user" => self.user_message,
            "assistant" => self.assistant_message,
            "system" => self.system_message,
            "tool" => self.tool_message,
            "error" => self.error,
            _ => self.text,
        }
    }
}

impl Default for ColorPalette {
    fn default() -> Self {
        Self::marketing()
    }
}
