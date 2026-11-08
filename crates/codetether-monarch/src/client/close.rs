//! Best-effort cleanup for one initialized remote MCP session.
use super::Client;

impl Client {
    pub(crate) async fn close(&self) {
        if let Some(session) = &self.session {
            let _ = self
                .http
                .delete(&self.endpoint)
                .bearer_auth(&self.token)
                .header("Mcp-Session-Id", session)
                .header("MCP-Protocol-Version", &self.version)
                .send()
                .await;
        }
    }
}
