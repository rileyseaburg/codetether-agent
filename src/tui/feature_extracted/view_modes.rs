//! View mode display names, shortcuts, and help text

/// ViewMode is defined in src/tui/app/state.rs. This module provides
/// display helpers for all view modes.

// ALL_VIEW_MODES + view_mode_display_name + shortcut_hint + help_rows + compact_summary

const ALL_VIEW_MODES: [ViewMode; 9] = [
    ViewMode::Chat,
    ViewMode::Swarm,
    ViewMode::Ralph,
    ViewMode::BusLog,
    ViewMode::Protocol,
    ViewMode::SessionPicker,
    ViewMode::ModelPicker,
    ViewMode::AgentPicker,
    ViewMode::FilePicker,
];

fn view_mode_display_name(mode: ViewMode) -> &'static str {
    match mode {
        ViewMode::Chat => "Chat",
        ViewMode::Swarm => "Swarm",
        ViewMode::Ralph => "Ralph",
        ViewMode::BusLog => "Bus Log",
        ViewMode::Protocol => "Protocol",
        ViewMode::SessionPicker => "Session Picker",
        ViewMode::ModelPicker => "Model Picker",
        ViewMode::AgentPicker => "Agent Picker",
        ViewMode::FilePicker => "File Picker",
    }
}

fn view_mode_shortcut_hint(mode: ViewMode) -> &'static str {
    match mode {
        ViewMode::Chat => "Default (Esc from any overlay)",
        ViewMode::Swarm => "Ctrl+S / /view / F2",
        ViewMode::Ralph => "/ralph",
        ViewMode::BusLog => "Ctrl+L / F4 / /buslog",
        ViewMode::Protocol => "Ctrl+P / /protocol",
        ViewMode::SessionPicker => "/sessions",
        ViewMode::ModelPicker => "Ctrl+M / /model",
        ViewMode::AgentPicker => "Ctrl+A / /agent",
        ViewMode::FilePicker => "Ctrl+O / /file",
    }
}

fn view_mode_help_rows() -> Vec<String> {
    ALL_VIEW_MODES
        .into_iter()
        .map(|mode| {
            format!(
                "{:<14} {}",
                view_mode_display_name(mode),
                view_mode_shortcut_hint(mode)
            )
        })
        .collect()
}

fn view_mode_compact_summary() -> String {
    ALL_VIEW_MODES
        .into_iter()
        .map(view_mode_display_name)
        .collect::<Vec<_>>()
        .join(" | ")
}

