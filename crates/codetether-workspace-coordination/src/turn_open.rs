//! Start prompt-lifetime renewal only when the coordinator is active.
use crate::{LeaseHost, LeaseTurn};
use std::sync::Arc;
impl<H: LeaseHost> LeaseTurn<H> {
    /// Start renewal for an active coordinator and retain release ownership.
    /// # Arguments
    /// `host` supplies coordinator operations; `owner` identifies this prompt.
    /// # Returns
    /// A guard that cancels renewal and releases its leases on close or drop.
    /// # Panics
    /// An active coordinator requires a Tokio runtime to spawn renewal.
    /// # Examples
    /// See [LeaseTurn] for a complete open/close example.
    pub fn open(host: Arc<H>, owner: &str) -> Self {
        if !host.active() {
            return Self {
                owner: None,
                renewal: None,
                host,
            };
        }
        let owner = owner.to_string();
        let renewal = crate::renewal::start(host.clone(), owner.clone());
        Self {
            owner: Some(owner),
            renewal: Some(renewal),
            host,
        }
    }
}
