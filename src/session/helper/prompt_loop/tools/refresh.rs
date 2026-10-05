//! Reconcile a known session-editing tool before appending its result.
use super::{super::Runner, call::Call};
use anyhow::Result;
pub(super) async fn after(runner: &mut Runner<'_>, call: &Call, success: bool) -> Result<()> {
    if call.name != "context_pin" || !success {
        return Ok(());
    }
    let current = &runner.session;
    let mut latest = crate::session::Session::load_tail(&current.id, current.messages.len())
        .await?
        .session;
    latest.max_steps = current.max_steps;
    latest.bus = current.bus.clone();
    latest.metadata.subcall_provider = current.metadata.subcall_provider.clone();
    latest.metadata.subcall_model_name = current.metadata.subcall_model_name.clone();
    *runner.session = latest;
    Ok(())
}
