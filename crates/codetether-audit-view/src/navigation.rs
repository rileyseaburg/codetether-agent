//! Cursor movement and filter selection.
use crate::{AuditFilter, AuditViewState};

impl AuditViewState {
    /// Move the selection cursor up; saturates at 0.
    pub fn select_prev(&mut self) {
        self.selected = self.selected.saturating_sub(1);
    }

    /// Move the selection cursor down; saturates at `entries.len() - 1`.
    pub fn select_next(&mut self) {
        if self.selected + 1 < self.entries.len() {
            self.selected += 1;
        }
    }

    /// Toggle between [`AuditFilter::Subagents`] and [`AuditFilter::All`].
    pub fn toggle_filter(&mut self) {
        self.filter = match self.filter {
            AuditFilter::Subagents => AuditFilter::All,
            AuditFilter::All => AuditFilter::Subagents,
        };
        self.selected = 0;
    }
}
