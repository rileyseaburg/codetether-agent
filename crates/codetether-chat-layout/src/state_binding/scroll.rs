//! AppState scroll adapter using existing fields and operations.
/// Declare the existing scroll methods on the local view state.
/// # Usage
/// ```text
/// codetether_chat_layout::impl_app_chat_scroll!(AppState);
/// ```
#[macro_export]
macro_rules! impl_app_chat_scroll {
    ($state:ty) => {
        impl $state {
            pub fn scroll_up(&mut self, amount: usize) {
                if let Some(deficit) = self.chat_viewport.scroll_up(amount) {
                    self.history_page
                        .request_older(deficit, self.messages.len());
                }
            }

            pub fn scroll_down(&mut self, amount: usize) {
                self.chat_viewport.scroll_down(amount);
            }

            /// Set sentinel value — clamped to actual content height at render time.
            /// Re-engages [`codetether_chat_layout::ChatScrollState::chat_auto_follow`] so subsequent session events
            /// keep the user pinned to the latest output.
            pub fn scroll_to_bottom(&mut self) {
                self.chat_viewport.scroll_to_bottom();
            }

            pub fn scroll_to_top(&mut self) {
                self.chat_viewport.scroll_to_top();
            }

            pub fn set_chat_max_scroll(&mut self, max_scroll: usize) {
                self.chat_viewport.set_chat_max_scroll(max_scroll);
            }
        }
    };
}
