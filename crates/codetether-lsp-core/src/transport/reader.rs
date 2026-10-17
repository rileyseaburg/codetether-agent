//! reader responsibility for the stdio LSP transport.

use super::*;

pub(super) fn start(
    stdout: tokio::process::ChildStdout,
    pending_clone: Arc<RwLock<HashMap<i64, oneshot::Sender<JsonRpcResponse>>>>,
    diagnostics_clone: Arc<RwLock<HashMap<String, Vec<lsp_types::Diagnostic>>>>,
    diag_publish_seq_clone: Arc<AtomicU64>,
    response_tx: mpsc::Sender<String>,
) {
    tokio::spawn(async move {
        let mut reader = BufReader::new(stdout);
        let mut header_buf = String::new();

        while let Some(body_buf) = frame::read(&mut reader, &mut header_buf).await {
            let body = String::from_utf8_lossy(&body_buf);
            trace!("LSP RX: {}", body);
            message::dispatch(
                &body,
                &pending_clone,
                &diagnostics_clone,
                &diag_publish_seq_clone,
                &response_tx,
            )
            .await;
        }
    });
}
