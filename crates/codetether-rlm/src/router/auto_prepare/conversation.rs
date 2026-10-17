//! Build content hints, trace the system prompt, and prepare exploration messages.

use crate::router::{
    prompts::build_query_for_tool,
    prompts_system::{build_exploration_summary, build_system_prompt},
};
use crate::{
    chunker::RlmChunker,
    context_trace::{ContextEvent, ContextTrace},
    router::types::CrateAutoProcessContext,
    traits::LlmMessage,
};
pub(super) fn build(
    output: &str,
    ctx: &CrateAutoProcessContext<'_>,
    input_tokens: usize,
    trace: &mut ContextTrace,
) -> Vec<LlmMessage> {
    let content_type = RlmChunker::detect_content_type(output);
    let hints = RlmChunker::get_processing_hints(content_type);
    tracing::info!(content_type = ?content_type, tool = ctx.tool_id, "RLM: Content type detected");

    let processed = if input_tokens > 50000 {
        RlmChunker::compress(output, 40000, None)
    } else {
        output.to_string()
    };

    let base_query = build_query_for_tool(ctx.tool_id, &ctx.tool_args);
    let query = format!("{base_query}\n\n## Content Analysis Hints\n{hints}");
    let sys = build_system_prompt(input_tokens, ctx.tool_id, &query);
    trace.log_event(ContextEvent::SystemPrompt {
        content: sys.clone(),
        tokens: ContextTrace::estimate_tokens(&sys),
    });

    let exploration = build_exploration_summary(&processed, input_tokens);
    let user_text = format!(
        "{sys}\n\nHere is the context exploration:\n```\n{exploration}\n```\n\nNow analyze and answer the query."
    );
    let conversation = vec![LlmMessage::user(user_text)];

    conversation
}
