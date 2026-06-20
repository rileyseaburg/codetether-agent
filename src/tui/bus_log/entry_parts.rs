//! Shared bus entry conversion result.

use ratatui::style::Color;

pub(super) struct EntryParts {
    pub(super) kind: String,
    pub(super) summary: String,
    pub(super) detail: String,
    pub(super) kind_color: Color,
}

impl EntryParts {
    pub(super) fn new(
        kind: impl Into<String>,
        summary: impl Into<String>,
        detail: impl Into<String>,
        kind_color: Color,
    ) -> Self {
        Self {
            kind: kind.into(),
            summary: summary.into(),
            detail: detail.into(),
            kind_color,
        }
    }
}
