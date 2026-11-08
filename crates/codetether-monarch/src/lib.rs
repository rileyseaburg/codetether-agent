//! Official Monarch MCP access without private GraphQL or browser credentials.
//!
//! [`MonarchTool`] is available before login but never starts interactive OAuth
//! from an agent call. Run `codetether auth monarch` with the intended configured
//! secret backend. Only read-only remote tools are exposed by this connector.
mod client;
mod response;
mod tool;
pub use tool::MonarchTool;

#[cfg(test)]
mod tests;
