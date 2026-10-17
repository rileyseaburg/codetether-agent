//! Discovery and JSON-RPC route registration.
use super::*;
/// Build routes that await application state and middleware.
///
/// # Returns
/// A router with both well-known card paths and the root JSON-RPC POST route.
///
/// # Examples
/// ```rust,no_run
/// # fn configure<H: codetether_a2a_http::server::ServerHost>(host: H) {
/// let router = codetether_a2a_http::server::routes::<H>().with_state(host);
/// # let _: axum::Router = router;
/// # }
/// ```
pub fn routes<H: ServerHost>() -> Router<H> {
    Router::new()
        .route("/.well-known/agent.json", get(card::get_agent_card::<H>))
        .route(
            "/.well-known/agent-card.json",
            get(card::get_agent_card::<H>),
        )
        .route("/", post(rpc::handle_rpc::<H>))
}
