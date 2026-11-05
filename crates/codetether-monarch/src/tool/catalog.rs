//! Bounded discovery of explicitly read-only remote tools.
use crate::client::Client;
use anyhow::Result;
use serde_json::{Value, json};

pub(super) fn readable(tool: &Value) -> bool {
    tool["annotations"]["readOnlyHint"] == true
}

pub(super) async fn catalog(client: &mut Client) -> Result<Vec<Value>> {
    let mut tools = Vec::new();
    let mut params = json!({});
    for _ in 0..20 {
        let page = client.rpc("tools/list", params).await?;
        let entries = page["tools"]
            .as_array()
            .ok_or_else(|| anyhow::anyhow!("Invalid Monarch tool catalog"))?;
        tools.extend(entries.iter().filter(|t| readable(t)).cloned());
        match page.get("nextCursor").filter(|v| !v.is_null()) {
            None => return Ok(tools),
            Some(Value::String(cursor)) => params = json!({"cursor":cursor}),
            Some(_) => anyhow::bail!("Invalid Monarch tool cursor"),
        }
    }
    anyhow::bail!("Monarch tool catalog pagination limit exceeded")
}
