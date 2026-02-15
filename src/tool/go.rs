//! Go Tool - Autonomous task execution via OKR → PRD → Ralph pipeline
//!
//! Exposes the `/go` command as an MCP-callable tool for programmatic
//! autonomous work execution. Creates an OKR, generates a PRD from the
//! task description, runs the Ralph loop, and maps results back to the OKR.

use anyhow::{Context, Result};
use async_trait::async_trait;
use serde::Deserialize;
use serde_json::{Value, json};
use std::sync::Arc;
use uuid::Uuid;

use super::{Tool, ToolResult};
use crate::cli::go_ralph::{execute_go_ralph, format_go_ralph_result};
use crate::okr::{KeyResult, Okr, OkrRepository, OkrRun};

#[derive(Deserialize)]
struct GoParams {
    action: String,
    #[serde(default)]
    task: Option<String>,
    #[serde(default)]
    max_iterations: Option<usize>,
    #[serde(default)]
    max_concurrent_stories: Option<usize>,
    #[serde(default)]
    model: Option<String>,
    #[serde(default)]
    okr_id: Option<String>,
}

pub struct GoTool;

impl GoTool {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl Tool for GoTool {
    fn id(&self) -> &str {
        "go"
    }

    fn name(&self) -> &str {
        "Go"
    }

    fn description(&self) -> &str {
        r#"Autonomous task execution pipeline. Creates an OKR, generates a PRD from the task 
description using an LLM, runs the Ralph autonomous agent loop to implement all user stories, 
and maps results back to OKR outcomes.

This is the programmatic equivalent of the `/go` TUI command. OKR approval is automatic 
(no interactive gate) since this is called by MCP clients.

Actions:
- execute: Run the full autonomous pipeline (OKR → PRD → Ralph → results)
- status: Check status of an OKR run by okr_id

Required for execute: task
Optional for execute: max_iterations (default 10), max_concurrent_stories (default 3), model"#
    }

    fn parameters(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "action": {
                    "type": "string",
                    "enum": ["execute", "status"],
                    "description": "Action to perform"
                },
                "task": {
                    "type": "string",
                    "description": "Task description for autonomous execution"
                },
                "max_iterations": {
                    "type": "integer",
                    "description": "Maximum Ralph iterations (default: 10)"
                },
                "max_concurrent_stories": {
                    "type": "integer",
                    "description": "Maximum concurrent stories in Ralph (default: 3)"
                },
                "model": {
                    "type": "string",
                    "description": "Model to use for PRD generation and Ralph execution"
                },
                "okr_id": {
                    "type": "string",
                    "description": "OKR ID for status action"
                }
            },
            "required": ["action"]
        })
    }

    async fn execute(&self, params: Value) -> Result<ToolResult> {
        let p: GoParams = serde_json::from_value(params).context("Invalid params")?;

        match p.action.as_str() {
            "execute" => self.execute_go(p).await,
            "status" => self.check_status(p).await,
            _ => Ok(ToolResult::structured_error(
                "INVALID_ACTION",
                "go",
                &format!(
                    "Unknown action: '{}'. Valid actions: execute, status",
                    p.action
                ),
                None,
                Some(json!({
                    "action": "execute",
                    "task": "implement feature X with tests"
                })),
            )),
        }
    }
}

