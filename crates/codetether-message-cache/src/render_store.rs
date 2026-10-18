//! Recording render metadata and invalidating stale cached content.
use crate::{RenderInputs, RenderedLineCache};
use ratatui::text::Line;
impl RenderedLineCache {
    /// Stores freshly rebuilt chat lines and their validation metadata.
    ///
    /// `lines` must have been rendered for `max_width`. The method records the
    /// current message count, processing flag, and streaming-cache state so a
    /// later frame can decide whether the buffer is still safe to reuse.
    pub fn store_message_lines(
        &mut self,
        lines: Vec<Line<'static>>,
        max_width: usize,
        inputs: RenderInputs,
    ) {
        self.cached_message_lines = lines;
        self.cached_messages_len = inputs.messages_len;
        self.cached_max_width = max_width;
        self.cached_tool_preview_scroll = inputs.tool_preview_scroll;
        self.cached_streaming_current = true;
        self.cached_processing = inputs.processing;
    }
}

impl RenderedLineCache {
    /// Clear lines and prefix metadata while preserving other validation fields.
    pub fn clear(&mut self) {
        self.cached_message_lines.clear();
        self.cached_messages_len = 0;
        self.cached_frozen_len = 0;
    }
    /// Mark the streaming suffix stale after text changes.
    pub fn invalidate_streaming(&mut self) {
        self.cached_streaming_current = false;
    }
}
