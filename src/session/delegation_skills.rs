//! Canonical skill-name constants for CADMAS-CTX delegation hooks.
//!
//! ## Why a dedicated module
//!
//! Each routing surface in codetether — provider failover, swarm
//! dispatch, ralph handoff, autochat persona relay, RLM compaction
//! model — wants to key its per-(agent, skill, bucket) Beta
//! posteriors against a stable `skill` string. Spelling drift across
//! call sites ("model_call" vs "router" vs "provider_call") would
//! silently fragment posteriors and starve the LCB score of
//! evidence.
//!
//! This module centralises the catalogue. Every site that calls
//! [`DelegationState::update`] or [`DelegationState::score`] **must**
//! pass one of these constants as the `skill` argument.
//!
//! ## Stable shape
//!
//! * Names are lowercase `snake_case`.
//! * Never renamed — adding a new surface gets a new constant; old
//!   ones stay so existing sidecar data remains readable.
//! * Documented alongside the codetether surface they key.
//!
//! [`DelegationState::update`]: crate::session::delegation::DelegationState::update
//! [`DelegationState::score`]: crate::session::delegation::DelegationState::score
//!
//! ## Examples
//!
//! ```rust
//! use codetether_agent::session::delegation_skills;
//!
//! assert_eq!(delegation_skills::MODEL_CALL, "model_call");
//! assert!(delegation_skills::ALL.contains(&"swarm_dispatch"));
//! ```

/// Skill for provider-failover routing in
/// [`choose_router_target_bandit`](crate::session::helper::router::choose_router_target_bandit).
pub const MODEL_CALL: &str = "model_call";

/// Skill for RLM compaction model selection in [`crate::rlm::select_rlm_model`].
pub const RLM_COMPACT: &str = "rlm_compact";

/// Skill for swarm-executor dispatch
/// (`src/swarm/orchestrator.rs` — Phase C step 28).
pub const SWARM_DISPATCH: &str = "swarm_dispatch";

/// Skill for ralph-loop persona handoff
/// (`src/ralph/ralph_loop.rs` — Phase C step 29).
pub const RALPH_HANDOFF: &str = "ralph_handoff";

/// Skill for TUI autochat next-persona selection
/// (`src/tui/app/autochat/` — Phase C step 31).
pub const AUTOCHAT_PERSONA: &str = "autochat_persona";

/// Complete list of registered skill names.
///
/// New surfaces go here *and* get their own constant so grep-ability
/// across the codebase survives refactors.
pub const ALL: &[&str] = &[
    MODEL_CALL,
    RLM_COMPACT,
    SWARM_DISPATCH,
    RALPH_HANDOFF,
    AUTOCHAT_PERSONA,
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_skills_are_unique_and_non_empty() {
        let mut copy: Vec<&str> = ALL.to_vec();
        copy.sort_unstable();
        copy.dedup();
        assert_eq!(copy.len(), ALL.len(), "skill names must be unique");
        for name in ALL {
            assert!(!name.is_empty());
            assert_eq!(
                name.to_ascii_lowercase(),
                *name,
                "skills must be snake_case"
            );
        }
    }

    #[test]
    fn model_call_is_stable() {
        // Changing this string would silently fragment existing
        // DelegationState sidecars. Unit-test it so the rename is
        // caught in review, not in production.
        assert_eq!(MODEL_CALL, "model_call");
    }
}
