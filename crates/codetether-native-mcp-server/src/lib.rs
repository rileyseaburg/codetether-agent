//! Native MCP server identity, approval and executable-tool integration.
//!
//! [McpServer] uses the application tool catalog while [Native] preserves native
//! approval, command fallback and tool-call authorization. [PackageVersion]
//! retains the executable's own MCP version independently of this library.
use codetether_application_registry::Application;
use std::marker::PhantomData;
mod application;
mod fallback;
mod registry;
/// MCP server retaining application registry T and native execution policy.
/// # Examples
/// ```no_run
/// # fn example<T: codetether_application_registry::Application, V: codetether_native_mcp_server::PackageVersion>() {
/// let server = codetether_native_mcp_server::McpServer::<T, V>::new_local();
/// # drop(server); }
/// ```
#[allow(type_alias_bounds)]
pub type McpServer<T: Application, V: PackageVersion> =
    codetether_mcp_server::ApplicationServer<Native<T, V>>;
/// Native MCP protocol services for catalog T and package identity V.
/// # Examples
/// ```
/// let _: Option<codetether_native_mcp_server::Native<(), ()>> = None;
/// ```
pub struct Native<T, V>(PhantomData<fn() -> (T, V)>);
/// Version advertised by the embedding application during MCP initialization.
/// # Implementors
/// Return the executable's version rather than a library package version.
pub trait PackageVersion: 'static {
    /// MCP server version exposed to clients.
    fn version() -> &'static str;
}
#[cfg(test)]
mod tests;
