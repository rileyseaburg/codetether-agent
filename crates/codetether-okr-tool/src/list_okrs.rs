//! list okrs for OKR tool dispatch.
use super::*;
pub(super) async fn run(repo: &Arc<OkrRepository>, _p: Params) -> Result<ToolResult> {
    let okrs = repo.list_okrs().await?;
    if okrs.is_empty() {
        return Ok(ToolResult::success("No OKRs found"));
    }
    Ok(ToolResult::success(serde_json::to_string_pretty(&okrs)?)
        .with_metadata("count", json!(okrs.len())))
}
