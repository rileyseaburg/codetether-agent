//! MCP server specialized to the application's native tool catalog.
/// Native MCP registration, approval and fallback services.
pub type McpServer = codetether_native_mcp_server::McpServer<
    crate::tool::registry_lifecycle::Native,
    PackageVersion,
>;
/// Executable package identity retained by native MCP initialization.
pub struct PackageVersion;
impl codetether_native_mcp_server::PackageVersion for PackageVersion {
    fn version() -> &'static str {
        env!("CARGO_PKG_VERSION")
    }
}
