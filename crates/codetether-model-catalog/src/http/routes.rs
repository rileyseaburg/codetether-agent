//! Compatible model discovery route assembly.
use super::{Host, handlers};
use axum::{Router, routing::get};

/// Build all model discovery routes over an application provider loader.
/// # Returns
/// The two model-list aliases and the VS Code-specific discovery endpoint.
/// # Examples
/// ```no_run
/// # fn example<H: codetether_model_catalog::http::Host>() {
/// let routes = codetether_model_catalog::http::router::<H, ()>();
/// # }
/// ```
pub fn router<H: Host, S: Clone + Send + Sync + 'static>() -> Router<S> {
    Router::new()
        .route("/models", get(handlers::list_models::<H>))
        .route("/v1/models", get(handlers::list_models::<H>))
        .route("/v1/models/vscode", get(handlers::list_vscode_models::<H>))
}
