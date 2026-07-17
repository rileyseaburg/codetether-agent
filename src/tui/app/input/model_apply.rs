use crate::tui::app::session_runtime::SessionSlot;
use crate::tui::app::state::App;

pub(super) fn selected(app: &mut App, slot: &mut SessionSlot) {
    if let Some(session) = slot.borrow_mut() {
        crate::tui::app::model_picker::apply_selected_model(app, session);
    }
    slot.refresh_view();
}
