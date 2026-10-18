//! RLM progress telemetry events.

use super::View;
use codetether_session_events::{RlmCompletion, RlmProgressEvent};

pub fn rlm_progress(view: &mut impl View, progress: RlmProgressEvent) {
    view.health().note_rlm_progress(&progress);
    view.set_status(format!(
        "RLM {} {}/{}",
        progress.status, progress.iteration, progress.max_iterations
    ));
}

pub fn rlm_complete(view: &mut impl View, done: RlmCompletion) {
    view.health().note_rlm_complete(&done);
    view.set_status(format!("RLM complete: {:?}", done.outcome));
}
