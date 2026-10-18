//! Select local or remote authorization evaluation.
use crate::{PolicyResource, PolicyUser, config::local_mode, local::evaluate_local};

// ─── Public API ──────────────────────────────────────────────────

/// Check whether the user is allowed to perform `action`.
///
/// Returns `true` if allowed, `false` if denied.
/// # Arguments
/// * `user` - Authenticated user and scopes.
/// * `action` - Permission string selected by the caller's route mapping.
/// * `resource` - Optional resource sent to the OPA sidecar.
/// # Returns
/// True when policy allows the action.
/// # Examples
/// ```no_run
/// # async fn example(user: &codetether_authorization_policy::PolicyUser) {
/// let _ = codetether_authorization_policy::check_policy(user, "tasks:read", None).await;
/// # }
/// ```
pub async fn check_policy(
    user: &PolicyUser,
    action: &str,
    resource: Option<&PolicyResource>,
) -> bool {
    // Local mode: evaluate in-process without OPA sidecar.
    if local_mode() {
        let allowed = evaluate_local(user, action);
        if !allowed {
            tracing::info!(
                user_id = %user.user_id,
                action = %action,
                "Local policy denied"
            );
        }
        return allowed;
    }

    crate::remote::check(user, action, resource).await
}

/// Enforce policy — returns `Ok(())` if allowed, `Err(StatusCode)` if denied.
/// # Arguments
/// * `user` - Authenticated user and scopes.
/// * `action` - Permission string selected by the caller's route mapping.
/// * `resource` - Optional resource sent to the OPA sidecar.
/// # Returns
/// Unit when allowed, or HTTP forbidden when denied.
/// # Errors
/// Returns HTTP forbidden when check_policy denies the action.
/// # Examples
/// ```no_run
/// # async fn example(user: &codetether_authorization_policy::PolicyUser) {
/// let _ = codetether_authorization_policy::enforce_policy(user, "tasks:read", None).await;
/// # }
/// ```
pub async fn enforce_policy(
    user: &PolicyUser,
    action: &str,
    resource: Option<&PolicyResource>,
) -> Result<(), axum::http::StatusCode> {
    if check_policy(user, action, resource).await {
        Ok(())
    } else {
        Err(axum::http::StatusCode::FORBIDDEN)
    }
}
