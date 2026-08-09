//! Dispatch inbound LSP responses, notifications, and server requests.

#[path = "transport_diagnostic_message.rs"]
mod diagnostic;
#[path = "transport_request.rs"]
mod request;

use std::collections::HashMap;
use std::sync::Arc;
use std::sync::atomic::AtomicU64;

use serde_json::Value;
use tokio::sync::{RwLock, mpsc, oneshot};
use tracing::debug;

use super::JsonRpcResponse;

pub(super) async fn dispatch(
    body: &str,
    pending: &Arc<RwLock<HashMap<i64, oneshot::Sender<JsonRpcResponse>>>>,
    diagnostics: &Arc<RwLock<HashMap<String, Vec<lsp_types::Diagnostic>>>>,
    publish_seq: &Arc<AtomicU64>,
    response_tx: &mpsc::Sender<String>,
) {
    let value = match serde_json::from_str::<Value>(body) {
        Ok(value) => value,
        Err(error) => {
            debug!(%error, body, "Failed to parse LSP message");
            return;
        }
    };
    if let Some(method) = value.get("method").and_then(Value::as_str) {
        if method == "textDocument/publishDiagnostics" {
            diagnostic::record(&value, diagnostics, publish_seq).await;
        } else if value.get("id").is_some() {
            request::respond(&value, response_tx).await;
        } else {
            debug!(method, "Ignoring unhandled LSP notification");
        }
        return;
    }
    let Ok(response) = serde_json::from_value::<JsonRpcResponse>(value) else {
        debug!(body, "Ignoring unrecognized LSP message");
        return;
    };
    let id = response.id;
    match pending.write().await.remove(&id) {
        Some(tx) => {
            if tx.send(response).is_err() {
                debug!(id, "LSP response receiver dropped");
            }
        }
        None => debug!(id, "Received response for unknown LSP request"),
    }
}
