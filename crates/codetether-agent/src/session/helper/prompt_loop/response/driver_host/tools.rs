//! Native message insertion and tool-dispatch bindings.
use super::super::super::Runner;
use crate::provider::Message;
use codetether_prompt_loop::driver::{ParsedCall, Tools};
impl Tools for Runner<'_> {
    fn track_writes(&mut self, step: usize, calls: &[ParsedCall]) {
        codetether_prompt_loop::response::progress::track_writes(self, step, calls);
    }
    fn add_message(&mut self, message: Message) {
        self.inner.session.add_message(message);
    }
    async fn execute(&mut self, step: usize, calls: Vec<ParsedCall>) -> anyhow::Result<()> {
        super::super::super::tools::execute(self, step, calls).await
    }
}

impl codetether_prompt_loop::response::progress::Host for Runner<'_> {
    fn progress(&mut self) -> &mut codetether_prompt_loop::LoopState {
        &mut self.inner.progress
    }
    fn add_message(&mut self, message: Message) {
        self.inner.session.add_message(message);
    }
}
