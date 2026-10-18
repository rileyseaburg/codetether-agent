//! Native control, tool execution, and history bindings for shared batches.
use super::super::{
    control,
    state::{State, ToolCall},
    tool_one,
};
use codetether_provider_core::Message;
use codetether_swarm_core::agent_loop::tools::Completed;

impl<H: crate::Host> codetether_swarm_core::agent_loop::tools::Host for State<H> {
    async fn ready(&mut self) -> anyhow::Result<()> {
        control::ready(self).await
    }
    async fn execute(&mut self, call: ToolCall) -> Completed {
        tool_one::execute(self, call).await
    }
    fn record(&mut self, message: Message) {
        self.inner.conversation.messages.push(message);
    }
}
