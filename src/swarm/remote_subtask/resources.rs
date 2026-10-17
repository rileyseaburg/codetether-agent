use super::*;
pub(super) struct Resources {
    pub(super) _registry: ProviderRegistry,
    pub(super) provider: Arc<dyn crate::provider::Provider>,
    pub(super) tool_registry: Arc<crate::tool::ToolRegistry>,
    pub(super) tool_defs: Vec<crate::provider::ToolDefinition>,
}
pub(super) async fn resolve(
    payload: &RemoteSubtaskPayload,
    working_dir: &PathBuf,
) -> Result<Resources> {
    let registry = ProviderRegistry::from_vault().await?;
    let provider = registry
        .get(&payload.provider)
        .ok_or_else(|| anyhow!("Provider '{}' unavailable in remote pod", payload.provider))?;

    let capability =
        workspace_registry::Capability::from_flags(payload.read_only, payload.verification);
    let tool_registry = Arc::new(workspace_registry::with_provider(
        Arc::clone(&provider),
        payload.model.clone(),
        working_dir,
        capability,
    ));
    let tool_defs = tool_registry.definitions();

    Ok(Resources {
        _registry: registry,
        provider,
        tool_registry,
        tool_defs,
    })
}
