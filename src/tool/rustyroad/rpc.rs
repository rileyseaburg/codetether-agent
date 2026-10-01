//! Sequential JSON-RPC exchange for a single RustyRoad invocation.

use super::response;
use anyhow::Result;
use serde_json::{Value, json};
use tokio::{
    io::{AsyncWriteExt, BufReader},
    process::{ChildStdin, ChildStdout},
};

pub(super) struct Rpc {
    pub(super) stdin: ChildStdin,
    pub(super) stdout: BufReader<ChildStdout>,
    pub(super) next_id: i64,
}

impl Rpc {
    pub(super) async fn request(&mut self, method: &str, params: Value) -> Result<Value> {
        self.next_id += 1;
        let id = self.next_id;
        self.send(json!({"jsonrpc": "2.0", "id": id, "method": method, "params": params}))
            .await?;
        response::read(&mut self.stdout, id).await
    }
    pub(super) async fn initialized(&mut self) -> Result<()> {
        self.send(json!({"jsonrpc": "2.0", "method": "notifications/initialized"}))
            .await
    }
    async fn send(&mut self, message: Value) -> Result<()> {
        let mut bytes = serde_json::to_vec(&message)?;
        bytes.push(b'\n');
        self.stdin.write_all(&bytes).await?;
        self.stdin.flush().await?;
        Ok(())
    }
}
