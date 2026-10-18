//! marketing terminal palette.
use super::ColorPalette;
use ratatui::style::Color;
impl ColorPalette {
    /// Marketing site inspired theme - dark with cyan accents
    /// # Arguments
    /// None.
    /// # Returns
    /// The marketing semantic palette.
    /// # Examples
    /// ```
    /// let _palette = codetether_theme::ColorPalette::marketing();
    /// ```
    pub fn marketing() -> Self {
        Self {
            user_message: Color::Rgb(6, 182, 212),       // cyan-400
            assistant_message: Color::Rgb(34, 211, 238), // cyan-300
            system_message: Color::Rgb(250, 204, 21),    // yellow-400
            error: Color::Rgb(248, 113, 113),            // red-400
            timestamp: Color::Rgb(107, 114, 128),        // gray-500
            border: Color::Rgb(31, 41, 55),              // gray-800
            code_block: Color::Rgb(75, 85, 99),          // gray-600
            tool_message: Color::Rgb(232, 121, 249),     // fuchsia-400
            text: Color::Rgb(229, 231, 235),             // gray-200
            background: Color::Rgb(3, 7, 18),            // gray-950
        }
    }
}
