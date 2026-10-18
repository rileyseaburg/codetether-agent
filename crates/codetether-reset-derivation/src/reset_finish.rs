//! Host summary retrieval followed by shared reset provenance finalization.
use super::reset_rebuild::rebuild_with_summary;
use anyhow::Result;
use codetether_context_policy::derivation::reset::{self, ResetSummaryPlan};
use codetether_context_policy::projection::helpers::DerivedContext;
use codetether_provider_core::Provider;
use codetether_session_events::SessionEvent;
use std::sync::Arc;
use tokio::sync::mpsc;
pub(super) async fn finish<T: crate::Source, S>(
    session: &T,
    plan: ResetSummaryPlan,
    provider: Arc<dyn Provider>,
    model: &str,
    event_tx: Option<&mpsc::Sender<SessionEvent<S>>>,
    resolver: codetether_rlm_background::Resolver,
) -> Result<DerivedContext> {
    let derived = rebuild_with_summary(
        session,
        &plan.prefix,
        &plan.tail,
        provider,
        model,
        plan.origin_len,
        plan.base_index,
        event_tx,
        resolver,
    )
    .await?;
    Ok(reset::finalize(plan, derived))
}
