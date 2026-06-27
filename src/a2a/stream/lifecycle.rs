//! Connection lifecycle state machine for the worker stream (Phase 5).
//!
//! Models the reconnect lifecycle explicitly instead of leaving it implicit in a
//! `loop { connect; sleep }`. The state is driven by connect outcomes and the
//! circuit breaker. See `docs/transport-first-class-plan.md` Phase 5.

/// Observable connection lifecycle state.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::a2a::stream::lifecycle::{LifecycleState, next_state};
///
/// // A successful connect moves Connecting -> Live.
/// assert_eq!(next_state(LifecycleState::Connecting, true, false), LifecycleState::Live);
/// // A failure with the breaker still closed -> Backoff.
/// assert_eq!(next_state(LifecycleState::Live, false, false), LifecycleState::Backoff);
/// // A failure with the breaker open -> Dead.
/// assert_eq!(next_state(LifecycleState::Live, false, true), LifecycleState::Dead);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LifecycleState {
    /// Attempting to establish the stream.
    Connecting,
    /// Stream established and delivering events.
    Live,
    /// Stream dropped; will reconnect after backoff.
    Backoff,
    /// Breaker tripped; server appears hard-down.
    Dead,
}

/// Compute the next lifecycle state from the current state and the last
/// connect outcome.
///
/// `connected` is whether the most recent connect attempt succeeded;
/// `breaker_open` is whether the circuit breaker has tripped.
pub fn next_state(_current: LifecycleState, connected: bool, breaker_open: bool) -> LifecycleState {
    if connected {
        LifecycleState::Live
    } else if breaker_open {
        LifecycleState::Dead
    } else {
        LifecycleState::Backoff
    }
}

#[cfg(test)]
mod tests {
    use super::{LifecycleState, next_state};

    #[test]
    fn success_goes_live() {
        assert_eq!(
            next_state(LifecycleState::Backoff, true, false),
            LifecycleState::Live
        );
    }

    #[test]
    fn failure_backs_off_until_breaker_opens() {
        assert_eq!(
            next_state(LifecycleState::Live, false, false),
            LifecycleState::Backoff
        );
        assert_eq!(
            next_state(LifecycleState::Backoff, false, true),
            LifecycleState::Dead
        );
    }
}
