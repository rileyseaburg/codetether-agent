//! Native context_summary adapter.
use super::host;
use crate::provider::Provider;
use crate::rlm::RlmConfig;
use std::sync::Arc;

pub(crate) fn context_summary(
    content: &str,
    reason: &str,
    session_id: &str,
    model: &str,
    provider: Arc<dyn Provider>,
    config: &RlmConfig,
) -> Option<String> {
    codetether_rlm_background::context_summary(
        content,
        reason,
        session_id,
        model,
        provider,
        config,
        host::resolve,
    )
}
