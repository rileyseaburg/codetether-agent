//! Iterative RLM analysis loop — main entry.

use tracing::warn;
use crate::config::RlmConfig;
use crate::context_trace::ContextTrace;
use crate::traits::{LlmMessage, ToolDefinition};
use super::types::{CrateAutoProcessContext, LoopOutcome};
use super::host::RouterHost;

mod support;
mod tool_dispatch;
mod text_path;

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
    let mut last_error: Option<String> = None;

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
                last_error = Some(format!("Model call failed at iteration {iterations}: {e}"));
                if iterations > 1 { break; }
                return LoopOutcome { final_answer: None, iterations, subcalls, aborted: false, last_error };
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
    LoopOutcome { final_answer, iterations, subcalls, aborted, last_error }
}
