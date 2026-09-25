use crate::tui::app::session_runtime::SessionSlot;
use crate::tui::app::state::App;

#[path = "model_apply_verifier.rs"]
mod verifier;

/// Apply the picker choice to the verifier or the session, per picker target.
pub(super) fn selected(app: &mut App, slot: &mut SessionSlot) {
    if app.state.model_picker_for_verifier {
        verifier::apply_verifier_model(app);
        return;
    }
    if let Some(session) = slot.borrow_mut() {
        crate::tui::app::model_picker::apply_selected_model(app, session);
    }
    slot.refresh_view();
}
