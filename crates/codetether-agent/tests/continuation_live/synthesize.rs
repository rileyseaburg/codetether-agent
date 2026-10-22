//! Invoke the same engine, continuation query, and clamp as summary production.
use anyhow::Result;
use codetether_agent::provider::{Message, Provider};
use codetether_context_policy::{
    summary_index::{SUMMARY_POLICY, SUMMARY_POLICY_VERSION},
    transcript::messages_to_rlm_context,
};
use codetether_rlm::{provider_adapter::ProviderWrap, router::CrateAutoProcessContext};
use std::sync::Arc;
pub(super) async fn summary(
    messages: &[Message],
    provider: Arc<dyn Provider>,
    model: &str,
) -> Result<String> {
    let input = format!(
        "Summary policy {SUMMARY_POLICY_VERSION}: {SUMMARY_POLICY}\n{}",
        messages_to_rlm_context(messages)
    );
    let context = CrateAutoProcessContext {
        tool_id: "proactive_session_context",
        tool_args: serde_json::json!({"target_tokens":512}),
        session_id: "isolated-incident-replay",
        abort: None,
        on_progress: None,
        provider: Arc::new(ProviderWrap(provider)),
        model: model.into(),
        bus: None,
        trace_id: None,
        subcall_provider: None,
        subcall_model: None,
        rewriter: None,
    };
    let result = tokio::time::timeout(
        std::time::Duration::from_secs(180),
        codetether_rlm::router::auto_process_with_repl(&input, context, &Default::default()),
    )
    .await??;
    anyhow::ensure!(result.success, "summary engine failed");
    codetether_summary_quality::bounded_summary(result, 512)
}
