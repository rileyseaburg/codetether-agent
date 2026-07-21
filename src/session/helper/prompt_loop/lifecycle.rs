//! Prompt-loop lifecycle guards around iterative provider and tool steps.

#[path = "lifecycle/begin.rs"]
mod begin;
#[path = "lifecycle/budget.rs"]
mod budget;
#[path = "lifecycle/steps.rs"]
mod steps;

use super::state::Runner;
use crate::session::SessionResult;
use anyhow::Result;

/// Runs provider/tool steps and returns the persisted session result.
///
/// # Errors
///
/// Returns an error from setup, completion, tool execution, or persistence.
pub(crate) async fn run(runner: &mut Runner<'_>) -> Result<SessionResult> {
    let _steering = super::super::steering::RunGuard::open(&runner.session.id);
    let leases = super::super::workspace_coordination::LeaseTurn::open(&runner.lease_owner);
    let result = steps::run(runner).await;
    leases.close().await;
    result
}
