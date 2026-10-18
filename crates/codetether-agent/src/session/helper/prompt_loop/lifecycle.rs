//! Prompt-loop lifecycle guards around iterative provider and tool steps.

#[path = "lifecycle/begin.rs"]
mod begin;
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
    crate::session::runtime::boundary::ensure_running(&runner.inner.session.id).await?;
    let mut control =
        crate::session::runtime::execution_scope::ExecutionScope::open(&runner.inner.session.id)?;
    let leases = super::super::workspace_coordination::LeaseTurn::open(&runner.inner.lease_owner);
    let result = control.run(steps::run(runner)).await;
    leases.close().await;
    result
}
