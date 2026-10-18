//! Application runtime and error-policy access for cognition routes.
use super::{Beliefs, Lifecycle, Personas, Proposals, Workspace};
/// Application state exposing its cognition runtime and HTTP error mapping.
/// # Implementors
/// Native request state returns its existing runtime and preserves error policy.
pub trait HttpState: Clone + Send + Sync + 'static {
    /// Runtime operations grouped by their responsibility.
    type Runtime: Lifecycle + Personas + Beliefs + Workspace + Proposals;
    /// Return the application's existing cognition runtime.
    fn runtime(&self) -> &Self::Runtime;
    /// Convert a runtime failure using the application's existing HTTP policy.
    fn map_error(error: anyhow::Error) -> (axum::http::StatusCode, String);
}
