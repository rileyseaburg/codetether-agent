//! dark terminal palette.
use super::ColorPalette;
use ratatui::style::Color;
impl ColorPalette {
    /// Create a new dark theme color palette with semantic colors
    /// # Arguments
    /// None.
    /// # Returns
    /// The dark semantic palette.
    /// # Examples
    /// ```
    /// let _palette = codetether_theme::ColorPalette::dark();
    /// ```
    pub fn dark() -> Self {
        Self {
            user_message: Color::Cyan,
            assistant_message: Color::Green,
            system_message: Color::Yellow,
            error: Color::Red,
            timestamp: Color::Gray,
            border: Color::Cyan,
            code_block: Color::Gray,
            tool_message: Color::Magenta,
            text: Color::White,
            background: Color::Black,
        }
    }
}
