//! Session-targeted and session-free model-picker initialization.
use super::{View, refresh};
use codetether_model_discovery::Registry;
use std::sync::Arc;
/// Open the picker and target the active session model.
/// # Arguments
/// * `view` - Picker and shell state.
/// * `model` - Current session model, copied into the selection target.
/// * `registry` - Provider registry for background refresh, if available.
/// # Returns
/// Completes after selection and refresh setup; absent providers clear the catalog.
/// # Examples
/// ```no_run
/// # fn example<V: codetether_model_picker::interactive::View, R: codetether_model_discovery::Registry + Send + 'static>(view: &mut V) {
/// codetether_model_picker::interactive::open_for_session::<V, R>(view, None, None);
/// # }
/// ```
pub fn open_for_session<V: View, R: Registry + Send + 'static>(
    view: &mut V,
    model: Option<String>,
    registry: Option<&Arc<R>>,
) {
    view.picker().open_model_picker();
    view.set_status("Model picker".to_string());
    view.show_picker();
    let picker = view.picker();
    picker.model_picker_target_model = model;
    if let Some(current_model) = picker.model_picker_target_model.as_deref()
        && let Some(index) = picker
            .filtered_models()
            .iter()
            .position(|model| *model == current_model)
    {
        picker.selected_model_index = index;
    }
    refresh::start(view, registry, true);
}
/// Open the picker without a session model or catalog reset on missing providers.
/// # Arguments
/// * `view` - Picker and shell state.
/// * `registry` - Optional provider registry for background refresh.
/// # Returns
/// Completes after opening and reporting refresh/provider availability.
/// # Examples
/// ```no_run
/// # fn example<V: codetether_model_picker::interactive::View, R: codetether_model_discovery::Registry + Send + 'static>(view: &mut V) {
/// codetether_model_picker::interactive::open_without_session::<V, R>(view, None);
/// # }
/// ```
pub fn open_without_session<V: View, R: Registry + Send + 'static>(
    view: &mut V,
    registry: Option<&Arc<R>>,
) {
    view.picker().open_model_picker();
    view.set_status("Model picker".to_string());
    view.show_picker();
    refresh::start(view, registry, false);
}
