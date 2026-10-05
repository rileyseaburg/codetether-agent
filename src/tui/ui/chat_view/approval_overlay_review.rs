//! Complete reviewer findings for scrollable approval details.

mod style;
use crate::review::ReviewVerdict;
use ratatui::{style::Stylize, text::Line};

/// Include every finding without reducing the command viewport's height.
pub(super) fn lines(review: Option<&Option<ReviewVerdict>>) -> Vec<Line<'static>> {
    match review {
        None => Vec::new(),
        Some(None) => vec![
            Line::default(),
            Line::from("Reviewer · inspecting…".yellow().bold()),
            Line::from("reading touched files and running read-only checks"),
        ],
        Some(Some(verdict)) => {
            let mut lines = vec![
                Line::default(),
                Line::from(
                    format!("Reviewer · {}", verdict.outcome.label())
                        .fg(style::color_for(verdict.outcome))
                        .bold(),
                ),
            ];
            lines.extend(style::lines_for(verdict));
            lines
        }
    }
}
