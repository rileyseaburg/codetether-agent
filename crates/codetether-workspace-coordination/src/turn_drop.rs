//! Cancellation fallback schedules release after stopping renewal.
use crate::{LeaseHost, LeaseTurn};
impl<H: LeaseHost> Drop for LeaseTurn<H> {
    fn drop(&mut self) {
        self.abort_renewal();
        let Some(owner) = self.owner.take() else {
            return;
        };
        crate::release::spawn(self.host.clone(), owner);
    }
}
