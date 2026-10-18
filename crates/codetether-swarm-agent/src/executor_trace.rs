pub(super) fn tool_call(step: usize, call_id: &str, tool_name: &str, arguments: &str) {
    tracing::info!(target: "codetether_agent::swarm::executor::executor_trace",
        step,
        tool_call_id = %call_id,
        tool = %tool_name,
        "Executing tool"
    );
    tracing::debug!(target: "codetether_agent::swarm::executor::executor_trace",
        tool = %tool_name,
        arguments,
        "Tool call arguments"
    );
}
