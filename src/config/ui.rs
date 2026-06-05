use crate::config::Config;
use serde::{Deserialize, Serialize};

/// UI/TUI settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiConfig {
    #[serde(default = "default_theme")]
    pub theme: String,
    #[serde(default = "default_true")]
    pub line_numbers: bool,
    #[serde(default = "default_true")]
    pub mouse: bool,
    #[serde(default)]
    pub custom_theme: Option<crate::tui::theme::Theme>,
    #[serde(default = "default_false")]
    pub hot_reload: bool,
}

impl Default for UiConfig {
    fn default() -> Self {
        Self {
            theme: default_theme(),
            line_numbers: true,
            mouse: true,
            custom_theme: None,
            hot_reload: false,
        }
    }
}

impl Config {
    /// Load theme based on configuration.
    pub fn load_theme(&self) -> crate::tui::theme::Theme {
        if let Some(custom) = &self.ui.custom_theme {
            return custom.clone();
        }
        match self.ui.theme.as_str() {
            "marketing" | "default" => crate::tui::theme::Theme::marketing(),
            "dark" => crate::tui::theme::Theme::dark(),
            "light" => crate::tui::theme::Theme::light(),
            "solarized-dark" => crate::tui::theme::Theme::solarized_dark(),
            "solarized-light" => crate::tui::theme::Theme::solarized_light(),
            _ => crate::tui::theme::Theme::marketing(),
        }
    }
}

fn default_theme() -> String {
    "marketing".to_string()
}
fn default_true() -> bool {
    true
}
fn default_false() -> bool {
    false
}
