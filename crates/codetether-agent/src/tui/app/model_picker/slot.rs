//! Native session-slot bindings for shared model selection.
use crate::session::Session;
use crate::tui::app::session_runtime::SessionSlot;
pub(super) struct SelectionSlot<'a>(pub &'a mut SessionSlot);
impl codetether_model_picker::interactive::Slot for SelectionSlot<'_> {
    type Session = Session;
    fn borrow_mut(&mut self) -> Option<&mut Session> {
        self.0.borrow_mut()
    }
    fn refresh_view(&mut self) {
        self.0.refresh_view();
    }
}
