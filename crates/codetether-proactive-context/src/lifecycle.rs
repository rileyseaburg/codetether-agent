//! Register, schedule and remove proactive context in the original order.
use codetether_provider_core::Provider;
use std::sync::Arc;

/// Register the preferred provider, model and RLM configuration.
/// # Arguments
/// Receives a session view, caller provider and caller model fallback.
/// # Returns
/// Updates this session's runtime registration.
/// # Examples
/// ```no_run
/// # fn example(s: &impl codetether_proactive_context::Source, p: std::sync::Arc<dyn codetether_provider_core::Provider>) {
/// codetether_proactive_context::register(s, p, "model");
/// # }
/// ```
pub fn register<S: crate::Source>(session: &S, provider: Arc<dyn Provider>, model: &str) {
    codetether_proactive_runtime::register(
        session.id().to_string(),
        super::runtime::resolve(session, provider, model),
    );
}

/// Register runtime settings before scheduling eligible work.
/// # Arguments
/// Receives session inputs, caller fallbacks and a native worker launcher.
/// # Returns
/// Returns after registering and scheduling work.
/// # Examples
/// ```no_run
/// # fn example(s: &impl codetether_proactive_context::Source, p: std::sync::Arc<dyn codetether_provider_core::Provider>) {
/// codetether_proactive_context::prepare(s, p, "model", |_| {});
/// # }
/// ```
pub fn prepare<S: crate::Source>(
    session: &S,
    provider: Arc<dyn Provider>,
    model: &str,
    spawn_worker: fn(String),
) {
    register(session, provider, model);
    schedule(session, spawn_worker);
}

/// Schedule eligible work using the supplied native worker launcher.
/// # Arguments
/// * `session` - Current session identity and transcript.
/// * `spawn_worker` - Launcher used only when a worker must start.
/// # Returns
/// Returns after registering work; execution continues in the worker.
/// # Examples
/// ```no_run
/// # fn launch(_: String) {}
/// # fn example(source: &impl codetether_proactive_context::Source) {
/// codetether_proactive_context::schedule(source, launch);
/// # }
/// ```
pub fn schedule<S: crate::Source>(session: &S, spawn_worker: fn(String)) {
    codetether_proactive_runtime::schedule(session.id(), session.messages().len(), spawn_worker);
}

/// Remove runtime registration, then remove its prepared sidecar.
/// # Arguments
/// Receives the session ID and native storage host.
/// # Returns
/// Completes after best-effort sidecar removal.
/// # Examples
/// ```no_run
/// # async fn example(host: &impl codetether_proactive_context::Host) {
/// codetether_proactive_context::remove("session-id", host).await;
/// # }
/// ```
pub async fn remove<H: crate::Host>(session_id: &str, host: &H) {
    codetether_proactive_runtime::remove(session_id);
    host.remove_prepared(session_id).await;
}
