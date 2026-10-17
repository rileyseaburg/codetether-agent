//! roles for portable theme configuration.

use crate::Theme;
use ratatui::style::{Color, Modifier, Style};

impl Theme {
    /// Resolve the terminal style for a conversation role.
    pub fn get_role_style(&self, role: &str) -> Style {
        let color = match role {
            "user" => self.user_color.to_color(),
            "assistant" => self.assistant_color.to_color(),
            "system" | "developer" => self.system_color.to_color(),
            "tool" => self.tool_color.to_color(),
            "error" => self.error_color.to_color(),
            _ => Color::White,
        };
        Style::default().fg(color).add_modifier(Modifier::BOLD)
    }
}
