//! Workspace-scoped tool registry construction.

use std::path::Path;
use std::sync::Arc;

use crate::provider::{Provider, ProviderRegistry};
use crate::tool::{ToolRegistry, application_mcp, batch, command_session, lsp, search_router};

#[path = "workspace_tools/autonomous.rs"]
mod autonomous;
#[cfg(test)]
#[path = "workspace_tools/autonomous_tests.rs"]
mod autonomous_tests;
#[path = "workspace_tools/file_uri.rs"]
mod file_uri;

/// Build a provider registry whose shell/LSP tools target `cwd`.
pub(crate) fn registry_for_cwd(
    provider: Arc<dyn Provider>,
    model: &str,
    cwd: &Path,
    prior_context_allowed: bool,
    autonomous: bool,
) -> Arc<ToolRegistry> {
    let mut registry = ToolRegistry::with_provider(Arc::clone(&provider), model.to_string());
    application_mcp::register(&mut registry);
    command_session::register(&mut registry, Some(cwd.to_path_buf()));
    registry.register(Arc::new(lsp::LspTool::with_root(file_uri::build(cwd))));
    register_search(&mut registry, provider);
    if !prior_context_allowed {
        super::runtime::remove_prior_context_tools(&mut registry);
    }
    if autonomous {
        autonomous::restrict(&mut registry);
    }
    with_batch(registry)
}

fn register_search(registry: &mut ToolRegistry, provider: Arc<dyn Provider>) {
    let mut providers = ProviderRegistry::new();
    providers.register(provider);
    registry.register(Arc::new(search_router::SearchTool::new(Arc::new(
        providers,
    ))));
}

fn with_batch(mut registry: ToolRegistry) -> Arc<ToolRegistry> {
    let batch_tool = Arc::new(batch::BatchTool::new());
    registry.register(batch_tool.clone());
    let registry = Arc::new(registry);
    batch_tool.set_registry(Arc::downgrade(&registry));
    registry
}
