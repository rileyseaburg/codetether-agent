//! Satisfaction wording and selection styles, separate from popup geometry.

use ratatui::{
    style::Stylize,
    text::{Line, Span},
};

pub(super) fn lines(yes: bool) -> Vec<Line<'static>> {
    vec![
        Line::from("Are you satisfied with the answer?"),
        Line::from("Goal work stays paused until you select Yes."),
        Line::from(vec![
            choice("[ Yes ]", yes),
            "    ".into(),
            choice("[ No ]", !yes),
        ]),
        Line::from("←/→ or Tab to select · Enter to confirm · Y/N".dim()),
    ]
}

fn choice(text: &'static str, selected: bool) -> Span<'static> {
    if selected {
        text.cyan().bold()
    } else {
        text.dim()
    }
}
