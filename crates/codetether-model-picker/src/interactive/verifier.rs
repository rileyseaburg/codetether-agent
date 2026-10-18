//! Apply a verifier model choice and close the picker.
use super::View;
/// Apply the verifier choice and return to chat.
/// # Arguments
/// * `view` — Picker state and presentation callbacks.
/// * `set_verifier` — Callback updating the process's verifier preference.
/// # Returns
/// Updates the selected target; session selections also refresh the slot view.
/// # Examples
/// ```no_run
/// # fn example(view: &mut impl codetether_model_picker::interactive::View) {
/// codetether_model_picker::interactive::apply_verifier_model(view, |_| {});
/// # }
/// ```
pub fn apply_verifier_model<V: View>(view: &mut V, set_verifier: impl FnOnce(Option<&str>)) {
    let model = view.picker().selected_model().map(str::to_string);
    set_verifier(model.as_deref());
    view.set_status(match model {
        Some(model) => format!("Verifier model set: {model}"),
        None => "No verifier model selected".to_string(),
    });
    view.picker().close_model_picker();
    view.picker().model_picker_target_model = None;
    view.return_to_chat();
}
