//! AppState scroll_page adapter using existing fields and operations.
/// Declare the existing scroll_page methods on the local view state.
/// # Usage
/// ```text
/// codetether_chat_layout::impl_app_chat_paging!(AppState);
/// ```
#[macro_export]
macro_rules! impl_app_chat_paging {
    ($state:ty) => {
        impl $state {
            pub(super) fn manual_chat_scroll(&self) -> usize {
                self.chat_viewport.manual_chat_scroll()
            }

            pub(crate) fn page_up(&mut self) {
                self.scroll_up(self.history_page.page_step());
            }

            pub(crate) fn page_down(&mut self) {
                self.scroll_down(self.history_page.page_step());
            }
        }
    };
}
