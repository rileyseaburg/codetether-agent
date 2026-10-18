//! Shared model picker state methods using the native provider registry.
pub use codetether_model_discovery::{ModelRefreshEvent, ModelRefreshSummary};
codetether_model_picker::impl_app_models!(
    super::AppState,
    crate::provider::ProviderRegistry,
    crate::tui::models::ViewMode::Model
);
