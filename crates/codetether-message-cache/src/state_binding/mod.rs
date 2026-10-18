//! Native render inputs and cache take/restore operations.
mod cache;
mod frozen;
pub use ratatui::text::Line;
/// Bind rendered-line caching to an application's existing state fields.
/// # Usage
/// ```text
/// codetether_message_cache::impl_app_cache!(AppState);
/// ```
#[macro_export]
macro_rules! impl_app_cache {
    ($state:ty) => {
        $crate::__app_message_cache!($state);
        $crate::__app_message_frozen!($state);
    };
}
