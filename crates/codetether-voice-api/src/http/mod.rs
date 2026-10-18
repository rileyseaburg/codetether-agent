//! Voice HTTP routes and application bus access.
//!
//! Implement [`HttpState`] for application state and merge [`router`] into
//! the server before applying its authentication, policy and audit layers.
mod handlers;
mod routes;
mod state;
pub use routes::router;
pub use state::HttpState;
