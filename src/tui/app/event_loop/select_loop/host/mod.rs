//! Native application bindings for shared reactor dispatch.
mod after;
mod events;
mod notices;
mod ticks;
use crate::tui::app::event_loop::SelectArgs;
pub(super) struct Native<'a, 'b>(pub &'a mut SelectArgs<'b>);
impl codetether_tui_reactor::dispatch::State for Native<'_, '_> {
    type Snapshot = codetether_tui_tick::Snapshot;
    fn snapshot(&self) -> Self::Snapshot {
        super::dirty::capture(self.0.app)
    }
    fn needs_redraw(&mut self) -> &mut bool {
        &mut self.0.app.state.needs_redraw
    }
}
