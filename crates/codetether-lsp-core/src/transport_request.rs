//! Respond to language-server requests required for diagnostics startup.

use serde_json::{Value, json};
use tokio::sync::mpsc;
use tracing::{debug, warn};

pub(super) async fn respond(message: &Value, tx: &mpsc::Sender<String>) {
    let Some(id) = message.get("id").cloned() else {
        return;
    };
    let method = message
        .get("method")
        .and_then(Value::as_str)
        .unwrap_or_default();
    let response = match method {
        "workspace/configuration" => success(id, configuration(message)),
        "client/registerCapability"
        | "client/unregisterCapability"
        | "window/workDoneProgress/create" => success(id, Value::Null),
        "workspace/workspaceFolders" | "window/showMessageRequest" => success(id, Value::Null),
        "workspace/applyEdit" => success(id, json!({"applied": false})),
        _ => json!({
            "jsonrpc": "2.0",
            "id": id,
            "error": {"code": -32601, "message": format!("Unsupported client method: {method}")}
        }),
    };
    match serde_json::to_string(&response) {
        Ok(encoded) => {
            if tx.send(encoded).await.is_err() {
                warn!(method, "Failed to send LSP server-request response");
            } else {
                debug!(method, "Responded to LSP server request");
            }
        }
        Err(error) => warn!(method, %error, "Failed to encode LSP server-request response"),
    }
}

fn configuration(message: &Value) -> Value {
    let count = message.pointer("/params/items").and_then(Value::as_array);
    json!(vec![Value::Null; count.map(Vec::len).unwrap_or(0)])
}

fn success(id: Value, result: Value) -> Value {
    json!({"jsonrpc": "2.0", "id": id, "result": result})
}
