//! Native presentation binding for the shared A2A attachment command.
use crate::tui::app::state::App;
pub(super) fn handle(app: &mut App, rest: &str) {
    codetether_a2a_interactive::command::handle(app, rest);
}
