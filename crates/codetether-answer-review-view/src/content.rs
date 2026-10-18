//! Satisfaction wording and selection styles, separate from popup geometry.

use ratatui::{
    style::Stylize,
    text::{Line, Span},
};

/// Build the satisfaction prompt lines.
///
/// # Arguments
/// * `yes` - Whether the Yes choice is highlighted.
///
/// # Returns
/// Owned question, explanation, choice, and keyboard-help lines.
///
/// # Examples
/// ```
/// assert_eq!(codetether_answer_review_view::lines(false).len(), 4);
/// ```
pub fn lines(yes: bool) -> Vec<Line<'static>> {
    vec![
        Line::from("Are you satisfied with the answer?"),
        Line::from("Goal work stays paused until you select Yes."),
        Line::from(vec![
            choice("[ Yes ]", yes),
            "    ".into(),
            choice("[ No ]", !yes),
        ]),
        Line::from("←/→ or Tab · Enter to confirm · Y/N · / for goal/task commands".dim()),
    ]
}

fn choice(text: &'static str, selected: bool) -> Span<'static> {
    if selected {
        text.cyan().bold()
    } else {
        text.dim()
    }
}
