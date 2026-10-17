//! get for shared swarm results.
use super::*;
impl SwarmShareTool {
    pub(super) async fn get(&self, p: Params) -> Result<ToolResult> {
        let key = p
            .key
            .ok_or_else(|| anyhow::anyhow!("key required for get"))?;

        match self.store.get(&key).await {
            Some(result) => {
                let output = json!({
                    "key": result.key,
                    "producer": result.producer_id,
                    "value": result.value,
                    "type": result.schema.type_name,
                    "tags": result.tags,
                    "published_at": result.published_at.to_rfc3339(),
                });
                Ok(ToolResult::success(
                    serde_json::to_string_pretty(&output)
                        .unwrap_or_else(|_| format!("{:?}", result.value)),
                ))
            }
            None => Ok(ToolResult::error(format!("No result found for key: {key}"))),
        }
    }
}
