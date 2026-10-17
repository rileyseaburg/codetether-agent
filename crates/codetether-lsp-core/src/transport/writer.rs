//! writer responsibility for the stdio LSP transport.

use super::*;

pub(super) fn start(
    mut stdin: tokio::process::ChildStdin,
    mut write_rx: mpsc::Receiver<String>,
    pending_clone: Arc<RwLock<HashMap<i64, oneshot::Sender<JsonRpcResponse>>>>,
) {
    tokio::spawn(async move {
        while let Some(msg) = write_rx.recv().await {
            let content_length = msg.len();
            let header = format!("Content-Length: {}\r\n\r\n", content_length);
            trace!("LSP TX header: {}", header.trim());
            trace!("LSP TX body: {}", msg);

            if let Err(e) = stdin.write_all(header.as_bytes()).await {
                error!("Failed to write header to LSP server: {}", e);
                break;
            }
            if let Err(e) = stdin.write_all(msg.as_bytes()).await {
                error!("Failed to write body to LSP server: {}", e);
                break;
            }
            if let Err(e) = stdin.flush().await {
                error!("Failed to flush LSP server stdin: {}", e);
                break;
            }
        }
        pending_clone.write().await.clear();
    });
}
