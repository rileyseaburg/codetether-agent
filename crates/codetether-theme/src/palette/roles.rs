//! Message-role lookup for semantic colors.
use super::ColorPalette;
use ratatui::style::Color;
impl ColorPalette {
    /// Get color based on message role
    /// # Arguments
    /// * `role` - Case-insensitive message role.
    /// # Returns
    /// The semantic role color, falling back to the palette text color.
    /// # Examples
    /// ```
    /// let palette = codetether_theme::ColorPalette::default();
    /// assert_eq!(palette.get_message_color("unknown"), palette.text);
    /// ```
    pub fn get_message_color(&self, role: &str) -> Color {
        match role.to_lowercase().as_str() {
            "user" => self.user_message,
            "assistant" => self.assistant_message,
            "system" | "developer" => self.system_message,
            "tool" => self.tool_message,
            "error" => self.error,
            _ => self.text,
        }
    }
}
