//! Preparation step for the auto-process loop.
//!
//! Sets up tracing, chunking, tool definitions, and the initial
//! exploration messages before the iterative loop begins.

use crate::chunker::RlmChunker;
use crate::config::RlmConfig;
use crate::context_trace::{ContextEvent, ContextTrace};
use crate::traits::ToolDefinition;
use uuid::Uuid;

use super::host::RouterHost;
use super::prompts::build_query_for_tool;
use super::prompts_system::{build_exploration_summary, build_system_prompt};
use super::types::CrateAutoProcessContext;
use crate::traits::LlmMessage;

/// Opaque state carried between prepare and the iterative loop.
pub(super) struct Prepared {
    pub trace_id: Uuid,
    pub trace: ContextTrace,
    pub tools: Vec<ToolDefinition>,
    pub conversation: Vec<LlmMessage>,
    pub summary_mode: bool,
    pub input_tokens: usize,
}

/// Set up trace, tools, and initial messages.
pub fn prepare(output: &str, ctx: &CrateAutoProcessContext<'_>, _config: &RlmConfig,
    host: &dyn RouterHost) -> Prepared {
    let input_tokens = RlmChunker::estimate_tokens(output);
    let trace_id = ctx.trace_id.unwrap_or_else(Uuid::new_v4);
    let trace_budget = input_tokens.saturating_mul(2).max(4096);
    let mut trace = ContextTrace::new(trace_budget);

    let summary_mode = matches!(ctx.tool_id, "session_context" | "context_reset" | "summary_index");
    let tools = if summary_mode { vec![] } else { host.tool_definitions() };

    let content_type = RlmChunker::detect_content_type(output);
    let hints = RlmChunker::get_processing_hints(content_type);
    tracing::info!(content_type = ?content_type, tool = ctx.tool_id, "RLM: Content type detected");

    let processed = if input_tokens > 50000 { RlmChunker::compress(output, 40000, None) } else { output.to_string() };

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

    Prepared { trace_id, trace, tools, conversation, summary_mode, input_tokens }
}
