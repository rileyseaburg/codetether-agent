//! Shared auto-chat request, relay memory and handoff policies.
//!
//! Parse requests with [parse_autochat_request], accumulate [shared_context]
//! deltas, and receive correlated handoffs with [transport].
mod constants;
pub(crate) use constants::REQUIRED_RELAY_CAPABILITIES;
pub use constants::{
    AUTOCHAT_DEFAULT_AGENTS, AUTOCHAT_MAX_AGENTS, AUTOCHAT_MAX_DYNAMIC_SPAWNS, AUTOCHAT_MAX_ROUNDS,
    AUTOCHAT_NO_PRD_FLAG, AUTOCHAT_QUICK_DEMO_TASK, AUTOCHAT_RLM_FALLBACK_CHARS,
    AUTOCHAT_RLM_HANDOFF_QUERY, AUTOCHAT_RLM_THRESHOLD_CHARS, AUTOCHAT_SPAWN_CHECK_MIN_CHARS,
    AUTOCHAT_STATUS_MAX_ROUNDS_REACHED,
};
mod request;
pub use request::ParsedAutochatRequest;
mod ensure_required_relay_capabilities;
pub use ensure_required_relay_capabilities::ensure_required_relay_capabilities;
mod parse_autochat_request;
pub use parse_autochat_request::parse_autochat_request;
mod parse_autochat_args;
pub use parse_autochat_args::parse_autochat_args;
mod normalize_for_convergence;
pub use normalize_for_convergence::normalize_for_convergence;
/// Shared context contracts, merging and prompt rendering.
pub mod shared_context;
#[cfg(test)]
mod tests;
/// Correlated handoff subscriptions and reception.
pub mod transport;

/// Shared provider and model rotation policy.
pub mod model_rotation;

/// Stable posterior-based ordering of auto-chat model candidates.
pub mod delegation;
