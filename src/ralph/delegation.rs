//! LCB-based provider selection for Ralph relay personas.
//!
//! Each relay role (planner, coder, reviewer, tester) has a skill
//! profile.  When delegation is enabled, [`choose_provider_for_role`]
//! picks the LCB-optimal provider; otherwise the default is returned.

use crate::provider::ProviderRegistry;
use crate::session::delegation::DelegationState;
use crate::session::delegation_skills;
use crate::session::relevance::{Bucket, Dependency, Difficulty, ToolUse};

/// Role identifiers used as delegation skill keys.
pub const ROLE_PLANNER: &str = "ralph_planner";
pub const ROLE_CODER: &str = "ralph_coder";
pub const ROLE_REVIEWER: &str = "ralph_reviewer";
pub const ROLE_TESTER: &str = "ralph_tester";

/// Pick the best `(provider_name, model_name)` for a relay role.
///
/// Maps the role to a relevance bucket and delegates to
/// [`DelegationState::rank_candidates`].
pub fn choose_provider_for_role(
    registry: &ProviderRegistry,
    state: &DelegationState,
    role: &str,
    default_provider: &str,
    default_model: &str,
) -> (String, String) {
    if !state.enabled() {
        return (default_provider.to_string(), default_model.to_string());
    }
    let skill = skill_for_role(role);
    let bucket = Bucket {
        difficulty: difficulty_for_role(role),
        dependency: Dependency::Isolated,
        tool_use: tool_use_for_role(role),
    };
    let picked = state.rank_candidates(&registry.list(), skill, bucket);
    match picked {
        Some(p) => {
            tracing::info!(provider = %p, role = %role, "LCB selected provider for relay role");
            (p.to_string(), default_model.to_string())
        }
        None => (default_provider.to_string(), default_model.to_string()),
    }
}

fn skill_for_role(role: &str) -> &'static str {
    match role {
        "auto-coder" => ROLE_CODER,
        "auto-reviewer" => ROLE_REVIEWER,
        "auto-tester" => ROLE_TESTER,
        _ => delegation_skills::MODEL_CALL,
    }
}

fn difficulty_for_role(role: &str) -> Difficulty {
    if role.contains("planner") || role.contains("reviewer") { Difficulty::Medium }
    else if role.contains("coder") { Difficulty::Hard }
    else { Difficulty::Easy }
}

fn tool_use_for_role(role: &str) -> ToolUse {
    if role.contains("coder") || role.contains("tester") { ToolUse::Yes }
    else { ToolUse::No }
}
