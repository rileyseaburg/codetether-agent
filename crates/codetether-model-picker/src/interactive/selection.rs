//! Route selected models to the session or independent verifier.
use super::{Slot, View};
/// Apply the picker choice to the verifier or the session, per picker target.
/// Apply the model choice to its configured target.
/// # Arguments
/// * `view` — Picker state and presentation callbacks.
/// * `slot` — Active session ownership and view refresh.
/// * `set_verifier` — Callback updating the process's verifier preference.
/// # Returns
/// Updates the selected target; session selections also refresh the slot view.
/// # Examples
/// ```no_run
/// # fn example(view: &mut impl codetether_model_picker::interactive::View, slot: &mut impl codetether_model_picker::interactive::Slot) {
/// codetether_model_picker::interactive::selected(view, slot, |_| {});
/// # }
/// ```
pub fn selected<V: View, S: Slot>(
    view: &mut V,
    slot: &mut S,
    set_verifier: impl FnOnce(Option<&str>),
) {
    if view.picker().model_picker_for_verifier {
        super::verifier::apply_verifier_model(view, set_verifier);
        return;
    }
    if let Some(session) = slot.borrow_mut() {
        super::apply(view, session);
    }
    slot.refresh_view();
}
