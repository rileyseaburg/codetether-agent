//! Installed reviewer configuration and provider handle storage.
//!
//! A [RuntimeStore] owns process-wide state. Install settings at application
//! startup, then call [RuntimeStore::enabled] before launching a reviewer.
use codetether_config::ReviewConfig;
use std::sync::{Arc, Mutex};
/// Reviewer settings and shared provider registry.
/// # Examples
/// ```
/// let runtime = codetether_approval_review::runtime::ReviewRuntime {
///     config: Default::default(), registry: std::sync::Arc::new(()) };
/// assert!(!runtime.config.enabled());
/// ```
pub struct ReviewRuntime<R> {
    /// Installed reviewer settings.
    pub config: ReviewConfig,
    /// Providers retained for subsequent reviewer runs.
    pub registry: Arc<R>,
}
impl<R> Clone for ReviewRuntime<R> {
    fn clone(&self) -> Self {
        Self {
            config: self.config.clone(),
            registry: self.registry.clone(),
        }
    }
}
/// Mutex-protected reviewer state, initialized empty.
/// # Examples
/// ```
/// let store = codetether_approval_review::runtime::RuntimeStore::<()>::new();
/// assert!(store.enabled().is_none());
/// ```
pub struct RuntimeStore<R>(Mutex<Option<ReviewRuntime<R>>>);
impl<R> RuntimeStore<R> {
    /// Create an empty reviewer store, suitable for a static initializer.
    pub const fn new() -> Self {
        Self(Mutex::new(None))
    }
    /// Install configuration and providers, replacing previous state.
    /// # Panics
    /// Panics if a prior writer poisoned the state mutex.
    pub fn install(&self, config: ReviewConfig, registry: Arc<R>) {
        *self.0.lock().expect("review runtime lock") = Some(ReviewRuntime { config, registry });
    }
    /// Clone installed handles only when reviewing is enabled.
    /// # Panics
    /// Panics if a prior writer poisoned the state mutex.
    pub fn enabled(&self) -> Option<ReviewRuntime<R>> {
        self.0
            .lock()
            .expect("review runtime lock")
            .clone()
            .filter(|runtime| runtime.config.enabled())
    }
    /// Forget installed state.
    /// # Panics
    /// Panics if a prior writer poisoned the state mutex.
    pub fn reset(&self) {
        *self.0.lock().expect("review runtime lock") = None;
    }
}
impl<R> Default for RuntimeStore<R> {
    fn default() -> Self {
        Self::new()
    }
}
