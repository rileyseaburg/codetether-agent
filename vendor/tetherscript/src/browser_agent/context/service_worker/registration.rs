//! Service-worker registration model.

/// Deterministic service-worker lifecycle state.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ServiceWorkerState {
    /// Registration exists but is not yet activated.
    Installing,
    /// Registration is activated and can control matching fetches.
    Active,
    /// Registration is no longer eligible for fetch interception.
    Redundant,
}

impl ServiceWorkerState {
    pub(crate) fn is_active(self) -> bool {
        self == Self::Active
    }
}

/// One origin and scope-bound service-worker registration.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ServiceWorkerRegistration {
    /// Origin key such as `https://example.test`.
    pub origin: String,
    /// Absolute scope URL controlled by the worker.
    pub scope: String,
    /// Absolute script URL used to register the worker.
    pub script_url: String,
    /// Current deterministic lifecycle state.
    pub state: ServiceWorkerState,
}

impl ServiceWorkerRegistration {
    pub(crate) fn new(origin: &str, scope: String, script_url: String) -> Self {
        Self {
            origin: origin.into(),
            scope,
            script_url,
            state: ServiceWorkerState::Installing,
        }
    }
}
