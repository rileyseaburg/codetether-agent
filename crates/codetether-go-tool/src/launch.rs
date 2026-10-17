//! Validate and launch one autonomous task without waiting for completion.

use crate::{GoTool, Host, launch_info::LaunchInfo, params::GoParams};
use anyhow::{Context, Result};
use codetether_tool_core::ToolResult;
pub(crate) async fn execute<H: Host>(tool: &GoTool<H>, p: GoParams) -> Result<ToolResult> {
    let task = match crate::validation::task(p.task) {
        Ok(task) => task,
        Err(result) => return Ok(result),
    };
    let max_iterations = p.max_iterations.unwrap_or(10);
    let max_concurrent = p.max_concurrent_stories.unwrap_or(3);
    let registry = H::registry()
        .await
        .context("Failed to load providers from Vault")?;
    let model = p.model.unwrap_or_else(|| "zai/glm-5".to_string());
    let (provider, resolved_model) = crate::provider::resolve_provider(registry.as_ref(), &model)?;
    let (okr_id, okr, run, prd_filename, progress_filename) = crate::setup::create(&task).await;
    let okr_id_str = okr_id.to_string();
    let working_dir = crate::record::initial(
        &okr_id_str,
        &task,
        &resolved_model,
        &prd_filename,
        &progress_filename,
    );
    crate::launch_log::starting(&task, okr_id, &resolved_model, max_iterations);

    let (bg_okr_id, bg_task) = (okr_id_str.clone(), task.clone());
    let bg_model = resolved_model.clone();
    let bg_callback = tool.completion_callback.clone();
    tokio::spawn(crate::background::run::<H>(
        okr,
        run,
        provider,
        registry,
        bg_okr_id,
        bg_task,
        bg_model,
        max_iterations,
        max_concurrent,
        bg_callback,
    ));
    crate::launch_response::render(LaunchInfo {
        okr_id_str: &okr_id_str,
        task: &task,
        resolved_model: &resolved_model,
        max_iterations,
        working_dir: &working_dir,
        prd_filename: &prd_filename,
        progress_filename: &progress_filename,
    })
}
