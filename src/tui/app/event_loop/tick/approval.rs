//! Native approval queue/display bindings for shared reconciliation.
use crate::tui::app::state::{App, approval_queue};
use codetether_tui_tick::approval::{Host, State};
struct Native<'a>(&'a mut App);
impl Host for Native<'_> {
    fn remove_stale(&mut self) -> usize {
        approval_queue::remove_stale()
    }
    fn remaining(&self) -> usize {
        approval_queue::len()
    }
    fn state(&mut self) -> State<'_> {
        State {
            waiting: &mut self.0.state.approval_waiting,
            preview_scroll: &mut self.0.state.approval_preview_scroll,
            status: &mut self.0.state.status,
        }
    }
}
pub(super) fn reconcile(app: &mut App) -> bool {
    codetether_tui_tick::approval::reconcile(&mut Native(app))
}
#[cfg(test)]
#[path = "approval_tests.rs"]
mod tests;
