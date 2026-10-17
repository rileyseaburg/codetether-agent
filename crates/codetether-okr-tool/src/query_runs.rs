//! query runs for OKR tool dispatch.
use super::*;
pub(super) async fn run(repo: &Arc<OkrRepository>, p: Params) -> Result<ToolResult> {
    let runs = if let Some(okr_id_str) = p.okr_id {
        let okr_id = parse_uuid(&okr_id_str, "okr_id")?;
        repo.query_runs_by_okr(okr_id).await?
    } else if let Some(status_str) = p.status {
        let status = parse_run_status(&status_str)?;
        repo.query_runs_by_status(status).await?
    } else if let Some(cid) = p.correlation_id {
        repo.query_runs_by_correlation(&cid).await?
    } else if let Some(cpid) = p.checkpoint_id {
        repo.query_runs_by_checkpoint(&cpid).await?
    } else if let Some(sid) = p.session_id {
        repo.query_runs_by_session(&sid).await?
    } else {
        anyhow::bail!(
            "query_runs requires okr_id, status, correlation_id, checkpoint_id, or session_id"
        );
    };
    Ok(ToolResult::success(serde_json::to_string_pretty(&runs)?)
        .with_metadata("count", json!(runs.len())))
}
