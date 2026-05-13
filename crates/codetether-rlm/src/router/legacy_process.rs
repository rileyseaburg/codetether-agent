//! Legacy iterative loop fallback.

use std::time::Instant;

use crate::config::RlmConfig;
use crate::result::RlmResult;

use super::host::RouterHost;
use super::legacy_answer;
use super::types::CrateAutoProcessContext;
use super::{auto_loop, auto_prepare};

/// Run the pre-engine iterative RLM loop.
pub async fn run(
    output: &str,
    ctx: CrateAutoProcessContext<'_>,
    config: &RlmConfig,
    host: &mut dyn RouterHost,
    start: Instant,
) -> anyhow::Result<RlmResult> {
    let prepared = auto_prepare::prepare(output, &ctx, config, host);
    let trace_id = prepared.trace_id;
    let mut conv = prepared.conversation;
    let mut trace = prepared.trace;
    let outcome = auto_loop::run(
        &ctx,
        config,
        host,
        &mut conv,
        &mut trace,
        &prepared.tools,
        prepared.summary_mode,
    )
    .await;
    let answer = legacy_answer::choose(output, &ctx, prepared.input_tokens, &outcome);
    let result = super::auto_process_emit::build_result(
        &answer,
        prepared.input_tokens,
        &outcome,
        start,
        trace_id,
    );
    super::auto_process_bus::emit_bus(&ctx, &result, &outcome, trace_id);
    Ok(result)
}
