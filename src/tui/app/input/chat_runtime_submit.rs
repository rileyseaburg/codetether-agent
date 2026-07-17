//! Runtime handoff and recovery for one moved TUI session.

use crate::tui::app::session_runtime::{PromptRequest, SessionSlot, TuiSessionHandle};
use crate::tui::app::state::App;

/// Reserve steering state and transfer one prompt request to the runtime.
pub(super) async fn submit(
    app: &mut App,
    slot: &mut SessionSlot,
    runtime: &TuiSessionHandle,
    request: PromptRequest,
) {
    if !runtime.activate_steering(&request.session.id) {
        slot.restore(request.session);
        app.state.status = "Session runtime is busy".to_string();
        return;
    }
    if let Err(request) = runtime.submit(request).await {
        runtime.clear_steering();
        slot.restore(request.session);
        app.state.status = "Session runtime is unavailable".to_string();
    }
}
