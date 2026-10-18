//! Cognition API access and shared error mapping.
use crate::AppState;
use codetether_cognition_api::http::{Beliefs, Lifecycle, Personas, Proposals, Workspace};
impl<R: Lifecycle + Personas + Beliefs + Workspace + Proposals + 'static>
    codetether_cognition_api::http::HttpState for AppState<R>
{
    type Runtime = R;
    fn runtime(&self) -> &Self::Runtime {
        &self.inner.cognition
    }
    fn map_error(error: anyhow::Error) -> (axum::http::StatusCode, String) {
        codetether_http_error::internal_error(error)
    }
}
