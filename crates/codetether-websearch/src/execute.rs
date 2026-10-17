//! execute for web search.
use super::*;
impl WebSearchTool {
    pub(crate) async fn run(&self, params: Value) -> Result<ToolResult> {
        let p: Params = serde_json::from_value(params).context("Invalid params")?;

        if p.query.trim().is_empty() {
            return Ok(ToolResult::error("Query cannot be empty"));
        }

        let results = self.search_ddg(&p.query, p.max_results).await?;

        if results.is_empty() {
            return Ok(ToolResult::success("No results found".to_string()));
        }

        let output = results
            .iter()
            .enumerate()
            .map(|(i, r)| {
                format!(
                    "{}. {}\n   URL: {}\n   {}",
                    i + 1,
                    r.title,
                    r.url,
                    r.snippet
                )
            })
            .collect::<Vec<_>>()
            .join("\n\n");

        Ok(ToolResult::success(output).with_metadata("count", json!(results.len())))
    }
}
