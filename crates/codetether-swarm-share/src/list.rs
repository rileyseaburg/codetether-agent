//! list for shared swarm results.
use super::*;
impl SwarmShareTool {
    pub(super) async fn list(&self, _p: Params) -> Result<ToolResult> {
        let results = self.store.get_all().await;
        if results.is_empty() {
            return Ok(ToolResult::success(
                "No shared results in store".to_string(),
            ));
        }

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
            "{} shared results:\n{}",
            output.len(),
            serde_json::to_string_pretty(&output).unwrap_or_default()
        )))
    }
}
