//! Cached catalog hydration and background refresh startup.
use super::View;
use codetether_model_discovery::Registry;
use std::sync::Arc;
pub(super) fn start<V: View, R: Registry + Send + 'static>(
    view: &mut V,
    registry: Option<&Arc<R>>,
    clear_unavailable: bool,
) {
    if let Some(registry) = registry {
        let cached_count = view.picker().hydrate_models_from_store();
        view.set_status(if cached_count == 0 {
            "Loading models...".to_string()
        } else {
            format!("Model picker: refreshing {cached_count} cached models")
        });
        view.picker().start_model_refresh(Arc::clone(registry));
    } else {
        if clear_unavailable {
            let picker = view.picker();
            picker.available_models.clear();
            picker.model_refresh_in_flight = false;
            picker.model_refresh_rx = None;
        }
        view.set_status("No models available".to_string());
    }
}
