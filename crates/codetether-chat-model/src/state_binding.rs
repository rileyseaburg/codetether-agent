//! AppState streaming adapter using existing fields and operations.
/// Declare the existing streaming methods on the local view state.
/// # Usage
/// ```text
/// codetether_chat_model::impl_app_streaming!(AppState);
/// ```
#[macro_export]
macro_rules! impl_app_streaming {
    ($state:ty) => {
        impl $state {
            pub fn begin_streaming(&mut self) {
                self.streaming.begin_streaming();
            }
            pub fn record_streaming_chars(&mut self, len: usize) {
                self.streaming.record_streaming_chars(len);
            }
            /// Replace cumulative reply text and invalidate its rendered suffix.
            pub fn replace_streaming_text(&mut self, text: ::std::string::String) {
                self.streaming.replace_streaming_text(text);
                self.message_cache.invalidate_streaming();
            }
            /// Clear reply text and invalidate its rendered suffix only when it changed.
            pub fn clear_streaming_text(&mut self) {
                if self.streaming.clear_streaming_text() {
                    self.message_cache.invalidate_streaming();
                }
            }
            pub fn streaming_tok_per_sec(&self) -> Option<f64> {
                self.streaming.streaming_tok_per_sec()
            }
        }
    };
}
