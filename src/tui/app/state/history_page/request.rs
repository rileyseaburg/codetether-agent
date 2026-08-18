//! History pager initialization and background request dispatch.

use super::types::Request;
use super::{HistoryPageState, load};

impl HistoryPageState {
    pub(crate) fn request_older(&mut self, rewind: usize, visible_messages: usize) -> bool {
        let max_items = crate::tui::retained_payload::CHAT_EXPANDED_MAX_ITEMS;
        if visible_messages.saturating_add(super::select::PAGE_MESSAGES) > max_items {
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
        };
        let tx = self.tx.clone();
        tokio::spawn(async move {
            let _ = tx.send(load::run(request).await);
        });
        true
    }
}
