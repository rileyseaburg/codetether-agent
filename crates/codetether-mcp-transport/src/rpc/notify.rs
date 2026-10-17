//! Send uncorrelated notifications through the connection transport.
use super::RpcConnection;
use anyhow::Result;
use codetether_mcp_core::types::JsonRpcNotification;
use serde_json::Value;
impl RpcConnection {
    /// Send a notification without registering a response channel.
    ///
    /// # Arguments
    /// * `method` — Remote notification method.
    /// * `params` — Optional serialized arguments.
    /// # Returns
    /// Unit after the transport accepts the notification.
    /// # Errors
    /// Returns transport send errors.
    pub async fn notify(&self, method: &str, params: Option<Value>) -> Result<()> {
        let notification = JsonRpcNotification::new(method, params);
        self.transport.send_notification(notification).await
    }
}
