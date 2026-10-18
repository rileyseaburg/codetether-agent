//! Native entry to shared session/verifier model selection.
use crate::tui::app::{session_runtime::SessionSlot, state::App};
pub(super) fn selected(app: &mut App, slot: &mut SessionSlot) {
    crate::tui::app::model_picker::apply_selection(app, slot);
}
