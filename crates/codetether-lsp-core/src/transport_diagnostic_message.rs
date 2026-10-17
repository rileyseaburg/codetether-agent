//! Record `textDocument/publishDiagnostics` notifications.

use std::collections::HashMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

use serde_json::Value;
use tokio::sync::RwLock;

pub(super) async fn record(
    message: &Value,
    cache: &Arc<RwLock<HashMap<String, Vec<lsp_types::Diagnostic>>>>,
    publish_seq: &Arc<AtomicU64>,
) {
    let Some(params) = message.get("params") else {
        return;
    };
    let uri = params
        .get("uri")
        .and_then(Value::as_str)
        .unwrap_or_default()
        .to_string();
    let diagnostics = params
        .get("diagnostics")
        .cloned()
        .and_then(|value| serde_json::from_value(value).ok())
        .unwrap_or_default();
    if !uri.is_empty() {
        cache.write().await.insert(uri, diagnostics);
        publish_seq.fetch_add(1, Ordering::SeqCst);
    }
}
