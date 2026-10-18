//! MCP server dispatch and transport loop.
//!
//! Implement [Host] with an existing catalog, transport and invocation policy.
//! [run] registers tools then serves requests until transport closure. [request]
//! and [notification] expose the same dispatch for embedded callers.
mod host;
mod notification;
mod request;
mod run;
pub use host::Host;
pub use notification::notification;
pub use request::request;
pub use run::run;

mod invocation;
pub use invocation::invoke;
/// Bounded MCP bus payloads.
pub mod payload;
/// Application tool registration and filtering.
pub mod registry;

/// Registry selection and tool-registration sequencing.
pub mod setup;

mod state;
pub use state::ServerState;
