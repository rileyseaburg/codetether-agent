//! Recording of synthetic or pre-execution tool results.

use super::{super::Runner, call::Call};

/// Records a synthetic or rejected tool result in events and history.
pub(super) async fn record(runner: &mut Runner<'_>, call: &Call, content: String, success: bool) {
    codetether_prompt_loop::tool_publication::record(runner, call, content, success).await
}
