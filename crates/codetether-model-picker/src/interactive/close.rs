//! Picker close and selected-model application.
use super::{Session, View};
/// Close the model picker and restore chat.
/// # Arguments
/// * `view` - Picker and shell state.
/// # Returns
/// Completes after clearing the target and restoring the chat view.
/// # Examples
/// ```no_run
/// # fn example<V: codetether_model_picker::interactive::View>(view: &mut V) {
/// codetether_model_picker::interactive::close(view);
/// # }
/// ```
pub fn close<V: View>(view: &mut V) {
    view.picker().close_model_picker();
    view.picker().model_picker_target_model = None;
    view.return_to_chat();
}
/// Apply the selected model preference and close the picker.
/// # Arguments
/// * `view` - Picker and shell state.
/// * `session` - Active session model preference.
/// # Returns
/// Completes after updating the model when selected and restoring chat.
/// # Examples
/// ```no_run
/// # fn example<V: codetether_model_picker::interactive::View, S: codetether_model_picker::interactive::Session>(view: &mut V, session: &mut S) {
/// codetether_model_picker::interactive::apply(view, session);
/// # }
/// ```
pub fn apply<V: View, S: Session>(view: &mut V, session: &mut S) {
    if let Some(model) = view.picker().selected_model().map(str::to_owned) {
        session.set_model(model.to_string());
        view.set_status(format!("Model set: {model}"));
    } else {
        view.set_status("No model selected".to_string());
    }
    close(view);
}
