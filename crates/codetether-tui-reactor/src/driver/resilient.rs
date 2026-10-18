//! Recoverable event iteration errors.
use super::Cycle;
pub(super) async fn run_resilient(host: &mut impl Cycle) -> bool {
    match host.select().await {
        Ok(quit) => quit,
        Err(err) => {
            tracing::error!(%err, "event loop iteration failed — recovering");
            host.mark_redraw();
            false
        }
    }
}
