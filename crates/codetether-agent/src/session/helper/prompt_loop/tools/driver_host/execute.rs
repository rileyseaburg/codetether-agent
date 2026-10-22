//! Native approval-aware tool invocation.
use super::super::super::Runner;
use codetether_prompt_loop::{Call, tool_driver::Execute};
use codetether_tool_outcome::Outcome;
impl Execute for Runner<'_> {
    async fn execute(&mut self, call: &Call) -> anyhow::Result<Outcome> {
        if codetether_tool_core::context_edit::is_context_edit(&call.name) {
            self.inner.session.save().await?;
        }
        super::super::invoke::execute(self, call).await
    }
}
