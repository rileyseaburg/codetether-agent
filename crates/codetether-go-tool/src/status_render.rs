//! Render persisted objective metrics and the most recent matching run.

use anyhow::Result;
use codetether_okr::{Okr, OkrRun};
use codetether_tool_core::ToolResult;
use serde_json::json;
use uuid::Uuid;
pub(crate) fn render(okr_id: Uuid, okr: Okr, latest_run: Option<&OkrRun>) -> Result<ToolResult> {
    let kr_status: Vec<String> = okr
        .key_results
        .iter()
        .map(|kr| {
            format!(
                "  - {}: {:.0}/{:.0} {} ({:.0}%)",
                kr.title,
                kr.current_value,
                kr.target_value,
                kr.unit,
                kr.progress() * 100.0
            )
        })
        .collect();

    let run_info = if let Some(run) = latest_run {
        format!(
            "\nLatest Run: {} ({:?})\n  Iterations: {}\n  Outcomes: {}",
            run.name,
            run.status,
            run.iterations,
            run.outcomes.len()
        )
    } else {
        "\nNo runs found.".to_string()
    };

    let output = format!(
        "# Go Status\n\n**OKR:** {}\n**Status:** {:?}\n**Progress:** {:.0}%\n\n## Key Results\n{}\n{}",
        okr.title,
        okr.status,
        okr.progress() * 100.0,
        kr_status.join("\n"),
        run_info
    );

    Ok(ToolResult::success(output).with_metadata("okr_id", json!(okr_id.to_string())))
}
