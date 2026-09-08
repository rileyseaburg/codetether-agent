//! Isolated fake language-server configuration; never touches user settings.

use crate::{config::LspSettings, lsp::LspManager};
use serde_json::json;
use std::{path::Path, sync::Arc};

pub(super) fn manager(root: &Path, mode: &str) -> Arc<LspManager> {
    let script = root.join("lsp.py");
    std::fs::write(&script, include_str!("fake_server.py")).unwrap();
    let python = which::which("python3").expect("python3 is required for LSP process tests");
    let timeout_ms = if mode == "silent" { 500 } else { 30000 };
    let settings: LspSettings = serde_json::from_value(json!({
        "servers": {"typescript": {
            "command": python, "args": [script, mode],
            "file_extensions": ["ts"], "timeout_ms": timeout_ms
        }}, "disable_builtin_linters": true
    }))
    .unwrap();
    Arc::new(LspManager::with_config(
        Some(crate::lsp::path_to_uri(root)),
        settings,
    ))
}

pub(super) async fn ready(root: &Path) {
    tokio::time::timeout(std::time::Duration::from_secs(3), async {
        while super::cooldown::reason(root, "typescript").is_some() {
            tokio::time::sleep(std::time::Duration::from_millis(20)).await;
        }
    })
    .await
    .expect("background diagnostics did not finish");
}
