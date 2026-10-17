//! Perform one request and interpret its response.

use crate::{
    context_trace::ContextTrace,
    router::{
        host::RouterHost,
        types::{CrateAutoProcessContext, LoopOutcome},
    },
    traits::{LlmMessage, ToolDefinition},
};
pub(super) async fn run(
    ctx: &CrateAutoProcessContext<'_>,
    host: &mut dyn RouterHost,
    conversation: &mut Vec<LlmMessage>,
    trace: &mut ContextTrace,
    tools: &[ToolDefinition],
    summary_mode: bool,
    state: &mut LoopOutcome,
) -> bool {
    let Some(response) = super::request::next(ctx, state, conversation, tools).await else {
        return false;
    };
    let response = super::support::maybe_rewrite(ctx, response, tools).await;
    conversation.push(LlmMessage::assistant_from(&response));
    if let Some(answer) = super::reply::process(
        host,
        &response,
        conversation,
        trace,
        summary_mode,
        state.iterations,
    ) {
        state.final_answer = Some(answer);
        return false;
    }
    true
}
