//! Read and display current story progress from the PRD.

use anyhow::Result;
use codetether_ralph_core::Prd;
use codetether_tool_core::ToolResult;
use std::path::Path;
pub(crate) async fn read(prd_path: &Path) -> Result<ToolResult> {
    match Prd::load(prd_path).await {
        Ok(prd) => {
            let passed_count = prd.passed_count();
            let output = format!(
                "# Ralph Status\n\n**Project:** {}\n**Feature:** {}\n**Progress:** {}/{} stories\n\n## Stories\n{}",
                prd.project,
                prd.feature,
                passed_count,
                prd.user_stories.len(),
                prd.user_stories
                    .iter()
                    .map(|s| format!(
                        "- [{}] {}: {}",
                        if s.passes { "x" } else { " " },
                        s.id,
                        s.title
                    ))
                    .collect::<Vec<_>>()
                    .join("\n")
            );
            Ok(ToolResult::success(output))
        }
        Err(_) => Ok(ToolResult::error(format!(
            "No PRD found at {}. Create one with: ralph({{action: 'create-prd', project: '...', feature: '...'}})",
            prd_path.display()
        ))),
    }
}
