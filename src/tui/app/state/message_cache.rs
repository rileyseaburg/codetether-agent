//! Message-line cache for the TUI chat renderer.
//!
//! The chat view stores fully rendered [`Line`] values so frames can skip
//! wrapping and styling work when the message list, panel width, streaming
//! preview text, and processing state are unchanged. The cache is deliberately
//! invalidated while a tool is pending because the tool panel displays elapsed
//! time and must refresh even when the underlying messages are stable.

use ratatui::text::Line;

mod frozen;

impl super::AppState {
    /// Checks whether cached chat lines match the current render inputs.
    ///
    /// `max_width` must be the same panel width used to build the cached
    /// lines. The cache is reusable only when no pending tool timer needs a
    /// live redraw and all render-affecting state matches the stored metadata.
    ///
    /// Returns `true` when callers may take and draw `cached_message_lines`.
    pub(crate) fn is_message_cache_valid(&self, max_width: usize) -> bool {
        // A pending tool renders a live "running (N.Ns)" spinner; force a
        // rebuild every frame so its elapsed time keeps ticking.
        self.pending_tool_started_at.is_none()
            && self.cached_messages_len == self.messages.len()
            && self.cached_max_width == max_width
            && self.cached_tool_preview_scroll == self.tool_preview_scroll
            && self.cached_streaming_snapshot.as_deref() == Some(&self.streaming_text)
            && self.cached_processing == self.processing
    }

    /// Takes ownership of cached lines when the cache is valid, avoiding clone.
    ///
    /// Callers must call [`Self::restore_cached_message_lines`] after drawing
    /// to put the lines back. Returns `None` when the cache is stale or empty.
    pub fn take_cached_if_valid(&mut self, max_width: usize) -> Option<Vec<Line<'static>>> {
        if self.is_message_cache_valid(max_width) && !self.cached_message_lines.is_empty() {
            Some(std::mem::take(&mut self.cached_message_lines))
        } else {
            None
        }
    }

    /// Restores lines that were taken via [`Self::take_cached_if_valid`].
    pub fn restore_cached_message_lines(&mut self, lines: Vec<Line<'static>>) {
        self.cached_message_lines = lines;
    }

    /// Takes ownership of cached lines and leaves the line buffer empty.
    ///
    /// This avoids cloning when a caller intentionally drains the cache. Cache
    /// metadata is not reset here, so callers should replace or invalidate the
    /// cache before relying on it again.
    #[allow(dead_code)]
    pub(crate) fn take_cached_message_lines(&mut self) -> Vec<Line<'static>> {
        self.cached_message_lines.drain(..).collect()
    }

    /// Returns cloned cached lines when the full cache can be reused.
    ///
    /// Prefer [`take_cached_if_valid`] for the zero-clone hot path. This
    /// method exists for callers that need an owned copy without the
    /// take/restore protocol (e.g. the webview renderer).
    pub fn get_or_build_message_lines(&mut self, max_width: usize) -> Option<Vec<Line<'static>>> {
        if self.is_message_cache_valid(max_width) && !self.cached_message_lines.is_empty() {
            Some(self.cached_message_lines.clone())
        } else {
            None
        }
    }

    /// Stores freshly rebuilt chat lines and their validation metadata.
    ///
    /// `lines` must have been rendered for `max_width`. The method records the
    /// current message count, processing flag, and streaming text snapshot so a
    /// later frame can decide whether the buffer is still safe to reuse.
    pub(crate) fn store_message_lines(&mut self, lines: Vec<Line<'static>>, max_width: usize) {
        self.cached_message_lines = lines;
        self.cached_messages_len = self.messages.len();
        self.cached_max_width = max_width;
        self.cached_tool_preview_scroll = self.tool_preview_scroll;
        self.cached_streaming_snapshot = if self.processing {
            Some(self.streaming_text.clone())
        } else {
            None
        };
        self.cached_processing = self.processing;
    }
}
