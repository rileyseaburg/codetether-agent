//! Native message insertion and tool-dispatch bindings.
use super::super::super::Runner;
use crate::provider::Message;
use codetether_prompt_loop::driver::{ParsedCall, Tools};
impl Tools for Runner<'_> {
    fn track_progress(&mut self, step: usize) {
        codetether_prompt_loop::response::progress::track(self, step);
    }
    fn add_message(&mut self, message: Message) {
        self.inner.session.add_message(message);
    }
    async fn execute(&mut self, step: usize, calls: Vec<ParsedCall>) -> anyhow::Result<()> {
        super::super::super::tools::execute(self, step, calls).await
    }
}

impl codetether_prompt_loop::response::progress::Host for Runner<'_> {
    fn evidence(&mut self) -> &mut codetether_repeat_guard::ProgressLedger {
        &mut self.inner.session.metadata.progress
    }
    fn add_message(&mut self, message: Message) {
        self.inner.session.add_message(message);
    }
}
