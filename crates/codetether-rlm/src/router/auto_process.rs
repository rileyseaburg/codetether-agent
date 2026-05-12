//! Top-level `auto_process` entry point.

use std::time::Instant;
use tracing::{info, warn};
use crate::config::RlmConfig;
use crate::result::RlmResult;
use super::auto_loop;
use super::auto_prepare;
use super::fallback::enhanced_fallback;
use super::host::RouterHost;
use super::types::CrateAutoProcessContext;

/// Process large output through the RLM iterative analysis loop.
pub async fn auto_process(
    output: &str,
    ctx: CrateAutoProcessContext<'_>,
    config: &RlmConfig,
    host: &mut dyn RouterHost,
) -> anyhow::Result<RlmResult> {
    let start = Instant::now();
    info!(tool = ctx.tool_id, "RLM: Starting auto-processing");

    let prepared = auto_prepare::prepare(output, &ctx, config, host);
    let trace_id = prepared.trace_id;
    let mut conv = prepared.conversation;
    let mut trace = prepared.trace;
    let tools = prepared.tools;

    let outcome = auto_loop::run(&ctx, config, host, &mut conv, &mut trace, &tools, prepared.summary_mode).await;
    let answer = outcome.final_answer.clone().unwrap_or_else(|| {
        warn!(iterations = outcome.iterations, "RLM: No FINAL produced, using fallback");
        enhanced_fallback(output, ctx.tool_id, &ctx.tool_args, prepared.input_tokens)
    });

    let result = super::auto_process_emit::build_result(&answer, prepared.input_tokens, &outcome, start, trace_id);
    super::auto_process_emit::emit_bus(&ctx, &result, &outcome, trace_id);
    Ok(result)
}
