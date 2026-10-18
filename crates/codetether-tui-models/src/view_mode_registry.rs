//! Canonical list of every [`ViewMode`] variant.

use crate::ViewMode;

/// Every major view in canonical help/selection order.
pub const ALL_VIEW_MODES: &[ViewMode] = &[
    ViewMode::Chat,
    ViewMode::Sessions,
    ViewMode::Swarm,
    ViewMode::Ralph,
    ViewMode::Bus,
    ViewMode::Subagents,
    ViewMode::Model,
    ViewMode::Settings,
    ViewMode::Lsp,
    ViewMode::Rlm,
    ViewMode::Latency,
    ViewMode::Transport,
    ViewMode::Protocol,
    ViewMode::FilePicker,
    ViewMode::Inspector,
    ViewMode::Audit,
    ViewMode::Git,
    ViewMode::AuditLoop,
    ViewMode::Editor,
];
