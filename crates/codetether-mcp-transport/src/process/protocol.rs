//! JSON-RPC operations for an owned subprocess.
use super::*;

#[async_trait]
impl Transport for ProcessTransport {
    async fn send_request(&self, request: JsonRpcRequest) -> Result<()> {
        self.send_json(serde_json::to_value(&request)?).await
    }

    async fn send_response(&self, response: JsonRpcResponse) -> Result<()> {
        self.send_json(serde_json::to_value(&response)?).await
    }

    async fn send_notification(&self, notification: JsonRpcNotification) -> Result<()> {
        self.send_json(serde_json::to_value(&notification)?).await
    }

    async fn receive(&self) -> Result<Option<McpMessage>> {
        let mut rx = self.rx.lock().await;
        match rx.recv().await {
            Some(line) => {
                let value: Value = serde_json::from_str(&line)?;
                let msg = McpMessage::from_json(value)?;
                Ok(Some(msg))
            }
            None => Ok(None),
        }
    }

    async fn close(&self) -> Result<()> {
        Ok(())
    }
}
