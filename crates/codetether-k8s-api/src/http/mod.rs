//! Typed Kubernetes HTTP routes over caller-owned services.
//!
//! Implement [HttpState] and merge [router] and [worker_routes] beneath the
//! application's authentication and authorization layers.
mod commands;
mod queries;
mod routes;
mod state;
pub use routes::{router, worker_routes};
pub use state::HttpState;
