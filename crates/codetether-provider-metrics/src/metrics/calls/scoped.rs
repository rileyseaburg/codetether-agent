//! Session-scoped non-streaming metrics delegation.

use super::super::super::{CompletionRequest, CompletionResponse};
use super::super::MetricsProvider;
use anyhow::Result;

pub(in super::super) async fn complete_scoped(
    provider: &MetricsProvider,
    request: CompletionRequest,
    session_id: &str,
) -> Result<CompletionResponse> {
    super::complete_inner(provider, request, Some(session_id)).await
}
