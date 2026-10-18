//! Plugin discovery routing and application state access.
//!
//! Implement [`HttpState`] to supply the plugin registry and server identity,
//! then merge [`router`] beneath the server's existing middleware layers.
mod handler;
mod routes;
mod state;
pub use routes::router;
pub use state::HttpState;
