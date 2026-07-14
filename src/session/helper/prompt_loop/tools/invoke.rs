//! Approval-aware execution of one normalized tool call.

use super::{super::Runner, call::Call, outcome::Outcome};

/// Applies approval policy and executes a normalized tool call.
pub(super) async fn execute(runner: &mut Runner<'_>, call: &Call) -> Outcome {
    let input = super::super::super::runtime::enrich_tool_input_for_session(
        &call.input,
        &runner.workspace.cwd,
        runner.session,
    );
    let started = super::super::super::persist::before_tool(runner.session).await;
    let (input, blocked) = if let Some(events) = &runner.events {
        super::super::super::tool_approval::gate(events, &call.id, &call.name, input)
            .await
            .into_parts()
    } else {
        (input, None)
    };
    let tuple = match blocked {
        Some(blocked) => blocked,
        None => approved(runner, call, &input, started).await,
    };
    super::outcome::render(runner, call, &input, started, tuple).await
}

async fn approved(
    runner: &Runner<'_>,
    call: &Call,
    input: &serde_json::Value,
    started: std::time::Instant,
) -> super::super::super::tool_policy::ToolTuple {
    let heartbeat = runner.events.as_ref().map(|events| {
        super::super::super::tool_heartbeat::spawn(events, &call.id, &call.name, started)
    });
    let progress = runner
        .events
        .as_ref()
        .map(|events| (events, call.id.as_str()));
    let result = super::super::super::tool_exec::execute_tool(
        &runner.model.registry,
        &call.name,
        input,
        &runner.session.id,
        started,
        progress,
    )
    .await;
    if let Some(heartbeat) = heartbeat {
        heartbeat.abort();
    }
    result
}
