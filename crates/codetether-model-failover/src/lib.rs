//! Model failover classification, candidate ordering and retry policy.
//!
//! Use `maybe_schedule_smart_switch_retry` to select a pending retry from
//! provider failures, or query classification and candidate helpers directly.
//! Hosts retain model execution and delayed-retry lifecycle management.

pub mod account_state;
pub mod candidates;
pub mod error_detection;
pub mod extra;
pub mod helpers;
pub mod local_model;
pub mod models;
pub mod priority;
pub mod provider_filter;
pub mod retry;
pub mod same_provider;

#[cfg(test)]
mod account_state_tests;
#[cfg(test)]
mod test_candidates;
#[cfg(test)]
mod tests;

pub use account_state::is_provider_account_exhausted;
pub use candidates::smart_switch_candidates;
pub use error_detection::{
    codex_overload, codex_overload_cooldown, is_retryable_provider_error, normalize_provider_alias,
    smart_switch_model_key,
};
pub use helpers::{should_execute_smart_switch, smart_switch_max_retries};
pub use models::{codex_overload_fallback, smart_switch_preferred_models};
pub use retry::{PendingSmartSwitchRetry, maybe_schedule_smart_switch_retry};

pub mod constants;
mod overload_fallback;

/// Temporary overload fallback planning and restoration metadata.
pub mod temporary_fallback;

mod state;
pub use state::RetryState;

/// Session failover candidate ladders and delegation-aware target selection.
pub mod router;
