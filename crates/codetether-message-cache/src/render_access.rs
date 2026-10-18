//! Taking, cloning and restoring owned rendered lines.
use crate::{RenderInputs, RenderedLineCache};
use ratatui::text::Line;
impl RenderedLineCache {
    /// Takes ownership of cached lines when the cache is valid, avoiding clone.
    ///
    /// Callers must call [`Self::restore_cached_message_lines`] after drawing
    /// to put the lines back. Returns `None` when the cache is stale or empty.
    pub fn take_cached_if_valid(
        &mut self,
        max_width: usize,
        inputs: RenderInputs,
    ) -> Option<Vec<Line<'static>>> {
        if self.is_message_cache_valid(max_width, inputs) && !self.cached_message_lines.is_empty() {
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
    pub fn take_cached_message_lines(&mut self) -> Vec<Line<'static>> {
        std::mem::take(&mut self.cached_message_lines)
    }

    /// Returns cloned cached lines when the full cache can be reused.
    ///
    /// Prefer [`Self::take_cached_if_valid`] for the zero-clone hot path. This
    /// method exists for callers that need an owned copy without the
    /// take/restore protocol (e.g. the webview renderer).
    pub fn get_or_build_message_lines(
        &mut self,
        max_width: usize,
        inputs: RenderInputs,
    ) -> Option<Vec<Line<'static>>> {
        if self.is_message_cache_valid(max_width, inputs) && !self.cached_message_lines.is_empty() {
            Some(self.cached_message_lines.clone())
        } else {
            None
        }
    }
}
