//! Retry scheduling logic.

#[path = "retry/pending.rs"]
mod pending;
#[path = "retry/schedule.rs"]
mod schedule;

pub use pending::PendingSmartSwitchRetry;
pub use schedule::maybe_schedule_smart_switch_retry;
