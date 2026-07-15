//! Apply completed background pages while preserving the rendered anchor.

use super::{HistoryPageState, Page};

impl HistoryPageState {
    pub(super) fn take_result(&mut self) -> Option<Result<Page, String>> {
        while let Ok((generation, result)) = self.rx.try_recv() {
            if generation == self.generation {
                return Some(result);
            }
        }
        None
    }

    pub(super) fn accept(
        &mut self,
        page: &Page,
        old_lines: usize,
        old_scroll: usize,
        follow_latest: bool,
    ) {
        self.loading = false;
        self.exhausted = page.exhausted || page.messages.is_empty();
        self.depth = page.depth;
        if !page.boundary.is_empty() {
            self.boundary.clone_from(&page.boundary);
        }
        self.expanded |= !page.messages.is_empty();
        let rewind = std::mem::take(&mut self.pending_rewind);
        if !follow_latest {
            self.pending_old_lines = Some(old_lines);
            self.pending_old_scroll = old_scroll;
            self.pending_render_rewind = if old_scroll == 0 { rewind } else { 0 };
        }
    }

    pub(super) fn fail(&mut self) {
        self.loading = false;
        self.pending_rewind = 0;
    }

    pub(crate) fn expanded(&self) -> bool {
        self.expanded
    }

    pub(super) fn take_anchor(&mut self, total: usize) -> Option<(usize, usize, usize)> {
        let inserted = total.saturating_sub(self.pending_old_lines.take()?);
        let old_scroll = std::mem::take(&mut self.pending_old_scroll);
        let rewind = std::mem::take(&mut self.pending_render_rewind);
        Some((inserted, old_scroll, rewind))
    }
}
