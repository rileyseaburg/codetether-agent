//! Native catalog and registry bindings for shared MCP setup.
use super::*;
impl<H: Application> ApplicationServer<H> {
    /// Setup default tools (public, for CLI use)
    pub async fn setup_tools_public(&self) {
        self.setup_tools().await;
    }

    /// Register the configured tools through shared setup sequencing.
    pub(super) async fn setup_tools(&self) {
        crate::setup::run(self).await;
    }
}

impl<H: Application> crate::setup::Host for ApplicationServer<H> {
    type Registry = Arc<H::Registry>;

    fn catalog(&self) -> &ServerCatalog {
        &self.inner.catalog
    }

    fn registry(&self) -> Option<&Self::Registry> {
        self.inner.tool_registry.as_ref()
    }

    async fn register_registry(&self, registry: &Self::Registry) {
        self.register_registry_tools(registry).await;
    }

    async fn register_fallback(&self) {
        H::register_fallback(&self.inner.catalog).await;
    }
}
