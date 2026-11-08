//! Assign request IDs before one MCP JSON-RPC call.
use super::Client;
use anyhow::Result;
use serde_json::{Value, json};

impl Client {
    pub(crate) async fn rpc(&mut self, method: &str, params: Value) -> Result<Value> {
        self.next_id += 1;
        self.post(
            json!({"jsonrpc":"2.0", "id":self.next_id, "method":method, "params":params}),
            Some(self.next_id),
        )
        .await
    }
}
