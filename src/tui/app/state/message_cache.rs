//! Message line cache for render performance.
//!
//! Caches rendered `Line<'static>` so the TUI can skip full rebuilds
//! when only the streaming preview changed.

use ratatui::text::Line;

impl super::AppState {
    /// Check whether the cached message lines are still valid for the given
    /// width. Returns `true` when the cache can be reused.
    pub(crate) fn is_message_cache_valid(&self, max_width: usize) -> bool {
        self.cached_messages_len == self.messages.len()
            && self.cached_max_width == max_width
            && self.cached_streaming_snapshot.as_deref() == Some(&self.streaming_text)
            && self.cached_processing == self.processing
    }

    /// Returns cached lines when still valid for `max_width`, or `None`.
    pub fn get_or_build_message_lines(&mut self, max_width: usize) -> Option<Vec<Line<'static>>> {
        if self.is_message_cache_valid(max_width) && !self.cached_message_lines.is_empty() {
            Some(self.cached_message_lines.clone())
        } else {
            None
        }
    }

    /// Take ownership of the cached lines, clearing the cache.
    #[allow(dead_code)]
    pub(crate) fn take_cached_message_lines(&mut self) -> Vec<Line<'static>> {
        self.cached_message_lines.drain(..).collect()
    }

    /// Store rebuilt message lines in the cache.
    pub(crate) fn store_message_lines(&mut self, lines: Vec<Line<'static>>, max_width: usize) {
        self.cached_message_lines = lines;
        self.cached_messages_len = self.messages.len();
        self.cached_max_width = max_width;
        self.cached_streaming_snapshot = if self.processing {
            Some(self.streaming_text.clone())
        } else {
            None
        };
        self.cached_processing = self.processing;
    }

    /// Clone only the frozen (non-streaming) prefix of cached lines.
    ///
    /// Returns `None` if there is no cache or the frozen prefix is empty.
    /// Used by the fast-path render that reuses the frozen prefix and
    /// only rebuilds the streaming suffix.
    pub(crate) fn clone_frozen_prefix(&self, max_width: usize) -> Option<Vec<Line<'static>>> {
        if self.cached_frozen_len == 0
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

    /// Store rebuilt message lines along with the frozen-prefix length.
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
