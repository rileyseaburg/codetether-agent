//! Human-readable view names.
use crate::ViewMode;

/// Return the display name of a major terminal view.
/// # Arguments
/// * `mode` - View to label.
/// # Returns
/// Its static human-readable name.
/// # Examples
/// ```
/// use codetether_tui_models::{ViewMode, view_mode_display::view_mode_display_name};
/// assert_eq!(view_mode_display_name(ViewMode::Model), "Model Picker");
/// ```
pub fn view_mode_display_name(mode: ViewMode) -> &'static str {
    match mode {
        ViewMode::Chat => "Chat",
        ViewMode::Sessions => "Sessions",
        ViewMode::Swarm => "Swarm",
        ViewMode::Ralph => "Ralph",
        ViewMode::Bus => "Bus Log",
        ViewMode::Subagents => "Subagents",
        ViewMode::Model => "Model Picker",
        ViewMode::Settings => "Settings",
        ViewMode::Lsp => "LSP",
        ViewMode::Rlm => "RLM",
        ViewMode::Latency => "Latency",
        ViewMode::Transport => "Transport",
        ViewMode::Protocol => "Protocol",
        ViewMode::FilePicker => "File Picker",
        ViewMode::Inspector => "Inspector",
        ViewMode::Audit => "Audit",
        ViewMode::Git => "Git",
        ViewMode::AuditLoop => "Audit Loop",
        ViewMode::Editor => "Editor",
    }
}
