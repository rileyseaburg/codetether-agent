//! A single bounded request/response exchange.
use super::{failure::Failure, framing, process::Process};
use crate::tool::ToolResult;
use tokio::io::AsyncWriteExt;

impl Process {
    pub(super) async fn exchange(&mut self, request: &[u8]) -> Result<ToolResult, Failure> {
        tracing::info!(pid = ?self.child.id(), request_bytes = request.len(), "Sending desktop worker request");
        self.stdin
            .write_all(request)
            .await
            .map_err(|_| Failure::Transport)?;
        self.stdin
            .write_all(b"\n")
            .await
            .map_err(|_| Failure::Transport)?;
        self.stdin.flush().await.map_err(|_| Failure::Transport)?;
        let frame = framing::read_frame(&mut self.stdout, framing::RESPONSE_LIMIT)
            .await
            .map_err(|error| {
                if error.kind() == std::io::ErrorKind::InvalidData {
                    Failure::Framing
                } else {
                    Failure::Transport
                }
            })?
            .ok_or(Failure::Eof)?;
        tracing::info!(pid = ?self.child.id(), response_bytes = frame.len(), "Received desktop worker response frame");
        serde_json::from_slice(&frame).map_err(|_| Failure::Framing)
    }
}
