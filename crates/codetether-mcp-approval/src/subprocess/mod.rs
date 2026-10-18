//! Policy authorization before spawning an external MCP server.
//!
//! Use [guard] with a native [Host]. Arguments retain the current command
//! rendering and optional approval receipt; approved aliases are evaluated
//! only after the primary MCP policy blocks the invocation.
mod arguments;
mod host;
mod policy;
pub use arguments::policy_args;
pub use host::Host;
pub use policy::guard;
