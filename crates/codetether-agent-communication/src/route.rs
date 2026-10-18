//! Result of attempting immediate same-turn delivery.

/// Routing decision for one inter-agent message.
#[derive(Debug, PartialEq, Eq)]
///
/// # Examples
/// ```
/// let result=codetether_agent_communication::Route::Idle;
/// assert_eq!(result,codetether_agent_communication::Route::Idle);
/// ```
pub enum Route {
    /// The active child accepted the message into its steering inbox.
    Steered,
    /// The child exists but is not currently accepting same-turn input.
    Idle,
    /// No child matching the caller's scope exists.
    NotFound,
}
