//! OpenAI completion HTTP routing, validation and provider selection.
//!
//! Implement [`Host`] and [`Registry`] for provider loading, then merge
//! [`router`] under the application's existing authentication and audit layers.
mod handler;
mod host;
mod routes;
pub use host::{Host, Registry};
pub use routes::router;
