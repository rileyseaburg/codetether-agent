//! Ralph Tool - Autonomous PRD-driven agent loop
//!
//! Exposes the Ralph loop as a tool for agents to invoke.

use anyhow::{Context, Result};
use async_trait::async_trait;
use serde::Deserialize;
use serde_json::{Value, json};
use std::path::PathBuf;
use std::process::Command;
use std::sync::Arc;

use super::{Tool, ToolResult};
use crate::provider::Provider;
use crate::ralph::{Prd, RalphConfig, RalphLoop, create_prd_template};
use crate::worktree::WorktreeManager;

/// Tool for running the Ralph autonomous agent loop
pub struct RalphTool {
    provider: Option<Arc<dyn Provider>>,
    model: String,
}

impl RalphTool {
    pub fn new() -> Self {
        Self {
            provider: None,
            model: String::new(),
        }
    }

    /// Create with a specific provider and model
    pub fn with_provider(provider: Arc<dyn Provider>, model: String) -> Self {
        Self {
            provider: Some(provider),
            model,
        }
    }

    /// Set the provider after construction
    #[allow(dead_code)]
    pub fn set_provider(&mut self, provider: Arc<dyn Provider>, model: String) {
        self.provider = Some(provider);
        self.model = model;
    }
}

#[derive(Deserialize)]
struct Params {
    action: String,
    #[serde(default)]
    prd_path: Option<String>,
    #[serde(default)]
    feature: Option<String>,
    #[serde(default)]
    project: Option<String>,
    #[serde(default)]
    max_iterations: Option<usize>,
}

#[async_trait]
impl Tool for RalphTool {
    fn id(&self) -> &str {
        "ralph"
    }
    fn name(&self) -> &str {
        "Ralph Agent"
    }

    fn description(&self) -> &str {
        r#"Run the Ralph autonomous agent loop to implement user stories from a PRD.

Ralph is an autonomous AI agent loop that runs repeatedly until all PRD items are complete.
Each iteration is a fresh instance with clean context. Memory persists via:
- Git history (commits from previous iterations)
- progress.txt (learnings and context)
- prd.json (which stories are done)

After completion, Ralph:
- Cleans up orphaned worktrees and branches
- Returns to your original branch
- Provides next steps (merge instructions or retry guidance)

The calling agent should handle the final merge based on the result metadata.

Actions:
- run: Start the Ralph loop with a PRD file
- status: Check progress of current Ralph run
- create-prd: Create a new PRD template

Returns metadata: {all_passed, ready_to_merge, feature_branch, passed, total}
"#
    }

