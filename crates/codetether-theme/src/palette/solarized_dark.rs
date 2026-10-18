//! solarized dark terminal palette.
use super::ColorPalette;
use ratatui::style::Color;
impl ColorPalette {
    /// Create a solarized dark theme color palette
    /// # Arguments
    /// None.
    /// # Returns
    /// The solarized dark semantic palette.
    /// # Examples
    /// ```
    /// let _palette = codetether_theme::ColorPalette::solarized_dark();
    /// ```
    pub fn solarized_dark() -> Self {
        Self {
            user_message: Color::Rgb(133, 153, 0),       // green
            assistant_message: Color::Rgb(42, 161, 152), // cyan
            system_message: Color::Rgb(181, 137, 0),     // yellow
            error: Color::Rgb(220, 50, 47),              // red
            timestamp: Color::Rgb(101, 123, 131),        // base01
            border: Color::Rgb(131, 148, 150),           // base1
            code_block: Color::Rgb(88, 110, 117),        // base03
            tool_message: Color::Rgb(211, 54, 130),      // magenta
            text: Color::Rgb(238, 232, 213),             // base2
            background: Color::Rgb(0, 43, 54),           // base03
        }
    }
}
