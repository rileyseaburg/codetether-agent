use crate::cognition::{
    ThinkerClient, ThoughtEvent, ThoughtResult, ThoughtWorkItem,
    beliefs::{self, Belief},
    loop_ctx::LoopCtx,
    thought_generate, tick_test_phase,
};
use crate::tool::ToolRegistry;
use std::sync::Arc;
pub(super) struct Native;
impl codetether_cognition_core::runtime::process::Host<ThinkerClient, ToolRegistry> for Native {
    async fn generate(
        &self,
        thinker: Option<&ThinkerClient>,
        work: &ThoughtWorkItem,
        context: &[ThoughtEvent],
    ) -> ThoughtResult {
        thought_generate::generate_phase_thought(thinker, work, context).await
    }
    async fn extract(
        &self,
        thinker: Option<&ThinkerClient>,
        persona: &str,
        text: &str,
    ) -> Vec<Belief> {
        beliefs::extract_beliefs_from_thought(thinker, persona, text).await
    }
    async fn run_tools(
        &self,
        ctx: &LoopCtx,
        tools: &Arc<ToolRegistry>,
        work: &ThoughtWorkItem,
        text: &str,
    ) -> Vec<ThoughtEvent> {
        tick_test_phase::run_tools(&ctx.personas, ctx.thinker.as_deref(), tools, work, text).await
    }
}
