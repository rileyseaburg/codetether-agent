//! Frontend chat history and render-cache retention binding.

/// Declare chat trimming for an existing frontend state and scroll sentinel.
///
/// Uses the shared retention limits and clears the same message cache after
/// removing old messages, preserving the frontend's viewport adjustment.
/// # Examples
/// ```text
/// codetether_view_retention::declare_chat_retention!(App, SCROLL_BOTTOM);
/// ```
#[macro_export]
macro_rules! declare_chat_retention {
    ($app:ty, $bottom:path $(,)?) => {
        pub(super) fn trim(app: &mut $app) {
            let state = &mut app.state;
            let cache = &mut state.message_cache;
            $crate::chat::trim(
                &mut state.messages,
                state.history_page.expanded(),
                &mut state.chat_viewport.chat_scroll,
                $bottom,
                || cache.clear(),
            );
        }
    };
}