    fn parameters(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "action": {
                    "type": "string",
                    "enum": ["run", "status", "create-prd"],
                    "description": "Action to perform"
                },
                "prd_path": {
                    "type": "string",
                    "description": "Path to prd.json file (default: prd.json)"
                },
                "feature": {
                    "type": "string",
                    "description": "Feature name for create-prd action"
                },
                "project": {
                    "type": "string",
                    "description": "Project name for create-prd action"
                },
                "max_iterations": {
                    "type": "integer",
                    "description": "Maximum iterations for run action (default: 10)"
                }
            },
            "required": ["action"]
        })
    }

    async fn execute(&self, params: Value) -> Result<ToolResult> {
        let p: Params = serde_json::from_value(params).context("Invalid params")?;
        let prd_path = PathBuf::from(p.prd_path.unwrap_or_else(|| "prd.json".to_string()));

        match p.action.as_str() {
            "run" => {
                let provider = self
                    .provider
                    .as_ref()
                    .ok_or_else(|| anyhow::anyhow!("No provider configured for Ralph"))?;

                // Remember the starting branch so we can return to it
                let cwd = std::env::current_dir().unwrap_or_default();
                let starting_branch = get_current_branch(&cwd);

                let config = RalphConfig {
                    prd_path: prd_path.to_string_lossy().to_string(),
                    max_iterations: p.max_iterations.unwrap_or(10),
                    progress_path: "progress.txt".to_string(),
                    quality_checks_enabled: true,
                    auto_commit: true,
                    model: Some(self.model.clone()),
                    use_rlm: false,
                    parallel_enabled: true,
                    max_concurrent_stories: 3,
                    worktree_enabled: true,
                    story_timeout_secs: 300,
                    conflict_timeout_secs: 120,
                    relay_enabled: false,
                    relay_max_agents: 8,
                    relay_max_rounds: 3,
                    max_steps_per_story: 30,
                };

                let mut ralph = RalphLoop::new(
                    prd_path.clone(),
                    Arc::clone(provider),
                    self.model.clone(),
                    config,
                )
                .await
                .context("Failed to initialize Ralph")?;

                let state = ralph.run().await.context("Ralph loop failed")?;

                let passed_count = state.prd.passed_count();
                let total_count = state.prd.user_stories.len();
                let feature_branch = state.prd.branch_name.clone();
                let all_passed = passed_count == total_count;

                // Clean up orphaned worktrees/branches
                let cleanup_count = if let Ok(mgr) = WorktreeManager::new(&cwd) {
                    mgr.cleanup_all().unwrap_or(0)
                } else {
                    0
                };

                // Return to starting branch if different
                let returned_to_original = if let Some(ref start) = starting_branch {
                    if !feature_branch.is_empty() && start != &feature_branch {
                        let _ = Command::new("git")
                            .args(["checkout", start])
                            .current_dir(&cwd)
                            .output();
                        true
                    } else {
                        false
                    }
                } else {
                    false
                };

                // Build the output with next steps guidance
                let next_steps = if all_passed {
                    format!(
                        "\n## Next Steps\n\n1. Review the changes on branch `{}`\n2. Create a pull request or merge to main:\n   ```bash\n   git checkout main && git merge {} --no-ff\n   ```\n3. Push the changes:\n   ```bash\n   git push\n   ```",
                        feature_branch, feature_branch
                    )
                } else {
                    let failed_stories: Vec<_> = state
                        .prd
                        .user_stories
                        .iter()
                        .filter(|s| !s.passes)
                        .map(|s| format!("- {}: {}", s.id, s.title))
                        .collect();
                    format!(
                        "\n## Incomplete Stories\n\n{}\n\n## Next Steps\n\n1. Review progress.txt for learnings\n2. Either:\n   - Re-run Ralph: `ralph({{action: 'run', prd_path: '{}'}})`\n   - Fix manually on branch `{}`\n   - Reset PRD to retry: edit {} and set `passes: false`",
                        failed_stories.join("\n"),
                        prd_path.display(),
                        feature_branch,
                        prd_path.display()
                    )
                };

                let cleanup_note = if cleanup_count > 0 {
                    format!(
                        "\n\n*(Cleaned up {} orphaned worktree(s)/branch(es))*",
                        cleanup_count
                    )
                } else {
                    String::new()
                };

                let branch_note = if returned_to_original {
                    format!(
                        "\n*(Returned to branch: {})*",
                        starting_branch.as_deref().unwrap_or("main")
                    )
                } else {
                    String::new()
                };

                let output = format!(
                    "# Ralph {:?}\n\n**Project:** {}\n**Feature:** {}\n**Progress:** {}/{} stories\n**Iterations:** {}/{}\n**Feature Branch:** {}\n\n## Stories\n{}{}{}\n{}",
                    state.status,
                    state.prd.project,
                    state.prd.feature,
                    passed_count,
                    total_count,
                    state.current_iteration,
                    state.max_iterations,
                    feature_branch,
                    state
                        .prd
                        .user_stories
                        .iter()
                        .map(|s| format!(
                            "- [{}] {}: {}",
                            if s.passes { "x" } else { " " },
                            s.id,
                            s.title
                        ))
                        .collect::<Vec<_>>()
                        .join("\n"),
                    cleanup_note,
                    branch_note,
                    next_steps
                );

                if all_passed {
                    Ok(ToolResult::success(output)
                        .with_metadata("status", json!(format!("{:?}", state.status)))
                        .with_metadata("passed", json!(passed_count))
                        .with_metadata("total", json!(total_count))
                        .with_metadata("feature_branch", json!(feature_branch))
                        .with_metadata("all_passed", json!(true))
                        .with_metadata("ready_to_merge", json!(true)))
                } else {
                    Ok(ToolResult::error(output)
                        .with_metadata("status", json!(format!("{:?}", state.status)))
                        .with_metadata("passed", json!(passed_count))
                        .with_metadata("total", json!(total_count))
                        .with_metadata("feature_branch", json!(feature_branch))
                        .with_metadata("all_passed", json!(false))
                        .with_metadata("ready_to_merge", json!(false)))
                }
            }

            "status" => match Prd::load(&prd_path).await {
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
            },

            "create-prd" => {
                let project = p.project.unwrap_or_else(|| "MyProject".to_string());
                let feature = p.feature.unwrap_or_else(|| "New Feature".to_string());

                let prd = create_prd_template(&project, &feature);

                prd.save(&prd_path).await.context("Failed to save PRD")?;

                let output = format!(
                    "# PRD Created\n\nSaved to: {}\n\n**Project:** {}\n**Feature:** {}\n**Branch:** {}\n\nEdit the file to add your user stories, then run:\n```\nralph({{action: 'run'}})\n```",
                    prd_path.display(),
                    prd.project,
                    prd.feature,
                    prd.branch_name
                );

                Ok(ToolResult::success(output))
            }

            _ => Ok(ToolResult::error(format!(
                "Unknown action: {}. Valid actions: run, status, create-prd",
                p.action
            ))),
        }
    }
}

/// Get the current git branch name
fn get_current_branch(dir: &std::path::Path) -> Option<String> {
    Command::new("git")
        .args(["rev-parse", "--abbrev-ref", "HEAD"])
        .current_dir(dir)
        .output()
        .ok()
        .and_then(|o| {
            if o.status.success() {
                Some(String::from_utf8_lossy(&o.stdout).trim().to_string())
            } else {
                None
            }
        })
}
