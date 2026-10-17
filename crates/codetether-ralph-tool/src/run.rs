//! Execute the pipeline, clean worktrees, restore the branch and report results.

use crate::{Host, RalphTool, params::Params};
use anyhow::Result;
use codetether_tool_core::ToolResult;
use codetether_worktree::WorktreeManager;
use std::{path::PathBuf, sync::Arc};
pub(crate) async fn run<H: Host>(
    tool: &RalphTool<H>,
    p: Params,
    prd_path: PathBuf,
) -> Result<ToolResult> {
    let provider = tool
        .provider
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("No provider configured for Ralph"))?;

    // Remember the starting branch so we can return to it
    let cwd = std::env::current_dir().unwrap_or_default();
    let starting_branch = crate::branch::get_current_branch(&cwd);

    let config = crate::config::build(&prd_path, &tool.model, p.max_iterations);
    let state = H::run(
        prd_path.clone(),
        Arc::clone(provider),
        tool.model.clone(),
        config,
    )
    .await?;
    let (passed_count, total_count) = (state.prd.passed_count(), state.prd.user_stories.len());
    let feature_branch = state.prd.branch_name.clone();
    let all_passed = passed_count == total_count;

    // Clean up orphaned worktrees/branches
    let mgr = WorktreeManager::for_repo(&cwd);
    let cleanup_count = mgr.cleanup_all().await.unwrap_or(0);

    // Return to starting branch if different
    let returned_to_original = crate::restore::branch::<H>(&cwd, &starting_branch, &feature_branch);
    // Build the output with next steps guidance

    let output = crate::render::run(
        &state,
        &prd_path,
        &feature_branch,
        all_passed,
        passed_count,
        total_count,
        cleanup_count,
        returned_to_original,
        &starting_branch,
    );
    crate::metadata::finish(
        output,
        &state,
        &feature_branch,
        all_passed,
        passed_count,
        total_count,
    )
}
