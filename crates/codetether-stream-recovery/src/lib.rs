//! Same-model stream-disconnect classification and reconnect policy.
//!
//! StreamRecoveryState decides whether to retain a retry prompt. Hosts own
//! submission and sleeping; backoff_secs and progress messages describe a retry.
mod backoff;
mod classify;
mod markers;
mod messages;
mod schedule;
mod state;
pub use backoff::backoff_secs;
pub use classify::is_stream_disconnect;
pub use messages::{reconnect_banner, reconnect_status};
pub use state::StreamRecoveryState;
/// Maximum same-model retries before the count is reset.
pub const MAX_RECONNECT_ATTEMPTS: u32 = 3;
#[cfg(test)]
mod classify_tests;
