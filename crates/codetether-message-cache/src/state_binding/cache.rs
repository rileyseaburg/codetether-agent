//! AppState cache operations for the shared rendered-line cache.
/// Internal application cache method declaration.
#[doc(hidden)]
#[macro_export]
macro_rules! __app_message_cache {
    ($state:ty) => {
        impl $state {
            pub(crate) fn render_cache_inputs(&self) -> $crate::RenderInputs {
                $crate::RenderInputs {
                    pending_tool: self.tool_timing.pending_tool_started_at.is_some(),
                    messages_len: self.messages.len(),
                    tool_preview_scroll: self.tool_viewport.tool_preview_scroll,
                    processing: self.processing,
                }
            }

            /// Checks whether cached chat lines match the current render inputs.
            ///
            /// `max_width` must be the same panel width used to build the cached
            /// lines. The cache is reusable only when no pending tool timer needs a
            /// live redraw and all render-affecting state matches the stored metadata.
            ///
            /// Returns `true` when callers may take and draw `cached_message_lines`.
            pub(crate) fn is_message_cache_valid(&self, max_width: usize) -> bool {
                let inputs = self.render_cache_inputs();
                self.message_cache.is_message_cache_valid(max_width, inputs)
            }

            /// Takes ownership of cached lines when the cache is valid, avoiding clone.
            ///
            /// Callers must call [`Self::restore_cached_message_lines`] after drawing
            /// to put the lines back. Returns `None` when the cache is stale or empty.
            pub fn take_cached_if_valid(
                &mut self,
                max_width: usize,
            ) -> Option<Vec<$crate::state_binding::Line<'static>>> {
                let inputs = self.render_cache_inputs();
                self.message_cache.take_cached_if_valid(max_width, inputs)
            }

            /// Restores lines that were taken via [`Self::take_cached_if_valid`].
            pub fn restore_cached_message_lines(
                &mut self,
                lines: Vec<$crate::state_binding::Line<'static>>,
            ) {
                self.message_cache.restore_cached_message_lines(lines)
            }

            /// Takes ownership of cached lines and leaves the line buffer empty.
            ///
            /// This avoids cloning when a caller intentionally drains the cache. Cache
            /// metadata is not reset here, so callers should replace or invalidate the
            /// cache before relying on it again.
            #[allow(dead_code)]
            pub(crate) fn take_cached_message_lines(
                &mut self,
            ) -> Vec<$crate::state_binding::Line<'static>> {
                self.message_cache.take_cached_message_lines()
            }

            /// Returns cloned cached lines when the full cache can be reused.
            ///
            /// Prefer [`take_cached_if_valid`] for the zero-clone hot path. This
            /// method exists for callers that need an owned copy without the
            /// take/restore protocol (e.g. the webview renderer).
            pub fn get_or_build_message_lines(
                &mut self,
                max_width: usize,
            ) -> Option<Vec<$crate::state_binding::Line<'static>>> {
                let inputs = self.render_cache_inputs();
                self.message_cache
                    .get_or_build_message_lines(max_width, inputs)
            }

            /// Stores freshly rebuilt chat lines and their validation metadata.
            ///
            /// `lines` must have been rendered for `max_width`. The method records the
            /// current message count, processing flag, and streaming-cache state so a
            /// later frame can decide whether the buffer is still safe to reuse.
            pub(crate) fn store_message_lines(
                &mut self,
                lines: Vec<$crate::state_binding::Line<'static>>,
                max_width: usize,
            ) {
                let inputs = self.render_cache_inputs();
                self.message_cache
                    .store_message_lines(lines, max_width, inputs)
            }
        }
    };
}
