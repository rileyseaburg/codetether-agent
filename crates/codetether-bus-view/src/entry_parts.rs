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
        let kind = kind.into();
        let summary = summary.into();
        let detail = detail.into();
        Self {
            kind: codetether_view_retention::bus::kind(&kind),
            summary: codetether_view_retention::bus::summary(&summary),
            detail: codetether_view_retention::bus::detail(&detail, "bus detail"),
            kind_color,
        }
    }
}
