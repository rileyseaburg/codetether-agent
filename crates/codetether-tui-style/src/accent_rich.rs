//! View-mode color mapping.
use codetether_tui_models::ViewMode;
use ratatui::style::Color;

pub(super) fn rich_accent(mode: &ViewMode) -> Color {
    match mode {
        ViewMode::Chat => Color::Rgb(0, 229, 255),
        ViewMode::Sessions => Color::Rgb(0, 255, 128),
        ViewMode::Swarm => Color::Rgb(255, 64, 216),
        ViewMode::Ralph => Color::Rgb(100, 80, 255),
        ViewMode::Bus => Color::Rgb(0, 200, 255),
        ViewMode::Subagents => Color::Rgb(0, 255, 180),
        ViewMode::Model => Color::Rgb(80, 160, 255),
        ViewMode::Settings => Color::Rgb(160, 160, 160),
        ViewMode::Lsp => Color::Rgb(80, 255, 160),
        ViewMode::Rlm => Color::Rgb(220, 80, 255),
        ViewMode::Latency => Color::Rgb(255, 100, 80),
        ViewMode::Transport => Color::Rgb(80, 180, 255),
        ViewMode::Protocol => Color::Rgb(255, 220, 60),
        ViewMode::FilePicker => Color::Rgb(60, 220, 120),
        ViewMode::Inspector => Color::Rgb(60, 220, 220),
        ViewMode::Audit => Color::Rgb(255, 180, 0),
        ViewMode::Git => Color::Rgb(255, 80, 80),
        ViewMode::AuditLoop => Color::Rgb(255, 160, 0),
        ViewMode::Editor => Color::Rgb(120, 120, 255),
    }
}
