//! Offline native registry used by the MCP image-wire regression.
use codetether_native_registry::Catalog;
use codetether_provider_core::Provider;
use std::sync::Arc;
struct TestCatalog;
type Application = codetether_native_registry::Application<TestCatalog>;
type ToolRegistry = codetether_application_registry::ToolRegistry<Application>;
type McpServer = crate::McpServer<Application, TestVersion>;
struct TestVersion;
impl crate::PackageVersion for TestVersion {
    fn version() -> &'static str {
        "test"
    }
}
impl Catalog for TestCatalog {
    fn populate(_: &mut ToolRegistry, _: Option<(Arc<dyn Provider>, String)>) {
        panic!("MCP image test explicitly registers its tool");
    }
}
mod image_server_tests;
