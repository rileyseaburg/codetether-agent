//! History pager reset for a newly selected session.

use codetether_provider_core::Message;

use super::HistoryPageState;
use crate::anchor;

impl<S> HistoryPageState<S> {
    /// Pin the viewport boundary to an absolute durable sequence.
    pub fn set_before(&mut self, before: usize) {
        self.before = Some(before);
    }
    /// Select a source, reset paging cursors and discard obsolete queued results.
    ///
    /// # Arguments
    /// * `source_id` — Session storage identity.
    /// * `boundary_messages` — Visible prefix used to locate the previous page.
    /// * `depth` — Number of messages already loaded.
    /// * `has_older` — Whether the source may contain earlier messages.
    ///
    /// # Returns
    /// Resets this state in place and advances its generation.
    ///
    /// # Examples
    /// ```
    /// let mut pager = codetether_history_page::interactive::HistoryPageState::<()>::default();
    /// pager.reset("session".into(), &[], 0, false);
    /// assert!(!pager.expanded());
    /// ```
    pub fn reset(
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
