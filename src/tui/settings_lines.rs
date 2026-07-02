//! Builds the text rows displayed in the Settings panel.

use ratatui::text::Line;

use crate::tui::app::state::AppState;

use super::rows::{access_mode_line, setting_line, value_line};

pub(super) fn settings_lines(s: &AppState) -> Vec<Line<'static>> {
    let idx = s.selected_settings_index;
    vec![
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
        access_mode_line(idx == 4),
        Line::from("  Cycles tool access: ask -> approve -> full (Enter to change)."),
        Line::from(""),
        value_line(
            "Bedrock service tier",
            crate::tui::app::settings::bedrock_service_tier_label().to_string(),
            idx == 5,
        ),
        Line::from("  Cycles Claude on Bedrock service_tier: default -> standard -> priority."),
        Line::from(""),
        value_line(
            "Bedrock thinking effort",
            crate::tui::app::settings::bedrock_thinking_effort_label().to_string(),
            idx == 6,
        ),
        Line::from("  Cycles adaptive thinking effort: medium -> low -> high."),
        Line::from(""),
        Line::from("Controls:"),
        Line::from("  - Up / Down selects a setting"),
        Line::from("  - Enter toggles or cycles the selected setting"),
        Line::from("  - Esc returns to chat"),
    ]
}
