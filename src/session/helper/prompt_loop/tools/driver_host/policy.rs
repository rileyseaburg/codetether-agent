//! Native policy and per-turn state bindings for single-call dispatch.
use super::super::super::Runner;
use codetether_prompt_loop::{Call, LoopState, tool_driver::Policy};
impl Policy for Runner<'_> {
    fn blocked(&mut self, call: &Call) -> Option<String> {
        super::super::call_guard::blocked(self, call)
    }
    fn progress(&mut self) -> &mut LoopState {
        &mut self.progress
    }
    fn codesearch_guard(&mut self, step: usize) -> bool {
        super::super::codesearch::guard(self, step)
    }
}
