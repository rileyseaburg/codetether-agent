//! Workspace-scoped tool registry construction.

use std::path::Path;
use std::sync::Arc;

use crate::provider::Provider;
use crate::tool::{ToolRegistry, bash, lsp};

/// Build a provider registry whose shell/LSP tools target `cwd`.
pub(crate) fn registry_for_cwd(
    provider: Arc<dyn Provider>,
    model: &str,
    cwd: &Path,
) -> Arc<ToolRegistry> {
    let mut registry = ToolRegistry::with_provider(provider, model.to_string());
    registry.register(Arc::new(bash::BashTool::with_cwd(cwd.to_path_buf())));
    registry.register(Arc::new(lsp::LspTool::with_root(format!(
        "file://{}",
        cwd.display()
    ))));
    Arc::new(registry)
}
