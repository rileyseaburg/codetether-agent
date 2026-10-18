//! Phase service bindings for the shared thinker client and tool registry.
use super::Host;
use crate::{
    ThoughtEvent, ThoughtResult, ThoughtWorkItem,
    beliefs::{Belief, extract::extract_beliefs_from_thought},
    execution::dispatch::{Registry, run_tools},
    runtime::LoopContext,
    thinker::client::{Client, Host as ClientHost},
    thought::generate::generate_phase_thought,
};
use std::sync::Arc;
/// Connect shared phase processing to a thinker client and tool registry.
///
/// # Examples
/// ```ignore
/// process_work(&context, &work, active, now, &mut output, &ClientServices).await;
/// ```
pub struct ClientServices;
impl<H: ClientHost, R: Registry> Host<Client<H>, R> for ClientServices {
    async fn generate(
        &self,
        thinker: Option<&Client<H>>,
        work: &ThoughtWorkItem,
        context: &[ThoughtEvent],
    ) -> ThoughtResult {
        generate_phase_thought(thinker, work, context).await
    }
    async fn extract(&self, thinker: Option<&Client<H>>, persona: &str, text: &str) -> Vec<Belief> {
        extract_beliefs_from_thought(thinker, persona, text).await
    }
    async fn run_tools(
        &self,
        ctx: &LoopContext<Client<H>, R>,
        tools: &Arc<R>,
        work: &ThoughtWorkItem,
        text: &str,
    ) -> Vec<ThoughtEvent> {
        run_tools(&ctx.personas, ctx.thinker.as_deref(), tools, work, text).await
    }
}
