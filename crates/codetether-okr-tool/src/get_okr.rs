//! get okr for OKR tool dispatch.
use super::*;
pub(super) async fn run(repo: &Arc<OkrRepository>, p: Params) -> Result<ToolResult> {
    let id = parse_uuid(
        p.id.as_deref()
            .ok_or_else(|| anyhow::anyhow!("id required"))?,
        "id",
    )?;
    match repo.get_okr(id).await? {
        Some(okr) => Ok(ToolResult::success(serde_json::to_string_pretty(&okr)?)),
        None => Ok(ToolResult::error(format!("OKR not found: {id}"))),
    }
}
