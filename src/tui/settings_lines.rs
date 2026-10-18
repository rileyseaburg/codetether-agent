//! Application projection for the shared settings panel.
use crate::tui::app::{
    settings::{
        self,
        access_mode::{current_access_mode, label},
    },
    state::AppState,
};
pub(super) fn view(s: &AppState) -> codetether_settings_view::SettingsView {
    codetether_settings_view::SettingsView {
        selected_settings_index: s.selected_settings_index,
        auto_apply_edits: s.auto_apply_edits,
        allow_network: s.allow_network,
        slash_autocomplete: s.slash_autocomplete,
        use_worktree: s.use_worktree,
        access_mode: label(current_access_mode()).to_string(),
        provider_labels: [
            settings::bedrock_service_tier_label().to_string(),
            settings::bedrock_thinking_effort_label().to_string(),
            settings::codex_thinking_effort_label(),
            settings::openrouter_thinking_effort_label(),
        ],
    }
}
#[cfg(test)]
pub(super) fn settings_lines(s: &AppState) -> Vec<ratatui::text::Line<'static>> {
    codetether_settings_view::settings_lines(view(s))
}
