//! Bind worker session claims to shared provider registry selection.
use crate::{provider::ProviderRegistry, session::Session};
use anyhow::Result;
pub(super) async fn load_task_provider_registry(session: &Session) -> Result<ProviderRegistry> {
    ProviderRegistry::for_task_keys(session.metadata.provider_keys.as_ref()).await
}
