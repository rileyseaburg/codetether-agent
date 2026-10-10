//! Process-wide relay services: one serialized state, owner verifier, analyzer.
use crate::{Analyze, state::State};
use codetether_companion_core::{Error, OwnerCredential};
use std::path::PathBuf;
use std::sync::{Arc, Mutex, MutexGuard};
use std::time::Duration;

/// Shared relay handle used by the router.
pub type Shared = Arc<Relay>;

/// One memory-only relay per process; restart revokes every session.
pub struct Relay {
    pub(crate) state: Mutex<State>,
    pub(crate) owner: OwnerCredential,
    pub(crate) analyze: Analyze,
    pub(crate) origin: String,
    pub(crate) assets: Option<PathBuf>,
}
impl Relay {
    /// Build a relay from the owner token (at least 32 characters).
    ///
    /// # Arguments
    /// * `token` - Owner bearer credential; only its hash is retained.
    /// * `analyze` - Direct vision analyzer.
    /// * `origin` - Fixed browser origin allowed for mutations.
    /// * `assets` - Optional web shell directory served under `/companion/`.
    /// # Errors
    /// Returns [`Error::Configuration`] for a short token.
    pub fn new(
        token: &str,
        analyze: Analyze,
        origin: &str,
        assets: Option<PathBuf>,
    ) -> Result<Shared, Error> {
        Ok(Arc::new(Self {
            state: Mutex::new(State::default()),
            owner: OwnerCredential::new(token)?,
            analyze,
            origin: origin.to_string(),
            assets,
        }))
    }
    pub(crate) fn lock(&self) -> MutexGuard<'_, State> {
        self.state
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
    }
}
/// Trusted server time in Unix milliseconds.
pub fn now() -> i64 {
    chrono::Utc::now().timestamp_millis()
}
/// Sweep expired sessions every ten seconds.
pub fn spawn_sweeper(relay: Shared) -> tokio::task::JoinHandle<()> {
    tokio::spawn(async move {
        let mut tick = tokio::time::interval(Duration::from_secs(10));
        loop {
            tick.tick().await;
            relay.lock().sweep(now());
        }
    })
}
/// Stop every session, aborting analysis and closing streams.
pub fn shutdown(relay: &Relay) {
    relay.lock().end_all(now());
}
