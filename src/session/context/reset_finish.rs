//! Host summary retrieval followed by shared reset provenance finalization.
use super::helpers::DerivedContext;
use super::reset_rebuild::rebuild_with_summary;
use crate::provider::Provider;
use crate::session::{Session, SessionEvent};
use anyhow::Result;
use codetether_context_policy::derivation::reset::{self, ResetSummaryPlan};
use std::sync::Arc;
use tokio::sync::mpsc;
pub(super) async fn finish(
    session: &Session,
    plan: ResetSummaryPlan,
    provider: Arc<dyn Provider>,
    model: &str,
    event_tx: Option<&mpsc::Sender<SessionEvent>>,
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
    )
    .await?;
    Ok(reset::finalize(plan, derived))
}
