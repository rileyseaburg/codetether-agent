//! Authenticated bus event streaming over HTTP.
//!
//! Implement [`HttpState`] for application state and merge [`router`] under
//! the existing authentication middleware that supplies JWT claims.
mod handler;
mod routes;
mod state;
pub use routes::router;
pub use state::HttpState;
