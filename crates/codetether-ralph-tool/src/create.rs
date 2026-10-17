//! Create a PRD template with the established default project and feature names.

use crate::params::Params;
use anyhow::{Context, Result};
use codetether_ralph_core::create_prd_template;
use codetether_tool_core::ToolResult;
use std::path::Path;
pub(crate) async fn create(p: Params, prd_path: &Path) -> Result<ToolResult> {
    let project = p.project.unwrap_or_else(|| "MyProject".to_string());
    let feature = p.feature.unwrap_or_else(|| "New Feature".to_string());

    let prd = create_prd_template(&project, &feature);

    prd.save(prd_path).await.context("Failed to save PRD")?;

    let output = format!(
        "# PRD Created\n\nSaved to: {}\n\n**Project:** {}\n**Feature:** {}\n**Branch:** {}\n\nEdit the file to add your user stories, then run:\n```\nralph({{action: 'run'}})\n```",
        prd_path.display(),
        prd.project,
        prd.feature,
        prd.branch_name
    );

    Ok(ToolResult::success(output))
}
