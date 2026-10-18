use super::signature;
use crate::LoopState;
pub(super) fn record(progress: &mut LoopState, error: &anyhow::Error) {
    let signature = signature::of(error);
    if progress.goal_failure_signature.as_ref() == Some(&signature) {
        progress.goal_failure_repeats += 1;
    } else {
        progress.goal_failure_signature = Some(signature);
        progress.goal_failure_repeats = 1;
    }
}
