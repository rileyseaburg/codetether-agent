//! query prefix for shared swarm results.
use super::*;
impl SwarmShareTool {
    pub(super) async fn query_prefix(&self, p: Params) -> Result<ToolResult> {
        let prefix = p
            .prefix
            .ok_or_else(|| anyhow::anyhow!("prefix required for query_prefix"))?;

        let results = self.store.query_by_prefix(&prefix).await;
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
            "Found {} results with prefix '{}':\n{}",
            output.len(),
            prefix,
            serde_json::to_string_pretty(&output).unwrap_or_default()
        )))
    }
}
