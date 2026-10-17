//! publish for shared swarm results.
use super::*;
impl SwarmShareTool {
    pub(super) async fn publish(&self, p: Params) -> Result<ToolResult> {
        let key = p
            .key
            .ok_or_else(|| anyhow::anyhow!("key required for publish"))?;
        let value = p
            .value
            .ok_or_else(|| anyhow::anyhow!("value required for publish"))?;
        let tags = p.tags.unwrap_or_default();

        let result = self
            .store
            .publish(&key, &self.producer_id, value, tags, None)
            .await?;

        Ok(ToolResult::success(format!(
            "Published result '{}' (type: {})",
            key, result.schema.type_name
        )))
    }
}
