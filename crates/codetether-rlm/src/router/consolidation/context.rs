//! Deterministic model and cancellation context.
use super::provider::Provider;
use crate::router::CrateAutoProcessContext;
use std::sync::Arc;
pub(super) fn build(
    first: Arc<Provider>,
    second: Arc<Provider>,
    abort: bool,
) -> CrateAutoProcessContext<'static> {
    let (_, receiver) = tokio::sync::watch::channel(abort);
    let ctx = CrateAutoProcessContext {
        tool_id: "webfetch",
        tool_args: serde_json::json!({}),
        session_id: "fixture",
        abort: Some(receiver),
        on_progress: None,
        provider: first.clone(),
        model: "main".into(),
        bus: None,
        trace_id: Some(uuid::Uuid::nil()),
        subcall_provider: Some(second.clone()),
        subcall_model: Some("sub".into()),
        rewriter: None,
    };
    ctx
}
