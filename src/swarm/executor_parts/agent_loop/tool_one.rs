//! Full lifecycle of one requested sub-agent tool call.

use super::{
    state::{State, ToolCall},
    tool_bus, tool_events, tool_input, tool_output, tool_raw,
};

pub(super) struct Completed {
    pub id: String,
    pub name: String,
    pub output: String,
    pub timed_out: bool,
}

pub(super) async fn execute(state: &mut State, call: ToolCall) -> Completed {
    state.tool_calls += 1;
    tool_events::started(state, &call.name);
    super::super::executor_trace::tool_call(state.steps, &call.id, &call.name, &call.arguments);
    let raw = match tool_input::prepare(state, &call.name, &call.arguments).await {
        Ok(args) => tool_raw::execute(state, &call.name, args).await,
        Err(output) => tool_raw::Result {
            output,
            success: false,
            timed_out: false,
        },
    };
    tool_events::finished(state, &call.name, &call.arguments, &raw.output, raw.success);
    tool_bus::publish(state, &call.name, &raw.output, raw.success);
    let (output, processing_timeout) =
        tool_output::process(state, &call.name, &raw.output, raw.success).await;
    Completed {
        id: call.id,
        name: call.name,
        output,
        timed_out: raw.timed_out || processing_timeout,
    }
}
