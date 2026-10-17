//! query okrs for OKR tool dispatch.
use super::*;
pub(super) async fn run(repo: &Arc<OkrRepository>, p: Params) -> Result<ToolResult> {
    let okrs = if let Some(status_str) = p.status {
        let status = parse_okr_status(&status_str)?;
        repo.query_okrs_by_status(status).await?
    } else if let Some(owner) = p.owner {
        repo.query_okrs_by_owner(&owner).await?
    } else if let Some(tid) = p.tenant_id {
        repo.query_okrs_by_tenant(&tid).await?
    } else {
        anyhow::bail!("query_okrs requires status, owner, or tenant_id");
    };
    Ok(ToolResult::success(serde_json::to_string_pretty(&okrs)?)
        .with_metadata("count", json!(okrs.len())))
}
