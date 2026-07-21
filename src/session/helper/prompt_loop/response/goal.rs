//! Final-answer routing for active goal continuation.

use super::super::{Runner, state::StepFlow};

pub(super) async fn flow(runner: &mut Runner<'_>) -> StepFlow {
    match crate::session::tasks::runtime::next_message(&runner.session.id).await {
        Ok(Some(message)) => {
            runner.session.add_message(message);
            reset_turn(runner);
            tracing::info!(session_id = %runner.session.id, "Continuing active goal");
            StepFlow::ContinueGoal
        }
        Ok(None) => StepFlow::Finish,
        Err(error) => {
            tracing::warn!(error = %error, "Failed to schedule goal continuation");
            StepFlow::Finish
        }
    }
}

fn reset_turn(runner: &mut Runner<'_>) {
    let progress = &mut runner.progress;
    progress.output.clear();
    progress.validation_retries = 0;
    progress.repeat_guard = Default::default();
    progress.codesearch_misses = 0;
    progress.build_retries = 0;
    progress.native_retries = 0;
    progress.steps_since_write = 0;
    progress.turn_id = uuid::Uuid::new_v4().to_string();
}
