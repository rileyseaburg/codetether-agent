//! Empty native registry for explicit offline test tools.
use codetether_native_registry::Catalog;
use codetether_provider_core::Provider;
use std::sync::Arc;
pub(super) type Application = codetether_native_registry::Application<TestCatalog>;
pub(super) type ToolRegistry = codetether_application_registry::ToolRegistry<Application>;
pub(super) struct TestCatalog;
impl Catalog for TestCatalog {
    fn populate(_: &mut ToolRegistry, _: Option<(Arc<dyn Provider>, String)>) {
        panic!("agent-loop fixtures must register their offline tools explicitly");
    }
}
