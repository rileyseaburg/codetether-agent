//! Resolve current providers and build the native autonomous tool registry.
use super::ApplicationHost;
use crate::{
    provider::Provider,
    tool::{ToolRegistry, agent::helpers},
};
use anyhow::Result;
use async_trait::async_trait;
use codetether_agent_spawn::EphemeralResources;
use std::{path::Path, sync::Arc};
#[async_trait]
impl EphemeralResources for ApplicationHost {
    type Registry = ToolRegistry;
    async fn resolve(model: &str) -> Result<(Arc<dyn Provider>, String)> {
        let providers = helpers::get_registry().await?;
        providers.resolve_model(model)
    }
    fn registry(
        read_only: bool,
        verification: bool,
        workspace: &Path,
        provider: Arc<dyn Provider>,
        model: String,
    ) -> Arc<ToolRegistry> {
        crate::tool::swarm_execute::agent_registry::standard(
            read_only,
            verification,
            workspace,
            provider,
            model,
        )
    }
}
