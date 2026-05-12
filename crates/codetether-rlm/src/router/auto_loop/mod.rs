//! Iterative RLM analysis loop — main entry.

use tracing::warn;
use crate::config::RlmConfig;
use crate::context_trace::ContextTrace;
use crate::traits::{LlmMessage, ToolDefinition};
use super::types::CrateAutoProcessContext;
use super::host::RouterHost;

mod support;
mod tool_dispatch;
mod text_path;

/// Outcome of the iterative loop.
pub(super) struct LoopOutcome {
    pub final_answer: Option<String>,
    pub iterations: usize,
    pub subcalls: usize,
    pub aborted: bool,
}

/// Run the iterative analysis loop.
pub async fn run(
    ctx: &CrateAutoProcessContext<'_>, config: &RlmConfig,
    host: &mut dyn RouterHost, conversation: &mut Vec<LlmMessage>,
    trace: &mut ContextTrace, tools: &[ToolDefinition], summary_mode: bool,
) -> LoopOutcome {
    let mut iterations = 0;
    let mut subcalls = 0;
    let mut final_answer: Option<String> = None;
    let mut aborted = false;

    for i in 0..config.max_iterations {
        iterations = i + 1;
        trace.next_iteration();
        support::emit_progress(ctx, iterations, config.max_iterations);
        if support::check_abort(ctx) { aborted = true; break; }
        let (provider, model) = support::active_provider(ctx, iterations);
        let response = match provider.complete(conversation.clone(), tools.to_vec(), &model, Some(0.7)).await {
            Ok(r) => r,
            Err(e) => {
                warn!(error = %e, iteration = iterations, "RLM: Model call failed");
                if iterations > 1 { break; }
                return LoopOutcome { final_answer: None, iterations, subcalls, aborted: false };
            }
        };
        let response = support::maybe_rewrite(ctx, response, tools).await;
        conversation.push(LlmMessage::assistant_from(&response));
        if let Some(a) = tool_dispatch::try_tool_calls(host, &response, conversation, trace, summary_mode) { final_answer = Some(a); break; }
        if let Some(a) = text_path::try_text_path(&response, summary_mode, iterations) { final_answer = Some(a); break; }
        text_path::push_continuation(conversation, &response);
        subcalls += 1;
        if subcalls >= config.max_subcalls { warn!(subcalls, "RLM: Max subcalls reached"); break; }
    }
    LoopOutcome { final_answer, iterations, subcalls, aborted }
}
