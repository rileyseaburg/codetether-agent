//! Builds the text rows displayed in the Settings panel.

use ratatui::text::Line;

use crate::SettingsView;

use super::provider_lines::provider_lines;
use super::{access_mode_line::access_mode_line, setting_line::setting_line};

/// Build owned text rows for the current settings and selection.
/// # Arguments
/// * `s` - Values and labels supplied by the application; labels move into rows.
/// # Returns
/// Settings, provider options and keyboard hints in display order.
/// # Examples
/// ```
/// let view = codetether_settings_view::SettingsView {
///     selected_settings_index: 0, auto_apply_edits: false, allow_network: false,
///     slash_autocomplete: true, use_worktree: false, access_mode: "ask".into(),
///     provider_labels: ["default".into(), "medium".into(), "default".into(), "default".into()],
/// };
/// assert!(!codetether_settings_view::settings_lines(view).is_empty());
/// ```
pub fn settings_lines(s: SettingsView) -> Vec<Line<'static>> {
    let idx = s.selected_settings_index;
    let mut lines = vec![
        Line::from("Settings"),
        Line::from(""),
        setting_line("Edit auto-apply", s.auto_apply_edits, idx == 0),
        Line::from("  Automatically confirms pending edit/multiedit previews in the TUI."),
        Line::from(""),
        setting_line("Network access", s.allow_network, idx == 1),
        Line::from("  Allows sandboxed bash commands in this TUI session to use network access."),
        Line::from(""),
        setting_line("Slash autocomplete", s.slash_autocomplete, idx == 2),
        Line::from("  Enables Tab completion for slash commands in the composer."),
        Line::from(""),
        setting_line("Worktree isolation", s.use_worktree, idx == 3),
        Line::from("  Runs agent work in a git worktree branch, auto-merged on success."),
        Line::from(""),
        access_mode_line(idx == 4, s.access_mode),
        Line::from("  Cycles tool access: ask -> approve -> full (Enter to change)."),
        Line::from(""),
    ];
    lines.extend(provider_lines(idx, s.provider_labels));
    lines.extend([
        Line::from("Controls:"),
        Line::from("  - Up / Down selects a setting"),
        Line::from("  - Enter toggles or cycles the selected setting"),
        Line::from("  - Esc returns to chat"),
    ]);
    lines
}
