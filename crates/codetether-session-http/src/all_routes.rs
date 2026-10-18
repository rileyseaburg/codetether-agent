//! Compose persisted-session, goal and fork routes under the caller's middleware.
use crate::GoalHost;
use codetether_session_fork::ForkSession;

/// Build the complete non-realtime session HTTP surface.
/// # Arguments
/// `H` selects session storage and goal/fork operations; `S` is the router state.
/// # Returns
/// Fork routes, then goal routes, then ordinary persisted-session routes.
/// # Examples
/// ```no_run
/// # fn example<H: codetether_session_http::GoalHost + codetether_session_fork::ForkSession>() {
/// let _router = codetether_session_http::all_routes::<H, ()>();
/// # }
/// ```
pub fn all_routes<H: GoalHost + ForkSession, S: Clone + Send + Sync + 'static>() -> axum::Router<S>
{
    axum::Router::new()
        .merge(crate::fork_router::<H, S>())
        .merge(crate::goal_router::<H, S>())
        .merge(crate::router::<H, S>())
}
