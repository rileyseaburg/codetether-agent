//! A2A session model configuration and best-effort persistence.
use std::future::Future;

/// Apply an available configured model to an A2A conversation.
///
/// # Arguments
/// * `model` — Mutable session model preference.
/// * `configured` — Application configuration lookup.
/// # Returns
/// Replaces the preference when configuration supplies a model; otherwise retains it.
/// # Examples
/// ```no_run
/// # async fn example() {
/// let mut model = None;
/// codetether_a2a_http::conversation::lifecycle::configure(&mut model, async { Some("provider/model".into()) }).await;
/// assert_eq!(model.as_deref(), Some("provider/model"));
/// # }
/// ```
pub async fn configure<F>(model: &mut Option<String>, configured: F)
where
    F: Future<Output = Option<String>>,
{
    if let Some(configured_model) = configured.await {
        *model = Some(configured_model);
    }
}

/// Persist an A2A conversation and log a save failure without propagating it.
///
/// # Arguments
/// * `session_id` — Session identifier included in failure diagnostics.
/// * `save` — Existing application persistence future.
/// # Returns
/// Completes after saving or logging the persistence failure.
/// # Examples
/// ```no_run
/// # async fn example() {
/// codetether_a2a_http::conversation::lifecycle::persist("session", async { Ok(()) }).await;
/// # }
/// ```
pub async fn persist<F>(session_id: &str, save: F)
where
    F: Future<Output = anyhow::Result<()>>,
{
    if let Err(e) = save.await {
        tracing::debug!(target: "codetether_agent::a2a::session_config", session_id = %session_id, error = %e, "Failed to persist A2A session");
    }
}
