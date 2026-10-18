//! Routes for dynamic registry discovery and heartbeat.
use crate::{HttpState, http_handlers};
use axum::{
    Router,
    routing::{get, post},
};
/// Build discovery routes using the application's registry state.
/// # Returns
/// The v1 tool routes and MCP list alias, without applying authentication layers.
/// # Examples
/// ```no_run
/// # fn example<S: codetether_tool_discovery::HttpState>() {
/// let routes = codetether_tool_discovery::router::<S>();
/// # }
/// ```
pub fn router<S: HttpState>() -> Router<S> {
    Router::new()
        .route("/v1/tools", get(http_handlers::list_tools::<S>))
        .route(
            "/v1/tools/register",
            post(http_handlers::register_tool::<S>),
        )
        .route(
            "/v1/tools/{id}/heartbeat",
            post(http_handlers::tool_heartbeat::<S>),
        )
        // MCP compatibility alias (marketing site SDK expects /mcp/v1/tools)
        .route("/mcp/v1/tools", get(http_handlers::list_tools::<S>))
}
