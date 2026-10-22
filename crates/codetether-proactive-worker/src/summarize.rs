//! Awaited RLM production for proactive summary-index nodes.

use codetether_context_policy::summary_index::SummaryNode;
use codetether_context_policy::transcript::messages_to_rlm_context;
use codetether_rlm::provider_adapter::ProviderWrap;
use codetether_rlm::router::{CrateAutoProcessContext, auto_process_with_repl};
use std::sync::Arc;

use codetether_proactive_planning::Plan;
use codetether_proactive_runtime::Snapshot;

pub(super) async fn range(snapshot: &Snapshot, plan: Plan) -> anyhow::Result<SummaryNode> {
    let slice = snapshot
        .messages
        .get(plan.range.start..plan.range.end)
        .ok_or_else(|| anyhow::anyhow!("proactive RLM range out of bounds"))?;
    let context = messages_to_rlm_context(slice);
    let provenance = codetether_context_policy::summary_index::SummaryProvenance::new(
        slice,
        "proactive-rlm",
        Some(&snapshot.runtime.model),
    )?;
    let policy = codetether_context_policy::summary_index::SUMMARY_POLICY;
    let version = codetether_context_policy::summary_index::SUMMARY_POLICY_VERSION;
    let input = format!(
        "Summary policy {version}: {policy}\nSource SHA-256: {}\nRequested model: {:?}\nSummarize this transcript range in at most {} tokens.\n\n{context}",
        provenance.source_sha256, provenance.requested_model, plan.target_tokens,
    );
    let request = CrateAutoProcessContext {
        tool_id: "proactive_session_context",
        tool_args: serde_json::json!({
            "start": plan.range.start,
            "end": plan.range.end,
            "target_tokens": plan.target_tokens,
        }),
        session_id: &snapshot.session_id,
        abort: None,
        on_progress: None,
        provider: Arc::new(ProviderWrap(snapshot.runtime.provider.clone())),
        model: snapshot.runtime.model.clone(),
        bus: None,
        trace_id: None,
        subcall_provider: None,
        subcall_model: None,
        rewriter: None,
    };
    let result = auto_process_with_repl(&input, request, &snapshot.runtime.config).await?;
    let content = codetether_summary_quality::bounded_summary(result, plan.target_tokens)?;
    Ok(SummaryNode {
        content,
        target_tokens: plan.target_tokens,
        granularity: plan.granularity,
        generation: snapshot.generation,
        provenance: Some(provenance),
    })
}
