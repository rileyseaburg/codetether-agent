//! View-mode color mapping.
use codetether_tui_models::ViewMode;
use ratatui::style::Color;

pub(super) fn named_accent(mode: &ViewMode) -> Color {
    match mode {
        ViewMode::Chat => Color::Cyan,
        ViewMode::Sessions | ViewMode::FilePicker => Color::Green,
        ViewMode::Swarm => Color::Magenta,
        ViewMode::Ralph => Color::Blue,
        ViewMode::Bus | ViewMode::Inspector | ViewMode::Subagents => Color::LightCyan,
        ViewMode::Model | ViewMode::Transport | ViewMode::Editor => Color::LightBlue,
        ViewMode::Settings => Color::Gray,
        ViewMode::Lsp => Color::LightGreen,
        ViewMode::Rlm => Color::LightMagenta,
        ViewMode::Latency | ViewMode::Git => Color::LightRed,
        ViewMode::Protocol => Color::LightYellow,
        ViewMode::Audit | ViewMode::AuditLoop => Color::Yellow,
    }
}
