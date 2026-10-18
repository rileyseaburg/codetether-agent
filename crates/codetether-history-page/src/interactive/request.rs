//! History pager initialization and background request dispatch.

use super::HistoryPageState;
use crate::{HistorySource, Request};

impl<S: HistorySource + 'static> HistoryPageState<S> {
    /// Request an older page or coalesce the rewind into a pending request.
    ///
    /// # Arguments
    /// * `rewind` — Requested backward movement in rendered rows.
    /// * `visible_messages` — Current buffer size for the retention limit.
    ///
    /// # Returns
    /// Whether an existing or newly spawned request can satisfy the rewind.
    /// Requires an active Tokio runtime when spawning a new request.
    ///
    /// # Examples
    /// ```no_run
    /// # fn request<S: codetether_history_page::HistorySource + 'static>(
    /// #     pager: &mut codetether_history_page::interactive::HistoryPageState<S>) {
    /// let pending = pager.request_older(10, 100);
    /// # let _ = pending;
    /// # }
    /// ```
    pub fn request_older(&mut self, rewind: usize, visible_messages: usize) -> bool {
        let max_items = codetether_view_retention::CHAT_EXPANDED_MAX_ITEMS;
        if visible_messages.saturating_add(crate::select::PAGE_MESSAGES) > max_items {
            self.exhausted = true;
            self.pending_rewind = 0;
            return false;
        }
        let Some(source_id) = self.source_id.clone() else {
            return false;
        };
        if self.exhausted || self.boundary.is_empty() {
            return false;
        }
        if self.loading {
            self.pending_rewind = self.pending_rewind.saturating_add(rewind);
            return true;
        }
        self.loading = true;
        self.pending_rewind = rewind;
        let request = Request {
            generation: self.generation,
            source_id,
            boundary: self.boundary.clone(),
            depth: self.depth,
            before: self.before,
        };
        let tx = self.tx.clone();
        tokio::spawn(async move {
            let _ = tx.send(crate::load::<S>(request).await);
        });
        true
    }
}
