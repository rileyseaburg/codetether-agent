//! Authenticated application MCP connections configured by the host environment.
//!
//! [from_env] validates trusted connection settings and constructs an optional
//! [ApplicationMcpTool]. Configuration, invocation mapping, HTTP exchange and
//! tool metadata are owned by separate modules.
mod config;
mod construct;
mod environment;
mod invocation;
mod request;
mod schema;
mod tool_impl;
pub use environment::from_env;
/// Authenticated application MCP connection exposed as a local tool.
///
/// Construct through [from_env]; credentials remain private to the connection.
///
/// # Examples
/// ```rust,no_run
/// let _connection=codetether_application_mcp::from_env()?;
/// # Ok::<(),anyhow::Error>(())
/// ```

pub struct ApplicationMcpTool {
    id: String,
    name: String,
    description: String,
    endpoint: reqwest::Url,
    token: String,
    client: reqwest::Client,
}
