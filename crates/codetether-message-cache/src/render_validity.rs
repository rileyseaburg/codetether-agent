//! Comparing cached metadata with current render inputs.
use crate::{RenderInputs, RenderedLineCache};
impl RenderedLineCache {
    /// Checks whether cached chat lines match the current render inputs.
    ///
    /// `max_width` must be the same panel width used to build the cached
    /// lines. The cache is reusable only when no pending tool timer needs a
    /// live redraw and all render-affecting state matches the stored metadata.
    ///
    /// Returns `true` when callers may take and draw `cached_message_lines`.
    pub fn is_message_cache_valid(&self, max_width: usize, inputs: RenderInputs) -> bool {
        // A pending tool renders a live "running (N.Ns)" spinner; force a
        // rebuild every frame so its elapsed time keeps ticking.
        !inputs.pending_tool
            && self.cached_messages_len == inputs.messages_len
            && self.cached_max_width == max_width
            && self.cached_tool_preview_scroll == inputs.tool_preview_scroll
            && (!inputs.processing || self.cached_streaming_current)
            && self.cached_processing == inputs.processing
    }
}
