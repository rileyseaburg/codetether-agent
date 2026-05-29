//! Frozen-prefix cache helpers for streaming chat rendering.
//!
//! While an assistant response streams, the beginning of the rendered chat
//! buffer often remains unchanged. These helpers let the renderer reuse that
//! stable prefix and rebuild only the suffix that contains live streaming text.

use ratatui::text::Line;

impl super::super::AppState {
    /// Clones the cached prefix that is unaffected by streaming output.
    ///
    /// `max_width` must match the width used to render the cached lines. The
    /// prefix is unavailable while a pending tool timer is active, because that
    /// timer changes every frame. Returns `None` when there is no reusable
    /// prefix or when the cache metadata no longer matches the current state.
    pub(crate) fn clone_frozen_prefix(&self, max_width: usize) -> Option<Vec<Line<'static>>> {
        if self.pending_tool_started_at.is_some()
            || self.cached_frozen_len == 0
            || self.cached_messages_len != self.messages.len()
            || self.cached_max_width != max_width
        {
            return None;
        }
        let frozen: Vec<Line<'static>> = self
            .cached_message_lines
            .iter()
            .take(self.cached_frozen_len)
            .cloned()
            .collect();
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
    pub(crate) fn store_message_lines_with_frozen(
        &mut self,
        lines: Vec<Line<'static>>,
        max_width: usize,
        frozen_len: usize,
    ) {
        self.cached_frozen_len = frozen_len;
        self.store_message_lines(lines, max_width);
    }
}
