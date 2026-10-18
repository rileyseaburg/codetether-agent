//! Native runtime and error-policy bindings for cognition HTTP routes.
mod beliefs;
mod lifecycle;
mod personas;
mod proposals;
mod workspace;
impl codetether_cognition_api::http::HttpState for super::AppState {
    type Runtime = crate::cognition::CognitionRuntime;
    fn runtime(&self) -> &Self::Runtime {
        &self.cognition
    }
    fn map_error(error: anyhow::Error) -> (axum::http::StatusCode, String) {
        codetether_http_error::internal_error(error)
    }
}
