//! Shortcut hints for terminal views.
use crate::ViewMode;

/// Return the keyboard or command hint for opening a view.
/// # Arguments
/// * `mode` - View whose shortcut should be shown.
/// # Returns
/// A static shortcut hint.
/// # Examples
/// ```
/// use codetether_tui_models::{ViewMode, view_mode_display::view_mode_shortcut_hint};
/// assert_eq!(view_mode_shortcut_hint(ViewMode::Settings), "/settings");
/// ```
pub fn view_mode_shortcut_hint(mode: ViewMode) -> &'static str {
    match mode {
        ViewMode::Chat => "Default (Esc)",
        ViewMode::Sessions => "/sessions",
        ViewMode::Swarm => "Ctrl+S",
        ViewMode::Ralph => "/ralph",
        ViewMode::Bus => "Ctrl+L",
        ViewMode::Subagents => "/agents",
        ViewMode::Model => "Ctrl+M",
        ViewMode::Settings => "/settings",
        ViewMode::Lsp => "/lsp",
        ViewMode::Rlm => "/rlm",
        ViewMode::Latency => "/latency",
        ViewMode::Transport => "/transport",
        ViewMode::Protocol => "Ctrl+P",
        ViewMode::FilePicker => "/file",
        ViewMode::Inspector => "/inspector",
        ViewMode::Audit => "/audit",
        ViewMode::Git => "/git",
        ViewMode::AuditLoop => "/auditloop",
        ViewMode::Editor => "/edit",
    }
}