impl GoTool {
    async fn execute_go(&self, p: GoParams) -> Result<ToolResult> {
        let task = match p.task {
            Some(t) if !t.trim().is_empty() => t,
            _ => {
                return Ok(ToolResult::structured_error(
                    "MISSING_FIELD",
                    "go",
                    "task is required for execute action",
                    Some(vec!["task"]),
                    Some(json!({
                        "action": "execute",
                        "task": "implement user authentication with OAuth2"
                    })),
                ));
            }
        };

        let max_iterations = p.max_iterations.unwrap_or(10);
        let max_concurrent = p.max_concurrent_stories.unwrap_or(3);

        // Load provider registry from Vault
        let registry = Arc::new(
            crate::provider::ProviderRegistry::from_vault()
                .await
                .context("Failed to load providers from Vault")?,
        );

        // Resolve model and provider
        let model = p.model.unwrap_or_else(|| "zai/glm-5".to_string());
        let (provider, resolved_model) = resolve_provider(&registry, &model)?;

        // Create OKR with default template
        let okr_id = Uuid::new_v4();
        let mut okr = create_default_okr(okr_id, &task);
        let mut run = OkrRun::new(
            okr_id,
            format!("Go {}", chrono::Local::now().format("%Y-%m-%d %H:%M")),
        );
        let _ = run.submit_for_approval();
        run.record_decision(crate::okr::ApprovalDecision::approve(
            run.id,
            "Auto-approved via MCP go tool",
        ));

        // Persist OKR before execution
        if let Ok(repo) = OkrRepository::from_config().await {
            let _ = repo.create_okr(okr.clone()).await;
            let _ = repo.create_run(run.clone()).await;
        }

        tracing::info!(
            task = %task,
            okr_id = %okr_id,
            model = %resolved_model,
            max_iterations,
            "Starting /go autonomous pipeline via MCP"
        );

        // Create bus for inter-iteration learning sharing
        let bus = crate::bus::AgentBus::new().into_arc();

        // Run the full pipeline
        match execute_go_ralph(
            &task,
            &mut okr,
            &mut run,
            provider,
            &resolved_model,
            max_iterations,
            Some(bus),
            max_concurrent,
            Some(registry),
        )
        .await
        {
            Ok(result) => {
                // Persist final state
                if let Ok(repo) = OkrRepository::from_config().await {
                    let _ = repo.update_run(run).await;
                }

                let summary = format_go_ralph_result(&result, &task);

                if result.all_passed {
                    Ok(ToolResult::success(summary)
                        .with_metadata("okr_id", json!(okr_id.to_string()))
                        .with_metadata("status", json!(format!("{:?}", result.status)))
                        .with_metadata("passed", json!(result.passed))
                        .with_metadata("total", json!(result.total))
                        .with_metadata("feature_branch", json!(result.feature_branch))
                        .with_metadata("all_passed", json!(true))
                        .with_metadata("ready_to_merge", json!(true))
                        .with_metadata("prd_path", json!(result.prd_path.display().to_string())))
                } else {
                    Ok(ToolResult::error(summary)
                        .with_metadata("okr_id", json!(okr_id.to_string()))
                        .with_metadata("status", json!(format!("{:?}", result.status)))
                        .with_metadata("passed", json!(result.passed))
                        .with_metadata("total", json!(result.total))
                        .with_metadata("feature_branch", json!(result.feature_branch))
                        .with_metadata("all_passed", json!(false))
                        .with_metadata("ready_to_merge", json!(false))
                        .with_metadata("prd_path", json!(result.prd_path.display().to_string())))
                }
            }
            Err(err) => {
                // Mark run as failed
                run.status = crate::okr::OkrRunStatus::Failed;
                if let Ok(repo) = OkrRepository::from_config().await {
                    let _ = repo.update_run(run).await;
                }

                Ok(ToolResult::error(format!(
                    "Go pipeline failed: {err}\n\nOKR: {okr_id}\nTask: {task}"
                ))
                .with_metadata("okr_id", json!(okr_id.to_string()))
                .with_metadata("all_passed", json!(false)))
            }
        }
    }

    async fn check_status(&self, p: GoParams) -> Result<ToolResult> {
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
}

/// Create a default OKR with standard key results for a task
fn create_default_okr(okr_id: Uuid, task: &str) -> Okr {
    let title = if task.len() > 60 {
        format!("Go: {}…", &task[..57])
    } else {
        format!("Go: {task}")
    };

    let mut okr = Okr::new(title, format!("Autonomous execution: {task}"));
    okr.id = okr_id;

    okr.add_key_result(KeyResult::new(
        okr_id,
        "All stories complete",
        100.0,
        "%",
    ));
    okr.add_key_result(KeyResult::new(
        okr_id,
        "Quality gates pass",
        1.0,
        "count",
    ));
    okr.add_key_result(KeyResult::new(okr_id, "No critical errors", 0.0, "count"));

    okr
}

/// Resolve provider and model string from the registry.
/// Uses the same fallback strategy as the TUI autochat.
fn resolve_provider(
    registry: &crate::provider::ProviderRegistry,
    model: &str,
) -> Result<(Arc<dyn crate::provider::Provider>, String)> {
    let (provider_name, model_name) = crate::provider::parse_model_string(model);

    // Try explicit provider/model pair
    if let Some(provider_name) = provider_name {
        if let Some(provider) = registry.get(provider_name) {
            return Ok((provider, model_name.to_string()));
        }
    }

    // Fallback provider order (same as TUI)
    let fallbacks = [
        "zai",
        "openai",
        "github-copilot",
        "anthropic",
        "openrouter",
        "novita",
        "moonshotai",
        "google",
    ];

    for name in fallbacks {
        if let Some(provider) = registry.get(name) {
            return Ok((provider, model.to_string()));
        }
    }

    // Last resort: first available provider
    if let Some(name) = registry.list().into_iter().next() {
        if let Some(provider) = registry.get(name) {
            return Ok((provider, model.to_string()));
        }
    }

    anyhow::bail!("No provider available for model '{model}' and no fallback providers found")
}
