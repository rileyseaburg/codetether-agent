//! Native state bindings for shared single-tool lifecycle ordering.
use super::super::{
    state::{State, ToolCall},
    tool_bus, tool_events, tool_input, tool_output, tool_raw,
};
use codetether_swarm_core::agent_loop::tools::raw::RawResult;
use serde_json::Value;

impl codetether_swarm_core::agent_loop::tools::call::Host for State {
    fn start(&mut self, call: &ToolCall) {
        self.inner.conversation.tool_calls += 1;
        tool_events::started(self, &call.name);
        super::super::super::executor_trace::tool_call(
            self.inner.conversation.steps,
            &call.id,
            &call.name,
            &call.arguments,
        );
    }
    async fn prepare(&self, call: &ToolCall) -> Result<Value, String> {
        tool_input::prepare(self, &call.name, &call.arguments).await
    }
    async fn raw(&self, name: &str, args: Value) -> RawResult {
        tool_raw::execute(self, name, args).await
    }
    fn finished(&self, call: &ToolCall, result: &RawResult) {
        tool_events::finished(
            self,
            &call.name,
            &call.arguments,
            &result.output,
            result.success,
        );
    }
    fn publish(&self, call: &ToolCall, result: &RawResult) {
        tool_bus::publish(self, &call.name, &result.output, result.success);
    }
    async fn render(&self, name: &str, result: &RawResult) -> (String, bool) {
        tool_output::process(self, name, &result.output, result.success).await
    }
}
