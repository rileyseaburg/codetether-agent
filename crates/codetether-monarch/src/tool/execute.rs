//! Credential acquisition and lifetime of one remote tool session.
use super::{input::Input, run::run};
use crate::client::Client;
use anyhow::Result;
use codetether_tool_core::ToolResult;
use serde_json::{Value, json};

pub(super) async fn execute(args: Value) -> Result<ToolResult> {
    let input = super::input::parse(args)?;
    if matches!(input, Input::Status {}) {
        return Ok(ToolResult::success(
            json!({"credentials_present":codetether_monarch_auth::status().await?,
                "login_command":"codetether auth monarch login", "mode":"read_only"})
            .to_string(),
        ));
    }
    if matches!(input, Input::Refresh {}) {
        codetether_monarch_auth::refresh().await?;
        return Ok(ToolResult::success(
            json!({"credentials_refreshed":true,"remote_access_checked":false,
                "mode":"read_only","next_action":"list_tools"})
            .to_string(),
        ));
    }
    let token = codetether_monarch_auth::access_token().await?;
    let mut client = Client::new("https://api.monarch.com/mcp".into(), token)?;
    let result = run(&mut client, input).await;
    client.close().await;
    result
}
