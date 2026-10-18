//! Correlated tool request and response publication.
//!
//! Pass the optional session bus and a [Context] to [request] or [response].
//! Absent buses produce no events; responses preserve compact payload behavior.
mod context;
mod request;
mod response;
pub use context::Context;
pub use request::request;
pub use response::response;
