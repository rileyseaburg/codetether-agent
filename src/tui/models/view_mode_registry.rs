//! Canonical list of every [`ViewMode`] variant.

use crate::tui::models::ViewMode;

pub const ALL_VIEW_MODES: &[ViewMode] = &[
    ViewMode::Chat,
    ViewMode::Sessions,
    ViewMode::Swarm,
    ViewMode::Ralph,
    ViewMode::Bus,
    ViewMode::Model,
    ViewMode::Settings,
    ViewMode::Lsp,
    ViewMode::Rlm,
    ViewMode::Latency,
    ViewMode::Protocol,
    ViewMode::FilePicker,
    ViewMode::Inspector,
    ViewMode::Audit,
];
