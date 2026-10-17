//! Abort the peer sweep when its lifecycle guard is dropped.
use super::AbortOnDrop;

impl Drop for AbortOnDrop {
    fn drop(&mut self) {
        self.0.abort();
    }
}
