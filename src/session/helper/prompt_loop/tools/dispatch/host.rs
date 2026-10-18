//! Native execution bindings for the shared batch dispatcher.
use super::super::super::Runner;
use codetether_tool_parallel::dispatch::ToolCall;
use serde_json::Value;
impl codetether_tool_parallel::dispatch::Host for Runner<'_> {
    fn allow_parallel(&self) -> bool {
        self.events.is_some()
    }
    async fn parallel(&mut self, calls: &[ToolCall]) -> bool {
        super::super::parallel::run(self, calls).await
    }
    async fn single(
        &mut self,
        step: usize,
        id: &str,
        name: &str,
        input: &Value,
    ) -> anyhow::Result<bool> {
        let call = super::super::call::Call::new(id.to_owned(), name.to_owned(), input.clone());
        super::super::call::run(self, step, call).await
    }
}
