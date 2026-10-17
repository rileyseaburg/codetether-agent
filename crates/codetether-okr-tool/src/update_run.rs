//! update run for OKR tool dispatch.
use super::*;
pub(super) async fn run(repo: &Arc<OkrRepository>, p: Params) -> Result<ToolResult> {
    let id = parse_uuid(
        p.id.as_deref()
            .ok_or_else(|| anyhow::anyhow!("id required"))?,
        "id",
    )?;
    let mut run = repo
        .get_run(id)
        .await?
        .ok_or_else(|| anyhow::anyhow!("Run not found: {id}"))?;

    if let Some(name) = p.name {
        run.name = name;
    }
    if let Some(status_str) = p.status {
        run.status = parse_run_status(&status_str)?;
    }
    if let Some(cid) = p.correlation_id {
        run.correlation_id = Some(cid);
    }
    if let Some(sid) = p.session_id {
        run.session_id = Some(sid);
    }

    let updated = repo.update_run(run).await?;
    Ok(ToolResult::success(serde_json::to_string_pretty(&updated)?))
}
