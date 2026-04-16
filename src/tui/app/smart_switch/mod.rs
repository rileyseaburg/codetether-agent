//! Smart model switching on provider errors.

pub mod candidates;
pub mod error_detection;
pub mod extra;
pub mod helpers;
pub mod models;
pub mod priority;
pub mod retry;
pub mod same_provider;

#[cfg(test)]
mod test_candidates;
#[cfg(test)]
mod tests;

pub use candidates::smart_switch_candidates;
pub use error_detection::{
    is_retryable_provider_error, normalize_provider_alias, smart_switch_model_key,
};
pub use helpers::{
    should_execute_smart_switch, smart_switch_max_retries,
};
pub use models::smart_switch_preferred_models;
pub use retry::{PendingSmartSwitchRetry, maybe_schedule_smart_switch_retry};

/// Sentinel value meaning "follow the latest message position".
pub const SCROLL_BOTTOM: usize = 1_000_000;
