//! Deadline-bound processing for oversized sub-agent tool results.

use crate::provider::Provider;
use std::sync::Arc;
use std::time::Instant;
use tokio::time::timeout;

pub(super) async fn process(
    result: &str,
    tool_name: &str,
    provider: Arc<dyn Provider>,
    model: &str,
    deadline: Instant,
) -> (String, bool) {
    let remaining = deadline.saturating_duration_since(Instant::now());
    match timeout(
        remaining,
        super::large_result::process(result, tool_name, provider, model),
    )
    .await
    {
        Ok(processed) => (processed, false),
        Err(_) => {
            tracing::warn!(tool = %tool_name, "Large-result processing exceeded sub-agent deadline");
            (
                crate::swarm::token_truncate::truncate_single_result(
                    result,
                    super::large_result::SIMPLE_TRUNCATE_CHARS,
                ),
                true,
            )
        }
    }
}
