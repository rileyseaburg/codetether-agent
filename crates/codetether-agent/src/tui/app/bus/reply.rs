//! Native active-task slot binding for shared remote replies.
use crate::tui::app::state::App;
pub fn emit(app: &mut App, status: &str, error: Option<String>) {
    codetether_bus_view::reply::emit(&mut app.state.worker.active_remote_task, status, error);
}
