//! Bind initialized services to native application request state.
use crate::cognition::CognitionRuntime;
use codetether_server_runtime::Parts;
pub(super) fn build(parts: Parts<CognitionRuntime>) -> crate::server::AppState {
    crate::server::AppState { inner: parts }
}
