//! RLM progress telemetry events.

use crate::session::{RlmCompletion, RlmProgressEvent};
use crate::tui::app::state::App;

pub(crate) fn rlm_progress(app: &mut App, progress: RlmProgressEvent) {
    app.state.context_health.note_rlm_progress(&progress);
    app.state.status = format!(
        "RLM {} {}/{}",
        progress.status, progress.iteration, progress.max_iterations
    );
}

pub(crate) fn rlm_complete(app: &mut App, done: RlmCompletion) {
    app.state.context_health.note_rlm_complete(&done);
    app.state.status = format!("RLM complete: {:?}", done.outcome);
}
