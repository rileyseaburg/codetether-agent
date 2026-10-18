//! Default channel and cursor state for backward history paging.

use super::HistoryPageState;

impl<S> Default for HistoryPageState<S> {
    fn default() -> Self {
        let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
        Self {
            source: std::marker::PhantomData,
            source_id: None,
            generation: 0,
            boundary: Vec::new(),
            depth: 0,
            before: None,
            loading: false,
            exhausted: true,
            expanded: false,
            viewport_height: 10,
            pending_rewind: 0,
            pending_old_lines: None,
            pending_old_scroll: 0,
            pending_render_rewind: 0,
            tx,
            rx,
        }
    }
}
