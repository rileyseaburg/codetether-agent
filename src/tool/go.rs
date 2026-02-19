//! Go Tool - Autonomous task execution via OKR → PRD → Ralph pipeline
//!
//! Exposes the `/go` command as an MCP-callable tool for programmatic
//! autonomous work execution. Creates an OKR, generates a PRD from the
//! task description, runs the Ralph loop, and maps results back to the OKR.
//!
//! The `execute` action spawns the pipeline in a background task and returns
//! immediately so the MCP client can monitor progress via `go_watch`.

use anyhow::{Context, Result};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

use super::{Tool, ToolResult};
use crate::cli::go_ralph::{execute_go_ralph, format_go_ralph_result};
use crate::okr::{KeyResult, Okr, OkrRepository, OkrRun};

// ─── Active execution tracking ──────────────────────────────────────────

/// Phase of a running go pipeline
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "phase", rename_all = "snake_case")]
pub enum GoRunPhase {
    Starting,
    Running,
    Completed {
        passed: usize,
        total: usize,
        all_passed: bool,
        feature_branch: String,
        summary: String,
    },
    Failed {
        error: String,
    },
}

/// Metadata for an active go run
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActiveGoRun {
    pub okr_id: String,
    pub task: String,
    pub model: String,
    pub started_at: String,
    pub working_dir: String,
    pub prd_filename: String,
    pub progress_filename: String,
    pub phase: GoRunPhase,
}

/// Global registry of active (and recently completed) go runs.
static ACTIVE_GO_RUNS: std::sync::LazyLock<Mutex<HashMap<String, ActiveGoRun>>> =
    std::sync::LazyLock::new(|| Mutex::new(HashMap::new()));

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

pub struct GoTool {
    /// Optional callback invoked when the background pipeline completes or fails.
    /// Used by the A2A worker to stream the final OKR result back to the calling LLM.
    completion_callback: Option<Arc<dyn Fn(String) + Send + Sync + 'static>>,
}

impl GoTool {
    pub fn new() -> Self {
        Self {
            completion_callback: None,
        }
    }

    /// Construct with a completion callback that fires when the background pipeline finishes.
    pub fn with_callback(cb: Arc<dyn Fn(String) + Send + Sync + 'static>) -> Self {
        Self {
            completion_callback: Some(cb),
        }
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

The pipeline runs in the background. Use action "watch" to monitor progress.

Actions:
- execute: Launch the autonomous pipeline (OKR → PRD → Ralph → results). Returns immediately.
- watch: Watch a running pipeline's progress by okr_id. Shows PRD status, progress notes, and phase.
- status: Check final status of an OKR run by okr_id

Required for execute: task
Optional for execute: max_iterations (default 10), max_concurrent_stories (default 3), model
Required for watch/status: okr_id"#
    }

