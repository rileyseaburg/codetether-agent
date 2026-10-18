//! Native stream, discovery, refresh and publication hooks.
use super::super::super::Runner;
use codetether_prompt_loop::{Call, tool_driver::Effects};
use codetether_tool_outcome::Outcome;
impl Effects for Runner<'_> {
    async fn start(&mut self, call: &Call) {
        super::super::call_guard::publish_start(self, call).await;
    }
    async fn discover(&mut self, call: &Call) -> anyhow::Result<bool> {
        super::super::discovery::run(self, call).await
    }
    fn request(&mut self, step: usize, call: &Call) {
        super::super::bus::request(self, step, call);
    }
    async fn rejected(&mut self, call: &Call, content: String) {
        super::super::simple::record(self, call, content, false).await;
    }
    async fn refresh(&mut self, call: &Call, success: bool) -> anyhow::Result<()> {
        super::super::refresh::after(self, call, success).await
    }
    async fn publish(&mut self, step: usize, call: &Call, outcome: Outcome) -> anyhow::Result<()> {
        super::super::publish::complete(self, step, call, outcome).await
    }
}
