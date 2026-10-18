//! Release steering resources before making executor ownership available.
use crate::RunGuard;

impl Drop for RunGuard {
    fn drop(&mut self) {
        let lease = self.execution.lease().clone();
        crate::CONTROL.finish_with(&lease, || {
            #[cfg(unix)]
            self._endpoint.take();
            codetether_session_steering::clear(lease.session_id());
        });
    }
}
