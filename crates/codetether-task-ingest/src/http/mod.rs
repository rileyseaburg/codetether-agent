//! HTTP task-ingress routing and application service access.
//!
//! Implement [`HttpState`] for the application's queue and audit log, then
//! merge [`router`] beneath its existing authentication and policy layers.
mod handler;
mod routes;
mod state;
pub use routes::router;
pub use state::HttpState;
