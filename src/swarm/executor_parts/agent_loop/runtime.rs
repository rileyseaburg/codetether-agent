//! Application operations called by the shared sub-agent turn driver.
use super::{control, limits, output, request, state::State, thinking, tools};
use anyhow::Result;
use async_trait::async_trait;
use codetether_provider_core::{CompletionResponse, Message};
use codetether_swarm_core::{
    agent_loop::{AgentRuntime, RequestOutcome, ToolCall},
    agent_retry::AgentLoopExit,
};
#[async_trait]
impl AgentRuntime for State {
    async fn ready(&mut self) -> Result<()> {
        control::ready(self).await
    }
    fn apply_input(&mut self) -> bool {
        control::apply_input(self)
    }
    fn limit_reached(&self) -> Option<AgentLoopExit> {
        limits::reached(self)
    }
    fn advance(&mut self) {
        codetether_swarm_core::agent_loop::advance_turn(&mut self.steps, &mut self.messages);
    }
    fn steps(&self) -> usize {
        self.steps
    }
    async fn request(&mut self) -> Result<RequestOutcome> {
        request::execute(self).await
    }
    fn normalize(&self, response: CompletionResponse) -> CompletionResponse {
        crate::session::helper::markup::normalize_textual_tool_calls(response, &self.tools)
    }
    fn publish_thinking(&self, text: &str) {
        thinking::publish(self, text);
    }
    fn record_output(&mut self, text: &str) {
        output::record(self, text);
    }
    fn append_message(&mut self, message: Message) {
        self.messages.push(message);
    }
    async fn execute_tools(&mut self, calls: Vec<ToolCall>) -> Result<bool> {
        tools::execute(self, calls).await
    }
}
