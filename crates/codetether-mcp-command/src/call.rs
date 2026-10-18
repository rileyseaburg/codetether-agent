//! Handle the MCP call subcommand.
use crate::CommandOutput;
use codetether_mcp_connections::{ConnectionClient, Connections};
use serde_json::Value;
pub(crate) async fn run<C: ConnectionClient>(
    registry: &Connections<C>,
    value: &str,
    response: &mut CommandOutput,
) {
    let mut parts = value.trim().splitn(3, char::is_whitespace);
    let Some(server_name) = parts.next().filter(|part| !part.is_empty()) else {
        response.status = "Usage: /mcp call <server> <tool> [json]".to_string();
        return;
    };
    let Some(tool_name) = parts.next().filter(|part| !part.is_empty()) else {
        response.status = "Usage: /mcp call <server> <tool> [json]".to_string();
        return;
    };
    let arguments = match parts.next().map(str::trim).filter(|part| !part.is_empty()) {
        Some(raw_json) => match serde_json::from_str::<Value>(raw_json) {
            Ok(value) => value,
            Err(error) => {
                response.status = format!("Invalid MCP JSON args: {error}");
                return;
            }
        },
        None => Value::Object(Default::default()),
    };

    crate::call_result::run(registry, server_name, tool_name, arguments, response).await;
}
