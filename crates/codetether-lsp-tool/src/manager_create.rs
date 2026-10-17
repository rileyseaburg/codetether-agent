//! Apply explicit and loaded configuration when creating a manager.
use super::{Host, LspTool};
use codetether_lsp_core::manager::LspManager;
use std::sync::Arc;
pub(super) async fn create<H: Host>(tool: &LspTool<H>) -> Arc<LspManager<H::Client>> {
    let manager = if let Some(settings) = &tool.lsp_settings {
        Arc::new(LspManager::with_config(
            tool.root_uri.clone(),
            settings.clone(),
        ))
    } else {
        match codetether_config::Config::load().await {
            Ok(config) if has_lsp_settings(&config.lsp) => {
                Arc::new(LspManager::with_config(tool.root_uri.clone(), config.lsp))
            }
            _ => Arc::new(LspManager::new(tool.root_uri.clone())),
        }
    };

    manager
}

/// Returns `true` if the user has configured any LSP settings.
fn has_lsp_settings(settings: &codetether_config::LspSettings) -> bool {
    !settings.servers.is_empty() || !settings.linters.is_empty() || settings.disable_builtin_linters
}
