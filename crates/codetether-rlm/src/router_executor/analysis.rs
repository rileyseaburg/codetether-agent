//! Analysis bridge implementation.

use anyhow::Result;
use std::time::Instant;

use super::RlmExecutor;
use crate::router::auto_process_with_repl;
use crate::{RlmAnalysisResult, RlmConfig, RlmStats, SubQuery};

/// Execute one bridge analysis through the crate router.
pub async fn analyze(exec: &mut RlmExecutor, query: &str) -> Result<RlmAnalysisResult> {
    let started = Instant::now();
    let ctx = super::context::auto_context(exec, query);
    let cfg = RlmConfig {
        max_iterations: exec.max_iterations,
        ..Default::default()
    };
    let result = auto_process_with_repl(&exec.context, ctx, &cfg).await?;
    let answer = answer_text(&result.processed);
    super::trace::record_trace(exec, query, &answer, &result);
    Ok(RlmAnalysisResult {
        sub_queries: vec![SubQuery {
            query: query.into(),
            context_slice: None,
            response: answer.clone(),
            tokens_used: result.stats.output_tokens,
        }],
        iterations: result.stats.iterations,
        stats: RlmStats {
            elapsed_ms: started.elapsed().as_millis() as u64,
            ..result.stats
        },
        answer,
    })
}

fn answer_text(processed: &str) -> String {
    processed
        .split_once("\n\n")
        .map_or(processed, |(_, answer)| answer)
        .trim()
        .to_string()
}
