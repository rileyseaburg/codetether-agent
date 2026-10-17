//! Parent-session access-policy resolution for spawned agents.

use crate::ChildSession;

/// Resolve the parent's effective policy, failing closed when it cannot load.
/// # Arguments
/// * `inherited` — Live policy override, used before disk lookup.
/// * `parent_id` — Parent to load when no live override exists.
/// # Returns
/// The live or loaded policy, true without a parent, or false on load failure.
/// # Examples
/// ```rust,no_run
/// use codetether_agent_session::{ChildSession, parent_prior_context_allowed};
/// async fn example<S: ChildSession>() {
///     assert!(!parent_prior_context_allowed::<S>(Some(false), None).await);
/// }
/// ```
pub async fn resolve<S: ChildSession>(inherited: Option<bool>, parent_id: Option<&str>) -> bool {
    if let Some(allowed) = inherited {
        return allowed;
    }
    let Some(parent_id) = parent_id else {
        return true;
    };
    match S::load_parent_policy(parent_id).await {
        Ok(allowed) => allowed,
        Err(error) => {
            tracing::warn!(target: "codetether_agent::tool::agent::session_factory::parent_policy", session_id = %parent_id, %error, "Unable to load parent access policy; denying prior context in child");
            false
        }
    }
}
