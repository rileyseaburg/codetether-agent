//! AppState model selection and asynchronous refresh bindings.
pub mod navigation;
mod refresh;
pub use anyhow;
pub use codetether_model_discovery::ModelRefreshSummary;
/// Bind model navigation and refresh to the existing registry and model-view variant.
/// # Usage
/// ```text
/// codetether_model_picker::impl_app_models!(AppState, Registry, ViewMode::Model);
/// ```
#[macro_export]
macro_rules! impl_app_models {
    ($state:ty, $registry:ty, $model_mode:path) => {
        $crate::__app_model_navigation!($state);
        $crate::__app_model_refresh!($state, $registry, $model_mode);
    };
}
