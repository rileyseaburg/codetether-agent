//! Parent-session access-policy resolution for spawned agents.

use crate::session::Session;

/// Resolve the parent's effective policy, failing closed when it cannot load.
pub(super) async fn resolve(inherited: Option<bool>, parent_id: Option<&str>) -> bool {
    if let Some(allowed) = inherited {
        return allowed;
    }
    let Some(parent_id) = parent_id else {
        return true;
    };
    match Session::load_tail(parent_id, 16).await {
        Ok(load) => {
            crate::session::helper::runtime::prior_context_allowed_for_session(&load.session)
        }
        Err(error) => {
            tracing::warn!(session_id = %parent_id, %error, "Unable to load parent access policy; denying prior context in child");
            false
        }
    }
}
