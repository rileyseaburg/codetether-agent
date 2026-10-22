//! Reconcile a known session-editing tool before appending its result.
use super::{super::Runner, call::Call};
use anyhow::Result;
pub(super) async fn after(runner: &mut Runner<'_>, call: &Call, success: bool) -> Result<()> {
    if !codetether_tool_core::context_edit::is_context_edit(&call.name) || !success {
        return Ok(());
    }
    runner.inner.session.refresh_context_from_store().await
}
