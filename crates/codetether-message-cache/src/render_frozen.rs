//! Taking stable prefixes while rebuilding streaming suffixes.
use crate::{RenderInputs, RenderedLineCache};
use ratatui::text::Line;
impl RenderedLineCache {
    /// Takes the cached prefix that is unaffected by streaming output.
    ///
    /// `max_width` must match the width used to render the cached lines. The
    /// prefix is unavailable while a pending tool timer is active, because that
    /// timer changes every frame. Returns `None` when there is no reusable
    /// prefix or when the cache metadata no longer matches the current state.
    pub fn take_frozen_prefix(
        &mut self,
        max_width: usize,
        inputs: RenderInputs,
    ) -> Option<Vec<Line<'static>>> {
        if inputs.pending_tool
            || self.cached_frozen_len == 0
            || self.cached_messages_len != inputs.messages_len
            || self.cached_tool_preview_scroll != inputs.tool_preview_scroll
            || self.cached_max_width != max_width
        {
            return None;
        }
        let mut frozen = std::mem::take(&mut self.cached_message_lines);
        frozen.truncate(self.cached_frozen_len);
        if frozen.is_empty() {
            None
        } else {
            Some(frozen)
        }
    }

    /// Stores rebuilt chat lines and records the reusable prefix length.
    ///
    /// `frozen_len` is the count of leading rendered lines that can be cloned
    /// on the next streaming frame before rebuilding the live suffix.
    pub fn store_message_lines_with_frozen(
        &mut self,
        lines: Vec<Line<'static>>,
        max_width: usize,
        frozen_len: usize,
        inputs: RenderInputs,
    ) {
        self.cached_frozen_len = frozen_len;
        self.store_message_lines(lines, max_width, inputs);
    }
}
