//! Retain initialized server services in request state without reconstructing them.
use crate::AppState;
use codetether_server_runtime::Parts;
impl<R> From<Parts<R>> for AppState<R> {
    /// Move the initialized service bundle into shared HTTP request state.
    fn from(inner: Parts<R>) -> Self {
        Self { inner }
    }
}
