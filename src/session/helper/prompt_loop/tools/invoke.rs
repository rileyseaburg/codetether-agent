//! Approval-aware execution of one normalized tool call.

#[path = "invoke_approved.rs"]
mod approved;
use super::{super::Runner, call::Call, outcome::Outcome};
use crate::session::helper::tool_approval;

/// Applies approval policy and executes a normalized tool call.
pub(super) async fn execute(runner: &mut Runner<'_>, call: &Call) -> Outcome {
    let input = super::super::super::runtime::enrich_tool_input_for_turn(
        &call.input,
        &runner.workspace.cwd,
        runner.session,
        &runner.lease_owner,
    );
    let started = super::super::super::persist::before_tool(runner.session).await;
    let (input, blocked, warnings) = if let Some(events) = &runner.events {
        tool_approval::gate(&runner.workspace.cwd, events, &call.id, &call.name, input)
            .await
            .into_checked_parts()
    } else {
        (input, None, Vec::new())
    };
    let tuple = match blocked {
        Some(blocked) => blocked,
        None => approved::run(runner, call, &input, started).await,
    };
    let tuple = tool_approval::annotate(tuple, warnings);
    super::outcome::render(runner, call, &input, started, tuple).await
}
