use std::sync::Arc;

use crate::provider::ProviderRegistry;

pub(super) async fn load_registry() -> Option<Arc<ProviderRegistry>> {
    super::secrets::init().await;
    ProviderRegistry::from_vault().await.ok().map(Arc::new)
}
