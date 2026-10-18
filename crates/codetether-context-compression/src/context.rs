//! Owned settings shared across buffer-level compression passes.
use codetether_rlm::RlmConfig;
/// Owned compression settings without a borrow of the native session.
/// # Examples
/// ```
/// let context = codetether_context_compression::CompressContext::new("session", &Default::default());
/// assert_eq!(context.session_id, "session");
/// ```
#[derive(Clone)]
pub struct CompressContext {
    /// RLM thresholds, model selectors and iteration limits.
    pub rlm_config: RlmConfig,
    /// Owning session identifier propagated to RLM traces.
    pub session_id: String,
}
impl CompressContext {
    /// Snapshot a session identifier and its RLM configuration.
    /// # Arguments
    /// * `session_id` — Owning session identifier.
    /// * `rlm_config` — RLM settings to clone for this pass.
    /// # Returns
    /// Owned inputs independent of later changes to the source.
    pub fn new(session_id: &str, rlm_config: &RlmConfig) -> Self {
        Self {
            rlm_config: rlm_config.clone(),
            session_id: session_id.to_string(),
        }
    }
}
