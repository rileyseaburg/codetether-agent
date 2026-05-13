//! TUI adapter for session runtime events.

mod context;
mod errors;
mod lifecycle;
mod text;
mod text_dispatch;
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
    mut evt: SessionEvent,
) {
    note_event(app);
    evt = match lifecycle::handle_event(app, session, worker_bridge, evt).await {
        Some(evt) => evt,
        None => return,
    };
    evt = match tools::handle_event(app, worker_bridge, evt).await {
        Some(evt) => evt,
        None => return,
    };
    evt = match text_dispatch::handle_event(app, evt) {
        Some(evt) => evt,
        None => return,
    };
    evt = match usage::handle_event(app, evt) {
        Some(evt) => evt,
        None => return,
    };
    evt = match errors::handle_event(app, session, worker_bridge, evt).await {
        Some(evt) => evt,
        None => return,
    };
    context::handle_event(app, evt);
}

fn note_event(app: &mut App) {
    app.state.main_last_event_at = Some(std::time::Instant::now());
    if app.state.chat_auto_follow {
        app.state.scroll_to_bottom();
    }
}
