//! delete okr for OKR tool dispatch.
use super::*;
pub(super) async fn run(repo: &Arc<OkrRepository>, p: Params) -> Result<ToolResult> {
    let id = parse_uuid(
        p.id.as_deref()
            .ok_or_else(|| anyhow::anyhow!("id required"))?,
        "id",
    )?;
    let deleted = repo.delete_okr(id).await?;
    if deleted {
        Ok(ToolResult::success(format!("Deleted OKR: {id}")))
    } else {
        Ok(ToolResult::error(format!("OKR not found: {id}")))
    }
}
