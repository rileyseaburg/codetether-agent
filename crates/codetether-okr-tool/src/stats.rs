//! stats for OKR tool dispatch.
use super::*;
pub(super) async fn run(repo: &Arc<OkrRepository>, _p: Params) -> Result<ToolResult> {
    let stats = repo.stats().await?;
    Ok(ToolResult::success(serde_json::to_string_pretty(&stats)?))
}
