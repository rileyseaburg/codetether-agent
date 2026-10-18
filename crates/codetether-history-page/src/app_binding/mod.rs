//! Native application bindings for bounded backward history loading.
mod entry;
mod view;
pub use codetether_chat_model::ChatMessage;
/// Bind an app and state to existing history storage, cache and viewport policy.
/// # Usage
/// ```text
/// codetether_history_page::impl_app_history_page!(App, AppState, Source, clear_cache, SCROLL_BOTTOM);
/// ```
#[macro_export]
macro_rules! impl_app_history_page {
    ($app:ty, $state:ty, $source:ty, $clear:path, $bottom:path) => {
        $crate::__app_history_view!($state, $source, $clear, $bottom);
        $crate::__app_history_entry!($app, $state);
    };
}
