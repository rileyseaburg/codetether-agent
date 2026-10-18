//! OpenAI chat completion route assembly.
use super::{Host, handler};
use axum::{Router, routing::post};

/// Build the chat completion route using the application's provider loader.
/// # Returns
/// The OpenAI-compatible completion route for merging into an HTTP server.
/// # Examples
/// ```no_run
/// # fn example<H: codetether_openai_serving::http::Host>() {
/// let routes = codetether_openai_serving::http::router::<H, ()>();
/// # }
/// ```
pub fn router<H: Host, S: Clone + Send + Sync + 'static>() -> Router<S> {
    Router::new().route("/v1/chat/completions", post(handler::complete::<H>))
}