    fn parameters(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "action": {
                    "type": "string",
                    "enum": ["execute", "watch", "status"],
                    "description": "Action to perform. Use 'execute' to start, 'watch' to monitor progress, 'status' for OKR results."
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
                    "description": "OKR ID for watch/status actions"
                }
            },
            "required": ["action"]
        })
    }

    async fn execute(&self, params: Value) -> Result<ToolResult> {
        let p: GoParams = serde_json::from_value(params).context("Invalid params")?;

        match p.action.as_str() {
            "execute" => self.execute_go(p).await,
            "watch" => self.watch_go(p).await,
            "status" => self.check_status(p).await,
            _ => Ok(ToolResult::structured_error(
                "INVALID_ACTION",
                "go",
                &format!(
                    "Unknown action: '{}'. Valid actions: execute, watch, status",
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
        let okr_id_str = okr_id.to_string();
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

        let run_id = run.id;
        let prd_filename = format!("prd_{}.json", run_id.to_string().replace('-', "_"));
        let progress_filename = format!("progress_{}.txt", run_id.to_string().replace('-', "_"));

        // Persist OKR before execution
        if let Ok(repo) = OkrRepository::from_config().await {
            let _ = repo.create_okr(okr.clone()).await;
            let _ = repo.create_run(run.clone()).await;
        }

        let working_dir = std::env::current_dir()
            .map(|p| p.display().to_string())
            .unwrap_or_else(|_| ".".into());

        // Register active run
        let active_run = ActiveGoRun {
            okr_id: okr_id_str.clone(),
            task: task.clone(),
            model: resolved_model.clone(),
            started_at: chrono::Utc::now().to_rfc3339(),
            working_dir: working_dir.clone(),
            prd_filename: prd_filename.clone(),
            progress_filename: progress_filename.clone(),
            phase: GoRunPhase::Starting,
        };
        if let Ok(mut runs) = ACTIVE_GO_RUNS.lock() {
            runs.insert(okr_id_str.clone(), active_run);
        }

        tracing::info!(
            task = %task,
            okr_id = %okr_id,
            model = %resolved_model,
            max_iterations,
            "Starting /go autonomous pipeline via MCP (background)"
        );

        // Spawn the pipeline in a background task
        let bg_okr_id = okr_id_str.clone();
        let bg_task = task.clone();
        let bg_model = resolved_model.clone();
        let bg_callback = self.completion_callback.clone();
        tokio::spawn(async move {
            // Update phase to Running
            if let Ok(mut runs) = ACTIVE_GO_RUNS.lock() {
                if let Some(r) = runs.get_mut(&bg_okr_id) {
                    r.phase = GoRunPhase::Running;
                }
            }

            // Create bus with S3 sink for training data archival
            let bus = crate::bus::AgentBus::new().into_arc();
            crate::bus::s3_sink::spawn_bus_s3_sink(bus.clone());

            match execute_go_ralph(
                &bg_task,
                &mut okr,
                &mut run,
                provider,
                &bg_model,
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

                    let summary = format_go_ralph_result(&result, &bg_task);

                    if let Ok(mut runs) = ACTIVE_GO_RUNS.lock() {
                        if let Some(r) = runs.get_mut(&bg_okr_id) {
                            r.phase = GoRunPhase::Completed {
                                passed: result.passed,
                                total: result.total,
                                all_passed: result.all_passed,
                                feature_branch: result.feature_branch,
                                summary,
                            };
                        }
                    }

                    tracing::info!(
                        okr_id = %bg_okr_id,
                        passed = result.passed,
                        total = result.total,
                        "Go pipeline completed"
                    );

                    // Notify the LLM session that the pipeline finished
                    if let Some(ref cb) = bg_callback {
                        let phase_str = {
                            let runs = ACTIVE_GO_RUNS.lock().unwrap_or_else(|e| e.into_inner());
                            if let Some(r) = runs.get(&bg_okr_id) {
                                if let GoRunPhase::Completed { passed, total, all_passed, ref feature_branch, ref summary } = r.phase {
                                    format!(
                                        "# Go Pipeline Completed {}\n\n\
                                         **OKR ID:** `{}`\n\
                                         **Result:** {}/{} stories passed\n\
                                         **Branch:** `{}`\n\n{}",
                                        if all_passed { "✅" } else { "⚠️" },
                                        bg_okr_id, passed, total, feature_branch, summary
                                    )
                                } else {
                                    format!("Go pipeline completed for OKR `{}`", bg_okr_id)
                                }
                            } else {
                                format!("Go pipeline completed for OKR `{}`", bg_okr_id)
                            }
                        };
                        cb(phase_str);
                    }
                }
                Err(err) => {
                    // Mark run as failed
                    run.status = crate::okr::OkrRunStatus::Failed;
                    if let Ok(repo) = OkrRepository::from_config().await {
                        let _ = repo.update_run(run).await;
                    }

                    let error_msg = err.to_string();
                    if let Ok(mut runs) = ACTIVE_GO_RUNS.lock() {
                        if let Some(r) = runs.get_mut(&bg_okr_id) {
                            r.phase = GoRunPhase::Failed {
                                error: error_msg.clone(),
                            };
                        }
                    }

                    tracing::error!(
                        okr_id = %bg_okr_id,
                        error = %error_msg,
                        "Go pipeline failed"
                    );

                    // Notify the LLM session that the pipeline failed
                    if let Some(ref cb) = bg_callback {
                        cb(format!(
                            "# Go Pipeline Failed ❌\n\n**OKR ID:** `{}`\n**Error:** {}",
                            bg_okr_id, error_msg
                        ));
                    }
                }
            }
        });

        // Return immediately with launch confirmation + watch instructions
        let output = format!(
            "# Go Pipeline Launched\n\n\
             **OKR ID:** `{okr_id_str}`\n\
             **Task:** {task}\n\
             **Model:** {resolved_model}\n\
             **Max Iterations:** {max_iterations}\n\
             **Working Directory:** {working_dir}\n\n\
             The autonomous pipeline is now running in the background.\n\n\
             ## Monitor Progress\n\n\
             Use the `go` tool with action `watch` to monitor this pipeline:\n\n\
             ```json\n\
             {{\"action\": \"watch\", \"okr_id\": \"{okr_id_str}\"}}\n\
             ```\n\n\
             The pipeline will:\n\
             1. Generate a PRD from the task description\n\
             2. Run the Ralph loop to implement all user stories\n\
             3. Run quality checks (typecheck, lint, test, build)\n\
             4. Map results back to OKR outcomes\n\n\
             PRD file: `{prd_filename}`\n\
             Progress file: `{progress_filename}`"
        );

        Ok(ToolResult::success(output)
            .with_metadata("okr_id", json!(okr_id_str))
            .with_metadata("phase", json!("starting"))
            .with_metadata("prd_filename", json!(prd_filename))
            .with_metadata("progress_filename", json!(progress_filename))
            .with_metadata(
                "watch_hint",
                json!({
                    "tool": "go",
                    "args": {"action": "watch", "okr_id": okr_id_str}
                }),
            ))
    }

    async fn watch_go(&self, p: GoParams) -> Result<ToolResult> {
        let okr_id_str = match p.okr_id {
            Some(id) if !id.trim().is_empty() => id,
            _ => {
                // If no OKR ID given, list all active runs
                let runs = ACTIVE_GO_RUNS.lock().unwrap_or_else(|e| e.into_inner());
                if runs.is_empty() {
                    return Ok(ToolResult::success(
                        "No active go pipelines. Use `go(action: \"execute\", task: \"...\")` to start one.",
                    ));
                }

                let mut output = String::from("# Active Go Pipelines\n\n");
                for (id, run) in runs.iter() {
                    let phase_str = match &run.phase {
                        GoRunPhase::Starting => "Starting".to_string(),
                        GoRunPhase::Running => "Running".to_string(),
                        GoRunPhase::Completed { passed, total, .. } => {
                            format!("Completed ({passed}/{total} passed)")
                        }
                        GoRunPhase::Failed { error } => {
                            format!("Failed: {}", &error[..error.len().min(80)])
                        }
                    };
                    output.push_str(&format!(
                        "- **{id}**: {phase_str}\n  Task: {}\n  Started: {}\n\n",
                        run.task, run.started_at
                    ));
                }
                output.push_str("Use `go(action: \"watch\", okr_id: \"<id>\")` for details.");
                return Ok(ToolResult::success(output));
            }
        };

        // Look up the active run
        let active_run = {
            let runs = ACTIVE_GO_RUNS.lock().unwrap_or_else(|e| e.into_inner());
            runs.get(&okr_id_str).cloned()
        };

        let Some(run) = active_run else {
            return Ok(ToolResult::error(format!(
                "No active go pipeline found for OKR `{okr_id_str}`.\n\n\
                 Use `go(action: \"watch\")` with no okr_id to list active pipelines,\n\
                 or `go(action: \"status\", okr_id: \"...\")` to check completed runs."
            )));
        };

        let mut output = format!(
            "# Go Pipeline Status\n\n\
             **OKR ID:** `{}`\n\
             **Task:** {}\n\
             **Model:** {}\n\
             **Started:** {}\n\
             **Working Directory:** {}\n",
            run.okr_id, run.task, run.model, run.started_at, run.working_dir
        );

        // Phase
        match &run.phase {
            GoRunPhase::Starting => {
                output.push_str("\n**Phase:** Starting (generating PRD...)\n");
            }
            GoRunPhase::Running => {
                output.push_str("\n**Phase:** Running Ralph loop\n");
            }
            GoRunPhase::Completed {
                passed,
                total,
                all_passed,
                feature_branch,
                summary,
            } => {
                output.push_str(&format!(
                    "\n**Phase:** Completed {}\n\
                     **Result:** {passed}/{total} stories passed\n\
                     **Feature Branch:** `{feature_branch}`\n\n\
                     ## Summary\n\n{summary}\n",
                    if *all_passed { "✅" } else { "⚠️" }
                ));
            }
            GoRunPhase::Failed { error } => {
                output.push_str(&format!("\n**Phase:** Failed ❌\n**Error:** {error}\n"));
            }
        }

        // Read PRD file for story-level progress
        if let Ok(prd_content) = std::fs::read_to_string(&run.prd_filename) {
            if let Ok(prd) = serde_json::from_str::<Value>(&prd_content) {
                if let Some(stories) = prd.get("user_stories").and_then(|s| s.as_array()) {
                    output.push_str("\n## Stories\n\n");
                    let mut passed_count = 0;
                    for story in stories {
                        let id = story.get("id").and_then(|v| v.as_str()).unwrap_or("?");
                        let title = story.get("title").and_then(|v| v.as_str()).unwrap_or("?");
                        let passes = story
                            .get("passes")
                            .and_then(|v| v.as_bool())
                            .unwrap_or(false);
                        let icon = if passes {
                            passed_count += 1;
                            "✅"
                        } else {
                            "⏳"
                        };
                        output.push_str(&format!("- {icon} **{id}**: {title}\n"));
                    }
                    output.push_str(&format!(
                        "\n**Progress:** {passed_count}/{} stories passed\n",
                        stories.len()
                    ));
                }
            }
        }

        // Read progress file
        if let Ok(progress) = std::fs::read_to_string(&run.progress_filename) {
            if !progress.trim().is_empty() {
                // Show last 30 lines to avoid overwhelming output
                let lines: Vec<&str> = progress.lines().collect();
                let start = lines.len().saturating_sub(30);
                let tail: String = lines[start..].join("\n");
                output.push_str(&format!(
                    "\n## Progress Notes (last {} lines)\n\n```\n{tail}\n```\n",
                    lines.len().min(30)
                ));
            }
        }

        // Hint for next action
        if matches!(run.phase, GoRunPhase::Starting | GoRunPhase::Running) {
            output.push_str(&format!(
                "\n---\n*Pipeline still running. Call `go(action: \"watch\", okr_id: \"{}\")` again to check progress.*\n",
                run.okr_id
            ));
        }

        Ok(ToolResult::success(output)
            .with_metadata("okr_id", json!(run.okr_id))
            .with_metadata(
                "phase",
                json!(serde_json::to_value(&run.phase).unwrap_or(json!("unknown"))),
            ))
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

    okr.add_key_result(KeyResult::new(okr_id, "All stories complete", 100.0, "%"));
    okr.add_key_result(KeyResult::new(okr_id, "Quality gates pass", 1.0, "count"));
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
