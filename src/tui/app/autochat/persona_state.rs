//! Process-wide [`DelegationState`] for autochat persona ranking.
//!
//! [`run_relay`] would otherwise spin up a fresh state per call, so
//! LCB ranking would always fall back to cold-start. This holder lets
//! ranks accumulate evidence across relays in the same process so the
//! `record_outcome` calls in [`run`] are observable on the next run.
//!
//! [`run_relay`]: super::run::run_relay
//! [`run`]: super::run

use std::sync::{Mutex, MutexGuard, OnceLock};

use crate::session::delegation::DelegationState;

static STATE: OnceLock<Mutex<DelegationState>> = OnceLock::new();

fn instance() -> &'static Mutex<DelegationState> {
    STATE.get_or_init(|| Mutex::new(DelegationState::default()))
}

/// Borrow the shared state, recovering from a poisoned guard so a panic
/// in one relay never deadlocks subsequent ones.
pub fn lock() -> MutexGuard<'static, DelegationState> {
    instance().lock().unwrap_or_else(|p| p.into_inner())
}
