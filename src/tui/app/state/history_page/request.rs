//! History pager initialization and background request dispatch.

use crate::provider::Message;

use super::types::Request;
use super::{HistoryPageState, anchor, load};

impl HistoryPageState {
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
        self.loading = false;
        self.exhausted = !has_older || self.boundary.is_empty();
        self.expanded = false;
        self.pending_rewind = 0;
        self.pending_old_lines = None;
        self.pending_old_scroll = 0;
        self.pending_render_rewind = 0;
        while self.rx.try_recv().is_ok() {}
    }

    pub(crate) fn request_older(&mut self, rewind: usize) -> bool {
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
