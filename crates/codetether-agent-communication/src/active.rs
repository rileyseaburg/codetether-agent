//! Immediate delivery to an active child prompt loop.

use crate::{Host, Route};

/// Attempt same-turn delivery, waiting while a child starts running.
/// # Arguments
/// * `target` — Child name or durable ID.
/// * `owner` — Owning session scope, or the existing unscoped lookup mode.
/// * `message` — Plain text for the steering inbox.
/// # Returns
/// [Route::Steered] when accepted, [Route::Idle] when execution stops accepting
/// input, or [Route::NotFound] when the scoped target does not exist.
/// # Examples
/// ```rust,no_run
/// use codetether_agent_communication::{Host, Route, steer};
/// # async fn example<H: Host>() {
/// let route = steer::<H>("reviewer", Some("parent"), "Check the diff").await;
/// match route {
///     Route::Steered => {}, Route::Idle => {}, Route::NotFound => {},
/// }
/// # }
/// ```
pub async fn steer<H: Host>(target: &str, owner: Option<&str>, message: &str) -> Route {
    let Some(agent_id) = H::resolve(target, owner) else {
        return Route::NotFound;
    };
    if super::wait::steer_or_idle::<H>(&agent_id, message).await {
        Route::Steered
    } else {
        Route::Idle
    }
}
