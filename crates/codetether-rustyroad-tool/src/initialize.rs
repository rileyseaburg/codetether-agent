//! RustyRoad MCP handshake and server identity check.

use super::rpc::Rpc;
use anyhow::{Result, bail};
use codetether_mcp_core::types::{InitializeResult, ServerInfo};
use serde_json::json;

pub(super) async fn run(rpc: &mut Rpc, client_version: &str) -> Result<ServerInfo> {
    let result: InitializeResult = serde_json::from_value(
        rpc.request(
            "initialize",
            json!({
                "protocolVersion": "2024-11-05", "capabilities": {},
                "clientInfo": {"name": "codetether-rustyroad", "version": client_version}
            }),
        )
        .await?,
    )?;
    if result.server_info.name != "rustyroad-mcp" {
        bail!(
            "Expected rustyroad-mcp, received {}",
            result.server_info.name
        );
    }
    rpc.initialized().await?;
    Ok(result.server_info)
}
