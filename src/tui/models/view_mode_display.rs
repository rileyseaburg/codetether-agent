//! Human-readable display names for each [`ViewMode`].

use crate::tui::models::ViewMode;

pub fn view_mode_display_name(mode: ViewMode) -> &'static str {
    match mode {
        ViewMode::Chat => "Chat",
        ViewMode::Sessions => "Sessions",
        ViewMode::Swarm => "Swarm",
        ViewMode::Ralph => "Ralph",
        ViewMode::Bus => "Bus Log",
        ViewMode::Model => "Model Picker",
        ViewMode::Settings => "Settings",
        ViewMode::Lsp => "LSP",
        ViewMode::Rlm => "RLM",
        ViewMode::Latency => "Latency",
        ViewMode::Protocol => "Protocol",
        ViewMode::FilePicker => "File Picker",
        ViewMode::Inspector => "Inspector",
    }
}

pub fn view_mode_shortcut_hint(mode: ViewMode) -> &'static str {
    match mode {
        ViewMode::Chat => "Default (Esc)",
        ViewMode::Sessions => "/sessions",
        ViewMode::Swarm => "Ctrl+W",
        ViewMode::Ralph => "/ralph",
        ViewMode::Bus => "Ctrl+L",
        ViewMode::Model => "Ctrl+M",
        ViewMode::Settings => "/settings",
        ViewMode::Lsp => "/lsp",
        ViewMode::Rlm => "/rlm",
        ViewMode::Latency => "/latency",
        ViewMode::Protocol => "Ctrl+P",
        ViewMode::FilePicker => "Ctrl+O",
        ViewMode::Inspector => "/inspector",
    }
}
