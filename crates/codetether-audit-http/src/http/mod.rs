//! Audit history routing and request middleware adapters.
//!
//! Implement [`HttpState`] on application state, merge [`router`] into the
//! server and install [`audit_request`] at the server's audit middleware layer.
mod handlers;
mod routes;
mod state;
pub use handlers::audit_request;
pub use routes::router;
pub use state::HttpState;
