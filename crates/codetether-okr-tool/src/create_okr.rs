//! create okr for OKR tool dispatch.
use super::*;
pub(super) async fn run(repo: &Arc<OkrRepository>, p: Params) -> Result<ToolResult> {
    let title = p.title.ok_or_else(|| anyhow::anyhow!("title required"))?;
    let desc = p.description.unwrap_or_default();
    let mut okr = Okr::new(&title, &desc);

    if let Some(owner) = p.owner {
        okr.owner = Some(owner);
    }
    if let Some(tid) = p.tenant_id {
        okr.tenant_id = Some(tid);
    }

    let krs = p
        .key_results
        .ok_or_else(|| anyhow::anyhow!("key_results required (at least one)"))?;
    for kr_p in krs {
        let kr = KeyResult::new(okr.id, &kr_p.title, kr_p.target_value, &kr_p.unit);
        okr.add_key_result(kr);
    }

    let created = repo.create_okr(okr).await?;
    Ok(ToolResult::success(serde_json::to_string_pretty(&created)?)
        .with_metadata("okr_id", json!(created.id.to_string())))
}
