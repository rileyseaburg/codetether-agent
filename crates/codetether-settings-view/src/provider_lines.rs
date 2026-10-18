//! Provider-specific rows for the TUI Settings panel.

use ratatui::text::Line;

use super::value_line::value_line;

pub(super) fn provider_lines(selected: usize, labels: [String; 4]) -> Vec<Line<'static>> {
    let [tier, bedrock, codex, openrouter] = labels;
    vec![
        value_line("Bedrock service tier", tier, selected == 5),
        Line::from("  Cycles Claude on Bedrock service_tier: default -> standard -> priority."),
        Line::from(""),
        value_line("Bedrock thinking effort", bedrock, selected == 6),
        Line::from("  Cycles adaptive thinking effort: medium -> low -> high."),
        Line::from(""),
        value_line("OpenAI Codex thinking effort", codex, selected == 7),
        Line::from("  Cycles default -> none -> low -> medium -> high -> xhigh -> max."),
        Line::from(""),
        value_line("OpenRouter thinking effort", openrouter, selected == 8),
        Line::from("  Cycles default -> none -> minimal -> low -> medium -> high -> xhigh -> max."),
        Line::from(""),
    ]
}
