//! Look up the requested objective and its persisted runs.

use crate::params::GoParams;
use anyhow::{Context, Result};
use codetether_okr::OkrRepository;
use codetether_tool_core::ToolResult;
use serde_json::json;
use uuid::Uuid;
pub(crate) async fn check(p: GoParams) -> Result<ToolResult> {
    let okr_id_str = match p.okr_id {
        Some(id) if !id.trim().is_empty() => id,
        _ => {
            return Ok(ToolResult::structured_error(
                "MISSING_FIELD",
                "go",
                "okr_id is required for status action",
                Some(vec!["okr_id"]),
                Some(json!({
                    "action": "status",
                    "okr_id": "uuid-of-okr"
                })),
            ));
        }
    };

    let okr_id: Uuid = okr_id_str
        .parse()
        .context("Invalid UUID format for okr_id")?;

    let repo = OkrRepository::from_config()
        .await
        .context("Failed to load OKR repository")?;

    let okr = match repo.get_okr(okr_id).await? {
        Some(okr) => okr,
        None => {
            return Ok(ToolResult::error(format!("OKR not found: {okr_id}")));
        }
    };

    let runs = repo.list_runs().await.unwrap_or_default();
    let runs: Vec<_> = runs.into_iter().filter(|r| r.okr_id == okr_id).collect();
    let latest_run = runs.last();

    crate::status_render::render(okr_id, okr, latest_run)
}
