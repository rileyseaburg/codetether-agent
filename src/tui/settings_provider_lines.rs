//! Provider-specific rows for the TUI Settings panel.

use ratatui::text::Line;

use super::rows::value_line;

pub(super) fn provider_lines(selected: usize) -> Vec<Line<'static>> {
    vec![
        value_line(
            "Bedrock service tier",
            crate::tui::app::settings::bedrock_service_tier_label().to_string(),
            selected == 5,
        ),
        Line::from("  Cycles Claude on Bedrock service_tier: default -> standard -> priority."),
        Line::from(""),
        value_line(
            "Bedrock thinking effort",
            crate::tui::app::settings::bedrock_thinking_effort_label().to_string(),
            selected == 6,
        ),
        Line::from("  Cycles adaptive thinking effort: medium -> low -> high."),
        Line::from(""),
        value_line(
            "OpenAI Codex thinking effort",
            crate::tui::app::settings::codex_thinking_effort_label(),
            selected == 7,
        ),
        Line::from("  Cycles default -> none -> low -> medium -> high -> xhigh -> max."),
        Line::from(""),
    ]
}
