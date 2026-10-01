//! RustyRoad MCP handshake and server identity check.

use super::rpc::Rpc;
use crate::mcp::{InitializeResult, ServerInfo};
use anyhow::{Result, bail};
use serde_json::json;

pub(super) async fn run(rpc: &mut Rpc) -> Result<ServerInfo> {
    let result: InitializeResult = serde_json::from_value(
        rpc.request(
            "initialize",
            json!({
                "protocolVersion": "2024-11-05", "capabilities": {},
                "clientInfo": {"name": "codetether-rustyroad", "version": env!("CARGO_PKG_VERSION")}
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
