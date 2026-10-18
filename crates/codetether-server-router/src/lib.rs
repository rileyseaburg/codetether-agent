//! Application HTTP route composition and mandatory middleware.
//!
//! Implement [`HttpState`] on the application's state and supply native
//! adapter routes through [`Extensions`]. [`build`] combines them with shared
//! API routes, A2A transports and the existing middleware stack.
mod extensions;
mod layers;
mod routes;
mod state;
mod transports;
pub use extensions::Extensions;
pub use routes::build;
pub use state::HttpState;
pub use transports::Transports;

mod workers;
pub use workers::worker_routes;
