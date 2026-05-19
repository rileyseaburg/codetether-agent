//! Top-level `auto_process` entry point.

use super::host::RouterHost;
use super::legacy_process;
use super::types::CrateAutoProcessContext;
use crate::config::RlmConfig;
use crate::result::RlmResult;
use std::time::Instant;
use tracing::{info, warn};

/// Process large output through the RLM iterative analysis loop.
pub async fn auto_process(
    output: &str,
    ctx: CrateAutoProcessContext<'_>,
    config: &RlmConfig,
    host: &mut dyn RouterHost,
) -> anyhow::Result<RlmResult> {
    let start = Instant::now();
    info!(tool = ctx.tool_id, "RLM: Starting auto-processing");

    match crate::engine::process(output, &ctx, config).await {
        Ok(Some(result)) => return Ok(result),
        Ok(None) => {}
        Err(e) => warn!(error = %e, "RLM engine failed, using legacy loop"),
    }

    legacy_process::run(output, ctx, config, host, start).await
}
