//! Agent-bus publication for tool request and response events.

use super::{super::Runner, call::Call, outcome::Outcome};

/// Publishes an agent-bus tool request.
pub(super) fn request(runner: &Runner<'_>, step: usize, call: &Call) {
    let Some(bus) = &runner.session.bus else {
        return;
    };
    bus.handle(&runner.session.agent).send_with_correlation(
        format!("agent.{}.tool.request", runner.session.agent),
        crate::bus::BusMessage::ToolRequest {
            request_id: call.id.clone(),
            agent_id: runner.session.agent.clone(),
            tool_name: call.name.clone(),
            arguments: call.input.clone(),
            step,
        },
        Some(runner.progress.turn_id.clone()),
    );
}

/// Publishes compact and full agent-bus tool responses.
pub(super) fn response(runner: &Runner<'_>, step: usize, call: &Call, outcome: &Outcome) {
    let Some(bus) = &runner.session.bus else {
        return;
    };
    let handle = bus.handle(&runner.session.agent);
    let output = super::super::super::live_bus::compact_tool(&outcome.rendered);
    handle.send_with_correlation(
        format!("agent.{}.tool.response", runner.session.agent),
        crate::bus::BusMessage::ToolResponse {
            request_id: call.id.clone(),
            agent_id: runner.session.agent.clone(),
            tool_name: call.name.clone(),
            result: output.clone(),
            success: outcome.success,
            step,
        },
        Some(runner.progress.turn_id.clone()),
    );
    handle.send_with_correlation(
        format!("agent.{}.tool.output", runner.session.agent),
        crate::bus::BusMessage::ToolOutputFull {
            agent_id: runner.session.agent.clone(),
            tool_name: call.name.clone(),
            output,
            success: outcome.success,
            step,
        },
        Some(runner.progress.turn_id.clone()),
    );
}
