//! Authenticated JSON-RPC HTTP transport.
use super::*;
impl A2AClient {
    /// Make a JSON-RPC call
    pub async fn call_rpc(&self, request: JsonRpcRequest) -> Result<JsonRpcResponse> {
        let mut req = self.client.post(&self.base_url);

        if let Some(ref auth) = self.auth
            && let Some(ref creds) = auth.credentials
        {
            req = req.bearer_auth(creds);
        }

        let res = req
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;

        let response: JsonRpcResponse = res.json().await?;
        Ok(response)
    }
}
