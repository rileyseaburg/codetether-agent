//! query tags for shared swarm results.
use super::*;
impl SwarmShareTool {
    pub(super) async fn query_tags(&self, p: Params) -> Result<ToolResult> {
        let tags = p
            .tags
            .ok_or_else(|| anyhow::anyhow!("tags required for query_tags"))?;

        let results = self.store.query_by_tags(&tags).await;
        let output: Vec<Value> = results
            .iter()
            .map(|r| {
                json!({
                    "key": r.key,
                    "producer": r.producer_id,
                    "type": r.schema.type_name,
                    "tags": r.tags,
                })
            })
            .collect();

        Ok(ToolResult::success(format!(
            "Found {} results matching tags {:?}:\n{}",
            output.len(),
            tags,
            serde_json::to_string_pretty(&output).unwrap_or_default()
        )))
    }
}
