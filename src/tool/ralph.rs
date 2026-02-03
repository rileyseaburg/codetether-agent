//! Ralph Tool - Autonomous PRD-driven agent loop
//!
//! Exposes the Ralph loop as a tool for agents to invoke.

use anyhow::{Context, Result};
use async_trait::async_trait;
use serde::Deserialize;
use serde_json::{json, Value};
use std::path::PathBuf;
use std::sync::Arc;

use super::{Tool, ToolResult};
use crate::ralph::{RalphLoop, RalphConfig, create_prd_template, Prd};
use crate::provider::Provider;

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
    fn id(&self) -> &str { "ralph" }
    fn name(&self) -> &str { "Ralph Agent" }
    
    fn description(&self) -> &str {
        r#"Run the Ralph autonomous agent loop to implement user stories from a PRD.

Ralph is an autonomous AI agent loop that runs repeatedly until all PRD items are complete.
Each iteration is a fresh instance with clean context. Memory persists via:
- Git history (commits from previous iterations)
- progress.txt (learnings and context)
- prd.json (which stories are done)

Actions:
- run: Start the Ralph loop with a PRD file
- status: Check progress of current Ralph run
- create-prd: Create a new PRD template
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
                let provider = self.provider.as_ref()
                    .ok_or_else(|| anyhow::anyhow!("No provider configured for Ralph"))?;

                let config = RalphConfig {
                    prd_path: prd_path.to_string_lossy().to_string(),
                    max_iterations: p.max_iterations.unwrap_or(10),
                    progress_path: "progress.txt".to_string(),
                    quality_checks_enabled: true,
                    auto_commit: true,
                    model: Some(self.model.clone()),
                    use_rlm: false,
                };

                let mut ralph = RalphLoop::new(
                    prd_path,
                    Arc::clone(provider),
                    self.model.clone(),
                    config,
                ).await.context("Failed to initialize Ralph")?;

                let state = ralph.run().await.context("Ralph loop failed")?;

                let passed_count = state.prd.passed_count();
                let total_count = state.prd.user_stories.len();

                let output = format!(
                    "# Ralph {:?}\n\n**Project:** {}\n**Feature:** {}\n**Progress:** {}/{} stories\n**Iterations:** {}/{}\n\n## Stories\n{}",
                    state.status,
                    state.prd.project,
                    state.prd.feature,
                    passed_count,
                    total_count,
                    state.current_iteration,
                    state.max_iterations,
                    state.prd.user_stories.iter()
                        .map(|s| format!("- [{}] {}: {}", if s.passes { "x" } else { " " }, s.id, s.title))
                        .collect::<Vec<_>>()
                        .join("\n")
                );

                let success = passed_count == total_count;
                if success {
                    Ok(ToolResult::success(output)
                        .with_metadata("status", json!(format!("{:?}", state.status)))
                        .with_metadata("passed", json!(passed_count))
                        .with_metadata("total", json!(total_count)))
                } else {
                    Ok(ToolResult::error(output)
                        .with_metadata("status", json!(format!("{:?}", state.status)))
                        .with_metadata("passed", json!(passed_count))
                        .with_metadata("total", json!(total_count)))
                }
            }

            "status" => {
                match Prd::load(&prd_path).await {
                    Ok(prd) => {
                        let passed_count = prd.passed_count();
                        let output = format!(
                            "# Ralph Status\n\n**Project:** {}\n**Feature:** {}\n**Progress:** {}/{} stories\n\n## Stories\n{}",
                            prd.project,
                            prd.feature,
                            passed_count,
                            prd.user_stories.len(),
                            prd.user_stories.iter()
                                .map(|s| format!("- [{}] {}: {}", if s.passes { "x" } else { " " }, s.id, s.title))
                                .collect::<Vec<_>>()
                                .join("\n")
                        );
                        Ok(ToolResult::success(output))
                    }
                    Err(_) => {
                        Ok(ToolResult::error(format!(
                            "No PRD found at {}. Create one with: ralph({{action: 'create-prd', project: '...', feature: '...'}})",
                            prd_path.display()
                        )))
                    }
                }
            }

            "create-prd" => {
                let project = p.project.unwrap_or_else(|| "MyProject".to_string());
                let feature = p.feature.unwrap_or_else(|| "New Feature".to_string());

                let prd = create_prd_template(&project, &feature);
                
                prd.save(&prd_path).await
                    .context("Failed to save PRD")?;

                let output = format!(
                    "# PRD Created\n\nSaved to: {}\n\n**Project:** {}\n**Feature:** {}\n**Branch:** {}\n\nEdit the file to add your user stories, then run:\n```\nralph({{action: 'run'}})\n```",
                    prd_path.display(),
                    prd.project,
                    prd.feature,
                    prd.branch_name
                );

                Ok(ToolResult::success(output))
            }

            _ => {
                Ok(ToolResult::error(format!(
                    "Unknown action: {}. Valid actions: run, status, create-prd",
                    p.action
                )))
            }
        }
    }
}
