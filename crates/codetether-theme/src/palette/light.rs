//! light terminal palette.
use super::ColorPalette;
use ratatui::style::Color;
impl ColorPalette {
    /// Create a new light theme color palette with semantic colors
    /// # Arguments
    /// None.
    /// # Returns
    /// The light semantic palette.
    /// # Examples
    /// ```
    /// let _palette = codetether_theme::ColorPalette::light();
    /// ```
    pub fn light() -> Self {
        Self {
            user_message: Color::Blue,
            assistant_message: Color::Green,
            system_message: Color::Yellow,
            error: Color::Red,
            timestamp: Color::Gray,
            border: Color::Gray,
            code_block: Color::Gray,
            tool_message: Color::Magenta,
            text: Color::Black,
            background: Color::White,
        }
    }
}
