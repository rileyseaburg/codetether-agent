//! Verification fixtures using the same native authorization services.
use codetether_application_registry::ToolRegistry;
use codetether_native_registry::{Application, Catalog};
use codetether_provider_core::Provider;
use std::sync::Arc;
struct TestCatalog;
impl Catalog for TestCatalog {
    fn populate(_: &mut ToolRegistry<Application<Self>>, _: Option<(Arc<dyn Provider>, String)>) {}
}
pub(super) async fn run_story_verification(
    root: &std::path::Path,
    story: &codetether_ralph_core::UserStory,
) -> anyhow::Result<()> {
    super::run_story_verification::<Application<TestCatalog>>(root, story).await
}
