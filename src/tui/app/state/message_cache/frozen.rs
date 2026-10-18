//! Frozen-prefix adapters for the shared rendered-line cache.
use ratatui::text::Line;
impl super::super::AppState {
    /// Takes the cached prefix that is unaffected by streaming output.
    ///
    /// `max_width` must match the width used to render the cached lines. The
    /// prefix is unavailable while a pending tool timer is active, because that
    /// timer changes every frame. Returns `None` when there is no reusable
    /// prefix or when the cache metadata no longer matches the current state.
    pub(crate) fn take_frozen_prefix(&mut self, max_width: usize) -> Option<Vec<Line<'static>>> {
        let inputs = self.render_cache_inputs();
        self.message_cache.take_frozen_prefix(max_width, inputs)
    }

    /// Stores rebuilt chat lines and records the reusable prefix length.
    ///
    /// `frozen_len` is the count of leading rendered lines that can be cloned
    /// on the next streaming frame before rebuilding the live suffix.
    pub(crate) fn store_message_lines_with_frozen(
        &mut self,
        lines: Vec<Line<'static>>,
        max_width: usize,
        frozen_len: usize,
    ) {
        let inputs = self.render_cache_inputs();
        self.message_cache
            .store_message_lines_with_frozen(lines, max_width, frozen_len, inputs)
    }
}
