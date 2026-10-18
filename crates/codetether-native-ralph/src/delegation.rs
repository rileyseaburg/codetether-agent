//! LCB-based provider selection for Ralph relay personas.
//!
//! Each relay role (planner, coder, reviewer, tester) has a skill
//! profile.  When delegation is enabled, [`choose_provider_for_role`]
//! picks the LCB-optimal provider; otherwise the default is returned.

use codetether_context_policy::delegation::DelegationState;
use codetether_native_providers::ProviderRegistry;
pub use codetether_ralph_core::delegation::{ROLE_CODER, ROLE_PLANNER, ROLE_REVIEWER, ROLE_TESTER};

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
    codetether_ralph_core::delegation::choose_provider_for_role(
        state,
        role,
        default_provider,
        default_model,
        || registry.list(),
    )
}
