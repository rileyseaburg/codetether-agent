use crate::provider::Message;
use crate::tool::ToolRegistry;
use std::path::Path;

#[path = "router_registry.rs"]
mod registry;
pub use codetether_model_failover::router::known_good_router_candidates;

pub async fn build_proactive_lsp_context_message(
    _selected_provider: &str,
    step: usize,
    tool_registry: &ToolRegistry,
    session_messages: &[Message],
    workspace_dir: &Path,
) -> Option<Message> {
    codetether_proactive_context::lsp::build(
        step,
        || tool_registry.get("lsp"),
        session_messages,
        workspace_dir,
    )
    .await
}

pub fn choose_router_target(
    registry: &crate::provider::ProviderRegistry,
    selected_provider: &str,
    current_model: &str,
) -> Option<(String, String)> {
    codetether_model_failover::router::choose_router_target(
        registry,
        selected_provider,
        current_model,
    )
}

/// CADMAS-CTX-aware variant of [`choose_router_target`] (Phase C step 17).
///
/// When delegation is enabled on `state`, the rule-based candidate list
/// from [`known_good_router_candidates`] is re-ordered by the LCB score
/// `μ − γ·√u` under the supplied `bucket`. Candidates with no
/// observations yet keep their original rule order (cold-start
/// conservatism). When delegation is disabled, this function
/// is exactly [`choose_router_target`].
///
/// The actual outcome update — `state.update(provider, "model_call",
/// bucket, success)` — lives in the prompt loop's retry handler and is
/// wired in a follow-up commit. This function is the *selection* half
/// of the bandit loop; the *update* half is the hook point.
///
/// # Arguments
///
/// * `registry` — Active provider registry (same as the non-bandit path).
/// * `state` — CADMAS-CTX sidecar for this session. Read-only here.
/// * `bucket` — Context bucket extracted from the current turn's
///   [`RelevanceMeta`](crate::session::relevance::RelevanceMeta).
/// * `selected_provider` / `current_model` — Same semantics as
///   [`choose_router_target`].
///
/// # Examples
///
/// ```rust,no_run
/// # tokio::runtime::Runtime::new().unwrap().block_on(async {
/// use codetether_agent::provider::ProviderRegistry;
/// use codetether_agent::session::delegation::{DelegationConfig, DelegationState};
/// use codetether_agent::session::helper::router::choose_router_target_bandit;
/// use codetether_agent::session::relevance::{Bucket, Dependency, Difficulty, ToolUse};
///
/// let registry = ProviderRegistry::from_vault().await.unwrap();
/// let state = DelegationState::with_config(DelegationConfig::default());
/// let bucket = Bucket {
///     difficulty: Difficulty::Easy,
///     dependency: Dependency::Isolated,
///     tool_use: ToolUse::No,
/// };
/// let _ = choose_router_target_bandit(&registry, &state, bucket, "openai", "gpt-5");
/// # });
/// ```
pub fn choose_router_target_bandit(
    registry: &crate::provider::ProviderRegistry,
    state: &crate::session::delegation::DelegationState,
    bucket: crate::session::relevance::Bucket,
    selected_provider: &str,
    current_model: &str,
) -> Option<(String, String)> {
    codetether_model_failover::router::choose_router_target_bandit(
        registry,
        state,
        bucket,
        selected_provider,
        current_model,
    )
}
