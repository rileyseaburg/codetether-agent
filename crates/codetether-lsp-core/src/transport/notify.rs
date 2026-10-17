//! notify responsibility for the stdio LSP transport.

use super::*;

impl LspTransport {
    /// Send a notification (no response expected)
    pub async fn notify(&self, method: &str, params: Option<serde_json::Value>) -> Result<()> {
        let notification = JsonRpcNotification::new(method, params);
        let json = serde_json::to_string(&notification)?;
        self.tx.send(json).await?;
        Ok(())
    }
}
