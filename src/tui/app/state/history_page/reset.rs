//! History pager reset for a newly selected session.

use crate::provider::Message;

use super::{HistoryPageState, anchor};

impl HistoryPageState {
    /// Pin the viewport boundary to an absolute durable sequence.
    pub(crate) fn set_before(&mut self, before: usize) {
        self.before = Some(before);
    }
    pub(crate) fn reset(
        &mut self,
        source_id: String,
        boundary_messages: &[Message],
        depth: usize,
        has_older: bool,
    ) {
        self.generation = self.generation.wrapping_add(1);
        self.source_id = Some(source_id);
        self.boundary = anchor::fingerprints(boundary_messages);
        self.depth = depth;
        self.before = None;
        self.loading = false;
        self.exhausted = !has_older || self.boundary.is_empty();
        self.expanded = false;
        self.pending_rewind = 0;
        self.pending_old_lines = None;
        self.pending_old_scroll = 0;
        self.pending_render_rewind = 0;
        while self.rx.try_recv().is_ok() {}
    }
}
