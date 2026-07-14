//! Concurrent completion and collapse sampling loop.

use super::{collapse, completion, state::State};
use crate::swarm::{CollapseController, CollapsePolicy};
use futures::StreamExt;
use tokio::time::{Duration, MissedTickBehavior};

#[path = "collect_cancel.rs"]
mod cancel;

const SAMPLE_SECONDS: u64 = 5;

pub(super) async fn all(state: &mut State<'_>) {
    let enabled = state.executor.config.collapse_enabled
        && state.manager.is_some()
        && state.active_worktrees.len() > 1;
    let mut controller = enabled.then(|| CollapseController::new(CollapsePolicy::default()));
    let mut tick = tokio::time::interval(Duration::from_secs(SAMPLE_SECONDS));
    tick.set_missed_tick_behavior(MissedTickBehavior::Skip);
    tick.tick().await;
    let control = state.executor.control().cloned();
    let mut cancelled = false;
    while !state.handles.is_empty() {
        tokio::select! {
            joined = state.handles.next() => {
                if let Some(joined) = joined { completion::record(state, joined); }
            }
            _ = tick.tick(), if controller.is_some() && !state.active_worktrees.is_empty() => {
                if let Some(controller) = &mut controller { collapse::sample(state, controller).await; }
            }
            _ = cancel::requested(control.as_ref()), if !cancelled => {
                cancelled = true;
                cancel::abort_all(&state.aborts);
            }
        }
    }
}
