//! Workspace-scoped tool registry construction.

use std::path::Path;
use std::sync::Arc;

use crate::provider::{Provider, ProviderRegistry};
use crate::tool::{ToolRegistry, bash, batch, lsp, search_router};

/// Build a provider registry whose shell/LSP tools target `cwd`.
pub(crate) fn registry_for_cwd(
    provider: Arc<dyn Provider>,
    model: &str,
    cwd: &Path,
) -> Arc<ToolRegistry> {
    let mut registry = ToolRegistry::with_provider(Arc::clone(&provider), model.to_string());
    registry.register(Arc::new(bash::BashTool::with_cwd(cwd.to_path_buf())));
    registry.register(Arc::new(lsp::LspTool::with_root(file_uri(cwd))));
    register_search(&mut registry, provider);
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

fn file_uri(path: &Path) -> String {
    let path = path.canonicalize().unwrap_or_else(|_| path.to_path_buf());
    let raw = path.to_string_lossy().replace('\\', "/");
    format!("file://{}", encode_path(&raw))
}

fn encode_path(path: &str) -> String {
    path.bytes().map(encode_byte).collect()
}

fn encode_byte(byte: u8) -> String {
    match byte {
        b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'/' | b'-' | b'_' | b'.' | b'~' => {
            (byte as char).to_string()
        }
        _ => format!("%{byte:02X}"),
    }
}
