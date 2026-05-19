//! Engine entry point.

use anyhow::Result;
use std::time::Instant;
use uuid::Uuid;

use crate::config::RlmConfig;
use crate::result::RlmResult;
use crate::router::CrateAutoProcessContext;

use super::{complete, deterministic, evidence, file, query, semantic};

/// Run the evidence-first engine.
pub async fn process(
    content: &str,
    ctx: &CrateAutoProcessContext<'_>,
    config: &RlmConfig,
) -> Result<Option<RlmResult>> {
    if config.mode == "off" || content.trim().is_empty() {
        return Ok(None);
    }
    let start = Instant::now();
    let query = query::extract(ctx);
    let source = file::source_label(ctx);
    let trace_id = ctx.trace_id.unwrap_or_else(Uuid::new_v4);
    let input_tokens = crate::RlmChunker::estimate_tokens(content);

    if let Some(answer) = deterministic::answer(content, &query, source.clone(), ctx)? {
        return Ok(Some(complete::finish(
            ctx,
            answer,
            input_tokens,
            start,
            trace_id,
            None,
        )));
    }

    let evidence = evidence::collect(content, &query, &source, config);
    let answer = semantic::synthesize(ctx, &query, &evidence).await?;
    Ok(Some(complete::finish(
        ctx,
        answer,
        evidence.input_tokens,
        start,
        trace_id,
        Some(&evidence),
    )))
}
