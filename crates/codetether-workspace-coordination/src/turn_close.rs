//! Explicit prompt completion cancels renewal before releasing ownership.
use crate::{LeaseHost, LeaseTurn};
impl<H: LeaseHost> LeaseTurn<H> {
    /// Stop renewal and finish best-effort release before returning.
    /// # Examples
    /// See [LeaseTurn] for a complete open/close example.
    pub async fn close(mut self) {
        self.abort_renewal();
        let Some(owner) = self.owner.clone() else {
            return;
        };
        crate::release::run(self.host.as_ref(), &owner).await;
        self.owner = None;
    }

    pub(super) fn abort_renewal(&mut self) {
        if let Some(renewal) = self.renewal.take() {
            renewal.abort();
        }
    }
}
