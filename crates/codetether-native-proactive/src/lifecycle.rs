//! Native worker launch and prepared-index lifecycle.
use crate::{Host, Session};
use codetether_context_policy::summary_index::SummaryIndex;
pub use codetether_proactive_context::register;
use codetether_provider_core::Provider;
use std::sync::Arc;
fn spawn<S: Session>(session_id: String) {
    codetether_proactive_worker::spawn(session_id, Arc::new(Host::<S>::default()));
}
/// Register provider settings and schedule eligible proactive context work.
/// # Arguments
/// * session - Session settings and loaded transcript.
/// * provider - Fallback subcall provider.
/// * model - Fallback model name.
/// # Returns
/// Completes after updating registration and scheduling the shared worker.
/// # Examples
/// ```no_run
/// # fn example<S: codetether_native_proactive::Session>(session: &S, provider: std::sync::Arc<dyn codetether_provider_core::Provider>) {
/// codetether_native_proactive::prepare(session, provider, "model");
/// # }
/// ```
pub fn prepare<S: Session>(session: &S, provider: Arc<dyn Provider>, model: &str) {
    codetether_proactive_context::prepare(session, provider, model, spawn::<S>);
}
/// Schedule eligible proactive work for the current session.
/// # Arguments
/// * session - Session whose transcript determines worker eligibility.
/// # Returns
/// Completes after scheduling any required worker.
/// # Examples
/// ```no_run
/// # fn example<S: codetether_native_proactive::Session>(session: &S) {
/// codetether_native_proactive::schedule(session);
/// # }
/// ```
pub fn schedule<S: Session>(session: &S) {
    codetether_proactive_context::schedule(session, spawn::<S>);
}
/// Resolve the prepared summary index for the current session window.
/// # Arguments
/// * session - Session window and absolute message offset.
/// # Returns
/// The compatible prepared index, or an empty index when unavailable.
/// # Examples
/// ```no_run
/// # async fn example<S: codetether_native_proactive::Session>(session: &S) {
/// let index = codetether_native_proactive::prepared_index(session).await;
/// # }
/// ```
pub async fn prepared_index<S: Session>(session: &S) -> SummaryIndex {
    codetether_proactive_context::prepared_index(session, &Host::<S>::default()).await
}
/// Clear runtime registration and remove the prepared sidecar.
/// # Arguments
/// * session_id - Durable session identity.
/// # Returns
/// Completes after best-effort cleanup in the existing order.
/// # Examples
/// ```no_run
/// # async fn example<S: codetether_native_proactive::Session>() {
/// codetether_native_proactive::remove::<S>("session-id").await;
/// # }
/// ```
pub async fn remove<S: Session>(session_id: &str) {
    codetether_proactive_context::remove(session_id, &Host::<S>::default()).await;
}
