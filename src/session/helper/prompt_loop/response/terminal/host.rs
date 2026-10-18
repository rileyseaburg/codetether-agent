//! Native services for shared terminal-response sequencing.
use super::super::super::Runner;
use crate::provider::Message;
use codetether_prompt_loop::{LoopState, StepFlow};
impl codetether_prompt_loop::terminal::Host for Runner<'_> {
    fn append(&mut self, message: Message) {
        self.inner.session.add_message(message);
    }
    fn drain_steering(&mut self) -> usize {
        super::super::super::super::steering::drain_into(self.inner.session)
    }
    fn agent(&self) -> &str {
        &self.inner.session.agent
    }
    fn progress(&mut self) -> &mut LoopState {
        &mut self.inner.progress
    }
    async fn validate(&mut self) -> anyhow::Result<Option<String>> {
        super::super::super::super::validation::build_validation_report(
            &self.inner.workspace.cwd,
            &self.inner.workspace.touched,
            &self.inner.workspace.baseline_dirty,
        )
        .await
        .map(|report| report.map(|report| report.prompt))
    }
    async fn final_flow(&mut self) -> anyhow::Result<StepFlow> {
        super::super::goal::after_message(self).await
    }
}
