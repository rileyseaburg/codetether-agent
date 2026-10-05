//! Colour and line composition for a rendered reviewer verdict.

use ratatui::{
    style::Color,
    text::{Line, Text},
};

use crate::review::{ReviewOutcome, ReviewVerdict};

pub(super) fn color_for(outcome: ReviewOutcome) -> Color {
    match outcome {
        ReviewOutcome::Approve => Color::Green,
        ReviewOutcome::RequestChanges => Color::Red,
        ReviewOutcome::Escalate => Color::Magenta,
    }
}

pub(super) fn lines_for(verdict: &ReviewVerdict) -> Vec<Line<'static>> {
    let mut lines = Text::raw(verdict.reason.clone()).lines;
    lines.extend(
        verdict
            .findings
            .iter()
            .flat_map(|finding| Text::raw(format!("• {finding}")).lines),
    );
    lines
}
