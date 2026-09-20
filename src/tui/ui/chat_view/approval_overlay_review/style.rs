//! Colour and line composition for a rendered reviewer verdict.

use ratatui::{style::Color, text::Line};

use crate::review::{ReviewOutcome, ReviewVerdict};

pub(super) fn color_for(outcome: ReviewOutcome) -> Color {
    match outcome {
        ReviewOutcome::Approve => Color::Green,
        ReviewOutcome::RequestChanges => Color::Red,
        ReviewOutcome::Escalate => Color::Magenta,
    }
}

pub(super) fn lines_for(verdict: &ReviewVerdict) -> Vec<Line<'static>> {
    let mut lines = vec![Line::raw(verdict.reason.clone())];
    lines.extend(
        verdict
            .findings
            .iter()
            .map(|finding| Line::raw(format!("• {finding}"))),
    );
    lines
}
