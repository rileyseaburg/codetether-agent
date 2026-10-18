use crate::provider::Message;
use crate::tool::ToolRegistry;
use std::path::Path;

#[path = "router_candidates.rs"]
mod router_candidates;

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

pub fn known_good_router_candidates(provider: &str, failed_model: &str) -> Vec<String> {
    let failed = failed_model.trim();
    let mut candidates: Vec<String> = preferred_failover_model_refs();

    candidates.extend(router_candidates::provider_failover_candidates(provider));

    candidates.extend([
        "minimax/MiniMax-M3".to_string(),
        "zai/glm-5.1".to_string(),
        "zai/glm-5".to_string(),
    ]);

    let mut seen = std::collections::HashSet::new();
    candidates.retain(|candidate| {
        let unique = seen.insert(candidate.to_ascii_lowercase());
        unique
            && !candidate.eq_ignore_ascii_case(failed)
            && candidate
                .split('/')
                .next_back()
                .map(|model| !model.eq_ignore_ascii_case(failed))
                .unwrap_or(true)
    });
    candidates
}

fn preferred_failover_model_refs() -> Vec<String> {
    std::env::var("CODETETHER_PREFERRED_MODELS")
        .or_else(|_| std::env::var("CODETETHER_FAILOVER_MODELS"))
        .ok()
        .map(|raw| parse_model_ref_list(&raw))
        .unwrap_or_default()
}

fn parse_model_ref_list(raw: &str) -> Vec<String> {
    raw.split([',', ';', '\n'])
        .map(str::trim)
        .filter(|entry| !entry.is_empty() && entry.contains('/'))
        .map(ToString::to_string)
        .collect()
}

pub fn choose_router_target(
    registry: &crate::provider::ProviderRegistry,
    selected_provider: &str,
    current_model: &str,
) -> Option<(String, String)> {
    let current_provider = selected_provider.to_ascii_lowercase();

    for candidate in known_good_router_candidates(selected_provider, current_model) {
        let (provider_name, model_name) = crate::provider::parse_model_string(&candidate);
        let provider_name = provider_name.unwrap_or(selected_provider);
        if registry.get(provider_name).is_some() {
            return Some((provider_name.to_string(), model_name.to_string()));
        }
    }

    if current_provider != "zai" && registry.get("zai").is_some() {
        return Some(("zai".to_string(), "glm-5".to_string()));
    }
    if current_provider != "glm5" && registry.get("glm5").is_some() {
        return Some(("glm5".to_string(), "glm-5".to_string()));
    }
    if current_provider != "openrouter" && registry.get("openrouter").is_some() {
        return Some((
            "openrouter".to_string(),
            "openai/gpt-oss-120b:free".to_string(),
        ));
    }

    None
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
    if !state.enabled() {
        return choose_router_target(registry, selected_provider, current_model);
    }

    // Enumerate rule-based candidates, annotate each with an LCB score
    // (or 0.0 when there is no posterior yet), sort stably by score
    // descending, and return the first one the registry knows about.
    let candidates = known_good_router_candidates(selected_provider, current_model);
    let mut scored: Vec<(String, f64)> = candidates
        .into_iter()
        .map(|raw| {
            let (provider_name, _model_name) = crate::provider::parse_model_string(&raw);
            let provider_name = provider_name.unwrap_or(selected_provider);
            let score = state
                .score(
                    provider_name,
                    crate::session::delegation_skills::MODEL_CALL,
                    bucket,
                )
                .unwrap_or(0.0);
            (raw, score)
        })
        .collect();
    scored.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

    for (candidate, _score) in scored {
        let (provider_name, model_name) = crate::provider::parse_model_string(&candidate);
        let provider_name = provider_name.unwrap_or(selected_provider);
        if registry.get(provider_name).is_some() {
            return Some((provider_name.to_string(), model_name.to_string()));
        }
    }

    // No re-ordered candidate was registered — fall back to the
    // original rule-based ladder so we never regress against the
    // non-bandit behaviour.
    choose_router_target(registry, selected_provider, current_model)
}
