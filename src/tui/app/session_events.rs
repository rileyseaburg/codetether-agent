//! TUI adapter for session runtime events.

mod context;
mod errors;
mod flow;
mod lifecycle;
mod pipeline;
mod text;
mod text_dispatch;
mod retention;
mod tools;
mod usage;

#[cfg(test)]
mod tests;

pub use usage::attach_usage_to_last_completion_message;

use crate::session::{Session, SessionEvent};
use crate::tui::app::state::App;
use crate::tui::worker_bridge::TuiWorkerBridge;

pub async fn handle_session_event(
    app: &mut App,
    session: &mut Session,
    worker_bridge: &Option<TuiWorkerBridge>,
    evt: SessionEvent,
) {
    note_event(app);
    let Some(evt) = pipeline::run(app, session, worker_bridge, evt).await else {
        return flow::stop(app);
    };
    retention::trim(app);
    context::handle_event(app, evt);
}

fn note_event(app: &mut App) {
    app.state.main_last_event_at = Some(std::time::Instant::now());
    if app.state.chat_auto_follow {
        app.state.scroll_to_bottom();
    }
}