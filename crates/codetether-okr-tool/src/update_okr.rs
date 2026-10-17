//! update okr for OKR tool dispatch.
use super::*;
pub(super) async fn run(repo: &Arc<OkrRepository>, p: Params) -> Result<ToolResult> {
    let id = parse_uuid(
        p.id.as_deref()
            .ok_or_else(|| anyhow::anyhow!("id required"))?,
        "id",
    )?;
    let mut okr = repo
        .get_okr(id)
        .await?
        .ok_or_else(|| anyhow::anyhow!("OKR not found: {id}"))?;

    if let Some(title) = p.title {
        okr.title = title;
    }
    if let Some(desc) = p.description {
        okr.description = desc;
    }
    if let Some(status_str) = p.status {
        okr.status = parse_okr_status(&status_str)?;
    }
    if let Some(owner) = p.owner {
        okr.owner = Some(owner);
    }
    if let Some(tid) = p.tenant_id {
        okr.tenant_id = Some(tid);
    }

    let updated = repo.update_okr(okr).await?;
    Ok(ToolResult::success(serde_json::to_string_pretty(&updated)?))
}
