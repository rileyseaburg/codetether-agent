//! list runs for OKR tool dispatch.
use super::*;
pub(super) async fn run(repo: &Arc<OkrRepository>, _p: Params) -> Result<ToolResult> {
    let runs = repo.list_runs().await?;
    if runs.is_empty() {
        return Ok(ToolResult::success("No runs found"));
    }
    Ok(ToolResult::success(serde_json::to_string_pretty(&runs)?)
        .with_metadata("count", json!(runs.len())))
}
