//! create run for OKR tool dispatch.
use super::*;
pub(super) async fn run(repo: &Arc<OkrRepository>, p: Params) -> Result<ToolResult> {
    let okr_id = parse_uuid(
        p.okr_id
            .as_deref()
            .ok_or_else(|| anyhow::anyhow!("okr_id required"))?,
        "okr_id",
    )?;
    let name = p.name.ok_or_else(|| anyhow::anyhow!("name required"))?;
    let mut run = OkrRun::new(okr_id, &name);

    if let Some(cid) = p.correlation_id {
        run.correlation_id = Some(cid);
    }
    if let Some(sid) = p.session_id {
        run.session_id = Some(sid);
    }

    let created = repo.create_run(run).await?;
    Ok(ToolResult::success(serde_json::to_string_pretty(&created)?)
        .with_metadata("run_id", json!(created.id.to_string())))
}
