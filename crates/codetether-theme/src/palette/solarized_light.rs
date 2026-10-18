//! solarized light terminal palette.
use super::ColorPalette;
use ratatui::style::Color;
impl ColorPalette {
    /// Create a solarized light theme color palette
    /// # Arguments
    /// None.
    /// # Returns
    /// The solarized light semantic palette.
    /// # Examples
    /// ```
    /// let _palette = codetether_theme::ColorPalette::solarized_light();
    /// ```
    pub fn solarized_light() -> Self {
        Self {
            user_message: Color::Rgb(133, 153, 0),       // green
            assistant_message: Color::Rgb(42, 161, 152), // cyan
            system_message: Color::Rgb(181, 137, 0),     // yellow
            error: Color::Rgb(220, 50, 47),              // red
            timestamp: Color::Rgb(131, 148, 150),        // base1
            border: Color::Rgb(88, 110, 117),            // base00
            code_block: Color::Rgb(147, 161, 161),       // base1
            tool_message: Color::Rgb(211, 54, 130),      // magenta
            text: Color::Rgb(88, 110, 117),              // base00
            background: Color::Rgb(253, 246, 227),       // base3
        }
    }
}
