//! LCB-based provider selection for swarm subtask dispatch.
//!
//! Scores registered providers with [`DelegationState::rank_candidates`]
//! and returns the highest-LCB provider for the subtask's specialty.

use crate::provider::ProviderRegistry;
use crate::session::delegation::DelegationState;
use crate::session::delegation_skills;
use crate::session::relevance::{Bucket, Dependency, Difficulty, ToolUse};

/// Pick the best `(provider_name, model_name)` for a subtask.
///
/// Returns the LCB-optimal provider when delegation is enabled,
/// or the supplied default when it is not.
pub fn choose_provider_for_subtask(
    registry: &ProviderRegistry,
    state: &DelegationState,
    specialty: &str,
    default_provider: &str,
    default_model: &str,
) -> (String, String) {
    if !state.enabled() {
        return (default_provider.to_string(), default_model.to_string());
    }
    let bucket = Bucket {
        difficulty: difficulty_for_specialty(specialty),
        dependency: Dependency::Isolated,
        tool_use: tool_use_for_specialty(specialty),
    };
    let providers = registry.list();
    let picked = state.rank_candidates(&providers, delegation_skills::SWARM_DISPATCH, bucket);
    match picked {
        Some(p) => {
            tracing::info!(provider = %p, specialty = %specialty, "LCB selected provider");
            (p.to_string(), default_model.to_string())
        }
        None => (default_provider.to_string(), default_model.to_string()),
    }
}

fn difficulty_for_specialty(s: &str) -> Difficulty {
    let s = s.to_ascii_lowercase();
    if s.contains("security") || s.contains("architect") {
        Difficulty::Hard
    } else if s.contains("review") || s.contains("test") {
        Difficulty::Medium
    } else {
        Difficulty::Easy
    }
}

fn tool_use_for_specialty(s: &str) -> ToolUse {
    let s = s.to_ascii_lowercase();
    if s.contains("deploy") || s.contains("infra") || s.contains("debug") {
        ToolUse::Yes
    } else {
        ToolUse::No
    }
}
