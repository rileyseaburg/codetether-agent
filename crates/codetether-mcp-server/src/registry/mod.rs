//! Application tool filtering and MCP catalog registration.
//!
//! Implement [Host] over an existing tool registry and call [register] to
//! publish its remote-safe tools with the application's policy handlers.
mod host;
mod register;
pub use host::Host;
pub use register::register;
