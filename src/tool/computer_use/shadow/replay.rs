//! Apply logical state only for accepted posts; preserve partial failure evidence.
use super::types::{Event, Plan, State};
use anyhow::Result;

#[derive(Debug, Default)]
pub(super) struct Outcome {
    pub queued: usize,
    pub error: Option<String>,
}
pub(super) fn execute(
    plan: &Plan,
    state: &mut State,
    mut post: impl FnMut(&Event) -> Result<()>,
) -> Outcome {
    let mut outcome = Outcome::default();
    for event in &plan.events {
        if let Err(error) = post(event) {
            outcome.error = Some(format!("{error:#}"));
            break;
        }
        *state = event.after;
        outcome.queued += 1;
    }
    outcome
}
