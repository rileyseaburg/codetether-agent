//! Application-specific registry wrapping for the shared server.
use super::{Application, ApplicationServer, ServerCatalog};
use std::sync::Arc;
impl<H: Application> ApplicationServer<H> {
    pub(super) async fn register_registry_tools(&self, registry: &Arc<H::Registry>) {
        H::register_registry(&self.inner.catalog, registry).await;
    }
}
