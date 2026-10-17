//! Application execution and event adapters for forage work selection.
#[cfg(test)]
use codetether_forage_core::progress::success_progress_increment_ratio;
use codetether_forage_core::seed::{
    seed_initial_okr_if_empty, seed_moonshot_okr_if_no_opportunities,
};

#[cfg(test)]
use codetether_forage_core::seed::seed_default_okr_if_empty;
use codetether_forage_core::{ForageOpportunity, MoonshotRubric, truncate_chars_with_suffix};
pub use codetether_forage_core::{ForageRunSummary, ForageSelectionSummary};
#[cfg(test)]
use codetether_forage_core::{normalize_prompt_field, status_weight, urgency_bonus};

use crate::a2a::types::{Part, TaskState};
use crate::audit::{self, AuditCategory, AuditLog, AuditOutcome};
use crate::bus::{AgentBus, BusHandle, BusMessage};
use crate::cli::ForageArgs;
use crate::okr::{Okr, OkrRepository, OkrRun, OkrRunStatus};
use crate::provider::ProviderRegistry;
use crate::swarm::{DecompositionStrategy, ExecutionMode, SwarmConfig, SwarmExecutor};
use run_exec::execute_opportunity_with_run;
mod clarify;
pub mod console;
mod console_entry;
mod quality_shell;
mod run_exec;
mod s3_optional;
mod tetherscript_score;
use crate::forage_println;
use anyhow::{Context, Result};
use chrono::Utc;
use serde::Serialize;
use serde_json::json;
use std::collections::{BTreeSet, HashSet};
use std::process::Command;
use std::time::Duration;
use uuid::Uuid;

#[derive(Debug, Clone)]
struct ExecutionOutcome {
    detail: String,
    changed_files: Vec<String>,
    quality_gates_passed: bool,
}

pub async fn execute(args: ForageArgs) -> Result<()> {
    execute_with_summary(args).await.map(|_| ())
}

pub async fn execute_with_summary(args: ForageArgs) -> Result<ForageRunSummary> {
    ensure_audit_log_initialized().await;
    let repo = OkrRepository::from_config().await?;
    let moonshot_rubric = load_moonshot_rubric(&args).await?;
    if args.execute {
        seed_initial_okr_if_empty(&repo, &moonshot_rubric).await?;
    }
    let bus = AgentBus::new().into_arc();
    // Make forage observable to other in-process agents/tools: install this
    // bus as the process-global one when none has been set yet (first writer
    // wins, so this is a no-op if an entrypoint already installed a bus).
    crate::bus::set_global(bus.clone());
    // S3 archival is best-effort: forage always runs, even without MinIO/S3.
    // `--no-s3` skips the attempt entirely; otherwise we try and degrade to
    // local-only mode (with a warning) when no configuration is present.
    let mut s3_sync_handle = if args.no_s3 {
        tracing::info!("S3 archival disabled (--no-s3); running forage in local-only mode");
        None
    } else {
        s3_optional::try_start_bus_s3_sink(bus.clone()).await
    };
    // Only police S3 liveness if a sink actually started.
    let require_s3 = s3_sync_handle.is_some();
    let forage_id = "forage-runtime";
    let bus_handle = bus.handle(forage_id);
    let mut observer = bus.handle("forage-observer");
    let _ = bus_handle.announce_ready(vec![
        "okr-governance".to_string(),
        "business-prioritization".to_string(),
        "autonomous-forage".to_string(),
    ]);

    log_audit(
        AuditCategory::Cognition,
        "forage.start",
        AuditOutcome::Success,
        Some(json!({
            "top": args.top,
            "loop_mode": args.loop_mode,
            "interval_secs": args.interval_secs,
            "max_cycles": args.max_cycles,
            "execute": args.execute,
            "execution_engine": args.execution_engine,
            "run_timeout_secs": args.run_timeout_secs,
            "fail_fast": args.fail_fast,
            "swarm_strategy": args.swarm_strategy,
            "swarm_max_subagents": args.swarm_max_subagents,
            "swarm_max_steps": args.swarm_max_steps,
            "swarm_subagent_timeout_secs": args.swarm_subagent_timeout_secs,
            "model": args.model,
            "moonshot_goals": moonshot_rubric.goals.clone(),
            "moonshot_required": moonshot_rubric.required,
            "moonshot_min_alignment": moonshot_rubric.min_alignment,
        })),
        None,
        None,
    )
    .await;

    let top = args.top.clamp(1, 50);
    let interval_secs = args.interval_secs.clamp(5, 86_400);
    let mut cycle: usize = 0;
    let mut summary = ForageRunSummary {
        cycles_completed: 0,
        execute_requested: args.execute,
        execution_engine: args.execution_engine.clone(),
        total_selected: 0,
        total_executed: 0,
        total_execution_failures: 0,
        last_cycle_selected: Vec::new(),
    };

    loop {
        // Only check S3 health if S3 is required
        if require_s3 {
            ensure_s3_sync_alive(&mut s3_sync_handle).await?;
        }
        cycle = cycle.saturating_add(1);
        let cycle_task_id = format!("forage-cycle-{cycle}");
        let _ = bus_handle.send_task_update(
            &cycle_task_id,
            TaskState::Working,
            Some("scanning OKR opportunities".to_string()),
        );
        let mut opportunities = build_opportunities(&repo, &moonshot_rubric).await?;
        if opportunities.is_empty() {
            // First try the moonshot seed (execute mode only), then fall back to
            // interactive goal clarification so forage never silently dead-ends.
            if args.execute
                && let Some(seed_okr_id) =
                    seed_moonshot_okr_if_no_opportunities(&repo, &moonshot_rubric).await?
            {
                tracing::info!(
                    okr_id = %seed_okr_id,
                    moonshot_count = moonshot_rubric.goals.len(),
                    "Seeded moonshot-derived OKR because forage found no opportunities"
                );
                opportunities = build_opportunities(&repo, &moonshot_rubric).await?;
            }
            if opportunities.is_empty() {
                let okr_count = repo.list_okrs().await?.len();
                if clarify::clarify_and_persist(&repo, okr_count).await? {
                    opportunities = build_opportunities(&repo, &moonshot_rubric).await?;
                }
            }
        }
        let selected: Vec<ForageOpportunity> = opportunities.into_iter().take(top).collect();
        let _ = bus_handle.send(
            format!("forage.{cycle_task_id}.summary"),
            BusMessage::SharedResult {
                key: format!("forage/cycle/{cycle}/summary"),
                value: json!({
                    "cycle": cycle,
                    "selected": selected.len(),
                    "top": top,
                }),
                tags: vec![
                    "forage".to_string(),
                    "okr".to_string(),
                    "summary".to_string(),
                ],
            },
        );
        let _ = bus_handle.send_to_agent(
            "user",
            vec![Part::Text {
                text: format!(
                    "Forage cycle {cycle}: selected {} opportunities from OKR governance queue.",
                    selected.len()
                ),
            }],
        );

        if args.json {
            #[derive(Serialize)]
            struct CycleOutput {
                cycle: usize,
                selected: Vec<ForageOpportunity>,
            }
            let payload = CycleOutput {
                cycle,
                selected: selected.clone(),
            };
            println!("{}", serde_json::to_string_pretty(&payload)?);
        } else {
            console::print_cycle(cycle, &selected, !moonshot_rubric.goals.is_empty());
        }
        summary.cycles_completed = cycle;
        summary.total_selected = summary.total_selected.saturating_add(selected.len());
        summary.last_cycle_selected = ForageRunSummary::summarize_selection(&selected);
        log_audit(
            AuditCategory::Cognition,
            "forage.cycle",
            AuditOutcome::Success,
            Some(json!({
                "cycle": cycle,
                "selected": selected.len(),
                "top": top,
            })),
            None,
            None,
        )
        .await;
        flush_bus_observer(&mut observer, cycle, args.json);

        if args.execute && !selected.is_empty() {
            for item in &selected {
                tracing::info!(
                    okr_id = %item.okr_id,
                    key_result_id = %item.key_result_id,
                    engine = %args.execution_engine,
                    score = item.score,
                    "Executing forage opportunity"
                );

                let exec_task_id = format!("forage-okr-{}-kr-{}", item.okr_id, item.key_result_id);
                let _ = bus_handle.send_task_update(
                    &exec_task_id,
                    TaskState::Working,
                    Some(format!(
                        "executing ({}) opportunity '{}' for KR '{}'",
                        args.execution_engine, item.okr_title, item.key_result_title
                    )),
                );
                summary.total_executed = summary.total_executed.saturating_add(1);
                match execute_opportunity(item, &args).await {
                    Ok(execution_outcome) => {
                        if let Err(err) = record_execution_success_to_okr(
                            &repo,
                            item,
                            &args,
                            &execution_outcome,
                            cycle,
                        )
                        .await
                        {
                            tracing::warn!(
                                okr_id = %item.okr_id,
                                key_result_id = %item.key_result_id,
                                error = %err,
                                "Failed to persist forage execution progress to OKR"
                            );
                        }
                        let _ = bus_handle.send_task_update(
                            &exec_task_id,
                            TaskState::Completed,
                            Some(execution_outcome.detail.clone()),
                        );
                        log_audit(
                            AuditCategory::Cognition,
                            "forage.execute",
                            AuditOutcome::Success,
                            Some(json!({
                                "engine": args.execution_engine,
                                "score": item.score,
                                "okr_title": item.okr_title,
                                "key_result_title": item.key_result_title,
                                "detail": execution_outcome.detail,
                                "changed_files": execution_outcome.changed_files,
                            })),
                            Some(item.okr_id),
                            None,
                        )
                        .await;
                    }
                    Err(err) => {
                        summary.total_execution_failures =
                            summary.total_execution_failures.saturating_add(1);
                        let error_message = format!("{err:#}");
                        let _ = bus_handle.send_task_update(
                            &exec_task_id,
                            TaskState::Failed,
                            Some(format!("execution failed: {error_message}")),
                        );
                        log_audit(
                            AuditCategory::Cognition,
                            "forage.execute",
                            AuditOutcome::Failure,
                            Some(json!({
                                "engine": args.execution_engine,
                                "score": item.score,
                                "okr_title": item.okr_title,
                                "key_result_title": item.key_result_title,
                                "error": error_message,
                            })),
                            Some(item.okr_id),
                            None,
                        )
                        .await;
                        if args.fail_fast {
                            return Err(err);
                        }
                    }
                }
            }
        }

        let keep_running = args.loop_mode && (args.max_cycles == 0 || cycle < args.max_cycles);
        let _ = bus_handle.send_task_update(
            &cycle_task_id,
            TaskState::Completed,
            Some(format!("cycle complete (keep_running={keep_running})")),
        );

        if !keep_running {
            break;
        }

        tracing::info!(
            cycle,
            interval_secs,
            "Forage loop sleeping before next cycle"
        );
        let _ = bus_handle.send_task_update(
            &cycle_task_id,
            TaskState::Submitted,
            Some(format!("sleeping {interval_secs}s before next cycle")),
        );
        tokio::time::sleep(Duration::from_secs(interval_secs)).await;
    }
    let _ = bus_handle.announce_shutdown();
    log_audit(
        AuditCategory::Cognition,
        "forage.stop",
        AuditOutcome::Success,
        Some(json!({ "cycles": cycle })),
        None,
        None,
    )
    .await;

    Ok(summary)
}

async fn ensure_s3_sync_alive(
    handle: &mut Option<tokio::task::JoinHandle<Result<()>>>,
) -> Result<()> {
    let Some(inner) = handle.as_ref() else {
        anyhow::bail!("S3 sync task missing");
    };
    if !inner.is_finished() {
        return Ok(());
    }

    let finished = handle.take().expect("checked is_some");
    match finished.await {
        Ok(Ok(())) => {
            anyhow::bail!("S3 sync task exited unexpectedly");
        }
        Ok(Err(err)) => Err(anyhow::anyhow!("S3 sync task failed: {err:#}")),
        Err(join_err) => Err(anyhow::anyhow!("S3 sync task join failure: {join_err}")),
    }
}
async fn record_execution_success_to_okr(
    repo: &OkrRepository,
    item: &ForageOpportunity,
    args: &ForageArgs,
    execution_outcome: &ExecutionOutcome,
    cycle: usize,
) -> Result<()> {
    let evidence = codetether_forage_core::progress::ExecutionEvidence {
        engine: &args.execution_engine,
        model: args.model.as_deref(),
        detail: &execution_outcome.detail,
        changed_files: &execution_outcome.changed_files,
        quality_gates_passed: execution_outcome.quality_gates_passed,
    };
    codetether_forage_core::progress::record(repo, item, &evidence, cycle).await
}

fn snapshot_git_changed_files() -> Option<BTreeSet<String>> {
    let cwd = std::env::current_dir().ok()?;
    let is_git_repo = Command::new("git")
        .args(["rev-parse", "--is-inside-work-tree"])
        .current_dir(&cwd)
        .output()
        .ok()?
        .status
        .success();
    if !is_git_repo {
        return None;
    }

    let mut changed = BTreeSet::new();
    changed.extend(git_name_list(&cwd, &["diff", "--name-only"]));
    changed.extend(git_name_list(&cwd, &["diff", "--name-only", "--cached"]));
    changed.extend(git_name_list(
        &cwd,
        &["ls-files", "--others", "--exclude-standard"],
    ));
    Some(changed)
}

fn git_name_list(cwd: &std::path::Path, args: &[&str]) -> Vec<String> {
    let Ok(output) = Command::new("git").args(args).current_dir(cwd).output() else {
        return Vec::new();
    };
    if !output.status.success() {
        return Vec::new();
    }

    String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(ToString::to_string)
        .collect()
}

async fn execute_opportunity(
    item: &ForageOpportunity,
    args: &ForageArgs,
) -> Result<ExecutionOutcome> {
    let before = snapshot_git_changed_files();
    let detail = match args.execution_engine.as_str() {
        "swarm" => execute_opportunity_with_swarm(item, args).await?,
        "go" => execute_opportunity_with_go(item, args).await?,
        _ => execute_opportunity_with_run(item, args).await?,
    };
    let after = snapshot_git_changed_files();
    let changed_files = match (before, after) {
        (Some(before_set), Some(after_set)) => after_set.difference(&before_set).cloned().collect(),
        (_, Some(after_set)) => after_set.into_iter().collect(),
        _ => Vec::new(),
    };

    // Run quality gates to verify the changes work
    let quality_result = run_quality_gates(&changed_files).await;

    let (final_detail, quality_passed) = match quality_result {
        Ok((qr, passed)) => (format!("{}\n\nQuality gates: {}", detail, qr), passed),
        Err(e) => {
            tracing::warn!(error = %e, "Quality gates failed to run");
            (detail, false) // Treat execution errors as quality failure
        }
    };

    Ok(ExecutionOutcome {
        detail: final_detail,
        changed_files,
        quality_gates_passed: quality_passed,
    })
}

/// Run quality gates (cargo check/test) to verify changes work
async fn run_quality_gates(changed_files: &[String]) -> Result<(String, bool)> {
    // Only run quality gates if there are Rust files changed
    let has_rust_files = changed_files.iter().any(|f| f.ends_with(".rs"));

    if !has_rust_files {
        return Ok((
            "no Rust files changed, skipping quality gates".to_string(),
            true,
        ));
    }

    let mut results = Vec::new();
    let mut all_passed = true;

    match quality_shell::run("cargo check --message-format=short").await {
        Ok(output) => {
            let status = if output.success {
                "PASS"
            } else {
                all_passed = false;
                "FAIL"
            };
            let summary = if output.combined.chars().count() > 200 {
                truncate_chars_with_suffix(&output.combined, 200, " [...truncated]")
            } else if output.combined.is_empty() {
                "no errors".to_string()
            } else {
                output.combined
            };
            results.push(format!("cargo check: {} - {}", status, summary));
        }
        Err(e) => {
            results.push(format!("cargo check: ERROR - {}", e));
            all_passed = false;
        }
    }

    match quality_shell::run("cargo test --quiet -- --nocapture").await {
        Ok(output) => {
            let status = if output.success {
                "PASS"
            } else {
                all_passed = false;
                "FAIL"
            };
            let summary = if output.combined.chars().count() > 300 {
                truncate_chars_with_suffix(&output.combined, 300, " [...truncated]")
            } else if output.combined.is_empty() {
                "no test output".to_string()
            } else {
                output.combined
            };
            results.push(format!("cargo test: {} - {}", status, summary));
        }
        Err(e) => {
            results.push(format!("cargo test: ERROR - {}", e));
            all_passed = false;
        }
    }

    Ok((results.join("\n"), all_passed))
}

async fn execute_opportunity_with_swarm(
    item: &ForageOpportunity,
    args: &ForageArgs,
) -> Result<String> {
    let timeout_secs = args.run_timeout_secs.clamp(30, 86_400);
    let swarm_config = build_swarm_config(args);
    let executor = SwarmExecutor::new(swarm_config);
    let strategy = parse_swarm_strategy(&args.swarm_strategy);

    match tokio::time::timeout(
        Duration::from_secs(timeout_secs),
        executor.execute(&item.prompt, strategy),
    )
    .await
    {
        Ok(Ok(result)) => {
            if result.success {
                Ok(format!(
                    "swarm execution completed (subagents_spawned={}, completed={}, failed={}, retries={})",
                    result.stats.subagents_spawned,
                    result.stats.subagents_completed,
                    result.stats.subagents_failed,
                    result
                        .subtask_results
                        .iter()
                        .map(|r| r.retry_count)
                        .sum::<u32>() as usize
                ))
            } else {
                let error = result.error.unwrap_or_else(|| {
                    format!(
                        "swarm reported failure (failed_subtasks={}, total_subtasks={})",
                        result.stats.subagents_failed, result.stats.subagents_spawned
                    )
                });
                anyhow::bail!(error);
            }
        }
        Ok(Err(err)) => Err(err),
        Err(_) => anyhow::bail!("swarm execution timed out after {timeout_secs}s"),
    }
}

fn build_swarm_config(args: &ForageArgs) -> SwarmConfig {
    SwarmConfig {
        max_subagents: args.swarm_max_subagents.max(1),
        max_steps_per_subagent: args.swarm_max_steps.max(1),
        subagent_timeout_secs: args.swarm_subagent_timeout_secs.clamp(30, 86_400),
        model: args.model.clone(),
        execution_mode: ExecutionMode::LocalThread,
        ..Default::default()
    }
}

/// Execute opportunity using the Ralph PRD-driven autonomous loop (go engine)
async fn execute_opportunity_with_go(
    item: &ForageOpportunity,
    args: &ForageArgs,
) -> Result<String> {
    use crate::cli::go_ralph::execute_go_ralph;

    let timeout_secs = args.run_timeout_secs.clamp(30, 86_400);
    let model = args
        .model
        .clone()
        .unwrap_or_else(|| "anthropic/claude-sonnet-4-20250514".to_string());

    // Load provider registry and get a provider
    let registry = ProviderRegistry::from_vault()
        .await
        .context("Failed to load provider registry for go engine")?;

    let (provider, resolved_model) = registry
        .resolve_model(&model)
        .with_context(|| format!("Failed to resolve model '{}' for go engine", model))?;

    // Load the OKR to get full context for PRD generation
    let repo = OkrRepository::from_config()
        .await
        .context("Failed to load OKR repository")?;

    let mut okr = repo
        .get_okr(item.okr_id)
        .await?
        .with_context(|| format!("OKR {} not found", item.okr_id))?;

    // Create an OKR run for this execution
    let mut okr_run = OkrRun::new(item.okr_id, format!("forage-cycle-{}", item.key_result_id));
    okr_run.submit_for_approval()?;
    okr_run.record_decision(crate::okr::ApprovalDecision::approve(
        okr_run.id,
        "Auto-approved from forage execution",
    ));

    // Build the task from the opportunity
    let task = item.prompt.clone();

    // Execute the Ralph PRD-driven autonomous loop
    match tokio::time::timeout(
        Duration::from_secs(timeout_secs),
        execute_go_ralph(
            &task,
            &mut okr,
            &mut okr_run,
            provider,
            &resolved_model,
            10,   // max_iterations
            None, // bus - could pass bus here for inter-iteration learning
            3,    // max_concurrent_stories
            None, // registry - passed via RalphLoop.with_registry if needed
        ),
    )
    .await
    {
        Ok(Ok(result)) => {
            if result.all_passed {
                Ok(format!(
                    "go execution completed - all {}/{} stories passed (iterations: {}/{}, branch: {})",
                    result.passed,
                    result.total,
                    result.iterations,
                    result.max_iterations,
                    result.feature_branch
                ))
            } else {
                Ok(format!(
                    "go execution completed - {}/{} stories passed (iterations: {}/{}, branch: {}, status: {:?})",
                    result.passed,
                    result.total,
                    result.iterations,
                    result.max_iterations,
                    result.feature_branch,
                    result.status
                ))
            }
        }
        Ok(Err(err)) => {
            // Update run status to failed
            okr_run.status = OkrRunStatus::Failed;
            Err(err).context("go execution failed")
        }
        Err(_) => anyhow::bail!("go execution timed out after {timeout_secs}s"),
    }
}

fn parse_swarm_strategy(value: &str) -> DecompositionStrategy {
    match value {
        "domain" => DecompositionStrategy::ByDomain,
        "data" => DecompositionStrategy::ByData,
        "stage" => DecompositionStrategy::ByStage,
        "none" => DecompositionStrategy::None,
        _ => DecompositionStrategy::Automatic,
    }
}

async fn ensure_audit_log_initialized() {
    if audit::try_audit_log().is_some() {
        return;
    }

    let default_sink = crate::config::Config::data_dir()
        .map(|base| base.join("audit"))
        .map(|audit_dir| {
            let _ = std::fs::create_dir_all(&audit_dir);
            audit_dir.join("forage_audit.jsonl")
        });
    let log = if std::env::var("CODETETHER_AUDIT_LOG_PATH").is_ok() {
        AuditLog::from_env()
    } else {
        AuditLog::new(10_000, default_sink)
    };
    let _ = audit::init_audit_log(log);
}

async fn log_audit(
    category: AuditCategory,
    action: &str,
    outcome: AuditOutcome,
    detail: Option<serde_json::Value>,
    okr_id: Option<Uuid>,
    session_id: Option<String>,
) {
    if let Some(audit_log) = audit::try_audit_log() {
        audit_log
            .log_with_correlation(
                category,
                action,
                outcome,
                Some("forage-runtime".to_string()),
                detail,
                okr_id.map(|id| id.to_string()),
                None,
                None,
                session_id,
            )
            .await;
    }
}

fn flush_bus_observer(observer: &mut BusHandle, cycle: usize, json_mode: bool) {
    if json_mode {
        return;
    }

    let mut shown = 0usize;
    while let Some(env) = observer.try_recv() {
        if shown >= 12 {
            break;
        }
        let label = match &env.message {
            BusMessage::AgentReady { .. } => "agent_ready",
            BusMessage::AgentShutdown { .. } => "agent_shutdown",
            BusMessage::TaskUpdate { .. } => "task_update",
            BusMessage::SharedResult { .. } => "shared_result",
            BusMessage::AgentMessage { .. } => "agent_message",
            _ => "other",
        };
        forage_println!(
            "   [bus cycle {cycle}] {} :: topic={} sender={}",
            label,
            env.topic,
            env.sender_id
        );
        shown = shown.saturating_add(1);
    }
}

async fn load_moonshot_rubric(args: &ForageArgs) -> Result<MoonshotRubric> {
    let mut goals = args
        .moonshots
        .iter()
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(ToString::to_string)
        .collect::<Vec<_>>();

    if let Some(path) = &args.moonshot_file {
        let content = tokio::fs::read_to_string(path)
            .await
            .with_context(|| format!("Failed to read moonshot file: {}", path.display()))?;

        if let Ok(json_list) = serde_json::from_str::<Vec<String>>(&content) {
            goals.extend(
                json_list
                    .into_iter()
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty()),
            );
        } else {
            goals.extend(
                content
                    .lines()
                    .map(str::trim)
                    .filter(|line| !line.is_empty() && !line.starts_with('#'))
                    .map(ToString::to_string),
            );
        }
    }

    let mut deduped = Vec::new();
    let mut seen = HashSet::new();
    for goal in goals {
        let key = goal.to_ascii_lowercase();
        if seen.insert(key) {
            deduped.push(goal);
        }
    }

    Ok(MoonshotRubric {
        goals: deduped,
        required: args.moonshot_required,
        min_alignment: args.moonshot_min_alignment.clamp(0.0, 1.0),
    })
}

async fn build_opportunities(
    repo: &OkrRepository,
    moonshots: &MoonshotRubric,
) -> Result<Vec<ForageOpportunity>> {
    let okrs = repo.list_okrs().await?;
    Ok(collect_opportunities_with_rubric(&okrs, moonshots))
}

#[cfg(test)]
fn collect_opportunities(okrs: &[Okr]) -> Vec<ForageOpportunity> {
    collect_opportunities_with_rubric(okrs, &MoonshotRubric::default())
}

#[cfg(test)]
mod tests {
    use super::{
        ExecutionOutcome, ForageOpportunity, MoonshotRubric, build_swarm_config,
        collect_opportunities, collect_opportunities_with_rubric, normalize_prompt_field,
        record_execution_success_to_okr, seed_default_okr_if_empty,
        seed_moonshot_okr_if_no_opportunities, status_weight, success_progress_increment_ratio,
        urgency_bonus,
    };
    use crate::cli::ForageArgs;
    use crate::okr::{KeyResult, Okr, OkrStatus};
    use chrono::{Duration, Utc};
    use tempfile::tempdir;
    use uuid::Uuid;

    #[test]
    fn status_weight_prioritizes_active_okrs() {
        assert!(status_weight(OkrStatus::Active) > status_weight(OkrStatus::Draft));
        assert!(status_weight(OkrStatus::Draft) > status_weight(OkrStatus::OnHold));
        assert_eq!(status_weight(OkrStatus::Completed), 0.0);
    }

    #[test]
    fn urgency_bonus_increases_for_due_dates() {
        let now = Utc::now();
        let overdue = urgency_bonus(Some(now - Duration::days(1)), now);
        let soon = urgency_bonus(Some(now + Duration::days(3)), now);
        let later = urgency_bonus(Some(now + Duration::days(45)), now);
        assert!(overdue > soon);
        assert!(soon > later);
    }

    #[test]
    fn collect_opportunities_skips_complete_or_cancelled_work() {
        let mut okr = Okr::new("Ship growth loop", "Increase retained users");
        okr.status = OkrStatus::Cancelled;
        let mut kr = KeyResult::new(okr.id, "Retained users", 100.0, "%");
        kr.update_progress(10.0);
        okr.add_key_result(kr);

        let items = collect_opportunities(&[okr]);
        assert!(items.is_empty());
    }

    #[test]
    fn collect_opportunities_ranks_remaining_work() {
        let mut okr = Okr::new("Ship growth loop", "Increase retained users");
        okr.status = OkrStatus::Active;

        let mut kr_low = KeyResult::new(okr.id, "KR Low Remaining", 100.0, "%");
        kr_low.update_progress(80.0);
        let mut kr_high = KeyResult::new(okr.id, "KR High Remaining", 100.0, "%");
        kr_high.update_progress(10.0);
        okr.add_key_result(kr_low);
        okr.add_key_result(kr_high);

        let items = collect_opportunities(&[okr]);
        assert_eq!(items.len(), 2);
        assert_eq!(items[0].key_result_title, "KR High Remaining");
    }

    #[test]
    fn moonshot_rubric_filters_low_alignment_work() {
        let mut okr = Okr::new(
            "Improve parser latency",
            "Reduce p95 latency for parser pipeline",
        );
        okr.status = OkrStatus::Active;
        let mut kr = KeyResult::new(okr.id, "Parser p95 under 50ms", 100.0, "%");
        kr.update_progress(10.0);
        okr.add_key_result(kr);

        let rubric = MoonshotRubric {
            goals: vec!["eliminate billing fraud globally".to_string()],
            required: true,
            min_alignment: 0.4,
        };

        let items = collect_opportunities_with_rubric(&[okr], &rubric);
        assert!(
            items.is_empty(),
            "non-aligned work should be filtered out when moonshot is required"
        );
    }

    #[test]
    fn forage_timeout_bounds_are_clamped() {
        let low = 5u64.clamp(30, 86_400);
        let high = 999_999u64.clamp(30, 86_400);
        assert_eq!(low, 30);
        assert_eq!(high, 86_400);
    }

    #[test]
    fn swarm_config_does_not_force_legacy_model_fallback() {
        let args = ForageArgs {
            top: 3,
            loop_mode: false,
            interval_secs: 120,
            max_cycles: 1,
            execute: true,
            moonshots: Vec::new(),
            moonshot_file: None,
            moonshot_required: false,
            moonshot_min_alignment: 0.10,
            execution_engine: "swarm".to_string(),
            run_timeout_secs: 900,
            fail_fast: false,
            swarm_strategy: "auto".to_string(),
            swarm_max_subagents: 8,
            swarm_max_steps: 100,
            swarm_subagent_timeout_secs: 300,
            model: None,
            json: false,
            no_s3: false,
        };

        let config = build_swarm_config(&args);
        assert!(config.model.is_none());
    }

    #[test]
    fn swarm_config_preserves_explicit_model_override() {
        let args = ForageArgs {
            top: 3,
            loop_mode: false,
            interval_secs: 120,
            max_cycles: 1,
            execute: true,
            moonshots: Vec::new(),
            moonshot_file: None,
            moonshot_required: false,
            moonshot_min_alignment: 0.10,
            execution_engine: "swarm".to_string(),
            run_timeout_secs: 900,
            fail_fast: false,
            swarm_strategy: "auto".to_string(),
            swarm_max_subagents: 8,
            swarm_max_steps: 100,
            swarm_subagent_timeout_secs: 300,
            model: Some("openai-codex/gpt-5-mini".to_string()),
            json: false,
            no_s3: false,
        };

        let config = build_swarm_config(&args);
        assert_eq!(config.model.as_deref(), Some("openai-codex/gpt-5-mini"));
    }

    #[test]
    fn normalize_prompt_field_compacts_and_truncates() {
        let input = "alpha   beta\n\n gamma    delta";
        assert_eq!(normalize_prompt_field(input, 128), "alpha beta gamma delta");

        let long = "x".repeat(400);
        let normalized = normalize_prompt_field(&long, 64);
        assert!(normalized.ends_with("...(truncated)"));
        assert!(normalized.len() > 64);
    }

    #[test]
    fn success_progress_increment_ratio_is_bounded() {
        let mut kr = KeyResult::new(Uuid::new_v4(), "KR", 100.0, "%");
        kr.update_progress(0.0);
        let high_remaining = success_progress_increment_ratio(&kr);
        assert!((high_remaining - 0.15).abs() < f64::EPSILON);

        kr.update_progress(95.0);
        let low_remaining = success_progress_increment_ratio(&kr);
        assert!((low_remaining - 0.05).abs() < f64::EPSILON);
    }

    #[tokio::test]
    async fn record_execution_success_updates_kr_progress_and_evidence() {
        let dir = tempdir().expect("create tempdir");
        let repo = crate::okr::OkrRepository::new(dir.path().to_path_buf());

        let mut okr = Okr::new("Autonomous Business-Aligned Execution", "Test objective");
        okr.status = OkrStatus::Active;
        let mut kr = KeyResult::new(okr.id, "KR1", 100.0, "%");
        kr.update_progress(0.0);
        let kr_id = kr.id;
        okr.add_key_result(kr);
        let okr_id = okr.id;
        let _ = repo.create_okr(okr).await.expect("create okr");

        let item = ForageOpportunity {
            score: 1.35,
            okr_id,
            okr_title: "Autonomous Business-Aligned Execution".to_string(),
            okr_status: OkrStatus::Active,
            key_result_id: kr_id,
            key_result_title: "KR1".to_string(),
            progress: 0.0,
            remaining: 1.0,
            target_date: None,
            moonshot_alignment: 0.0,
            moonshot_hits: Vec::new(),
            prompt: "test prompt".to_string(),
        };
        let args = ForageArgs {
            top: 3,
            loop_mode: false,
            interval_secs: 120,
            max_cycles: 1,
            execute: true,
            moonshots: Vec::new(),
            moonshot_file: None,
            moonshot_required: false,
            moonshot_min_alignment: 0.10,
            execution_engine: "run".to_string(),
            run_timeout_secs: 900,
            fail_fast: false,
            swarm_strategy: "auto".to_string(),
            swarm_max_subagents: 8,
            swarm_max_steps: 100,
            swarm_subagent_timeout_secs: 300,
            model: Some("openai-codex/gpt-5.1-codex".to_string()),
            json: false,
            no_s3: false,
        };

        let execution_outcome = ExecutionOutcome {
            detail: "run execution completed".to_string(),
            changed_files: vec!["src/forage/mod.rs".to_string()],
            quality_gates_passed: true,
        };
        record_execution_success_to_okr(&repo, &item, &args, &execution_outcome, 1)
            .await
            .expect("record success");

        let saved = repo
            .get_okr(okr_id)
            .await
            .expect("read okr")
            .expect("okr exists");
        let saved_kr = saved
            .key_results
            .into_iter()
            .find(|k| k.id == kr_id)
            .expect("kr exists");
        assert!(saved_kr.current_value > 0.0);
        assert_eq!(saved_kr.outcomes.len(), 1);
        assert!(
            saved_kr.outcomes[0]
                .evidence
                .iter()
                .any(|entry| entry.starts_with("engine:run"))
        );
    }

    #[tokio::test]
    async fn swarm_success_without_file_evidence_does_not_increment_progress() {
        let dir = tempdir().expect("create tempdir");
        let repo = crate::okr::OkrRepository::new(dir.path().to_path_buf());

        let mut okr = Okr::new("Autonomous Business-Aligned Execution", "Test objective");
        okr.status = OkrStatus::Active;
        let mut kr = KeyResult::new(okr.id, "KR1", 100.0, "%");
        kr.update_progress(10.0);
        let kr_id = kr.id;
        okr.add_key_result(kr);
        let okr_id = okr.id;
        let _ = repo.create_okr(okr).await.expect("create okr");

        let item = ForageOpportunity {
            score: 1.35,
            okr_id,
            okr_title: "Autonomous Business-Aligned Execution".to_string(),
            okr_status: OkrStatus::Active,
            key_result_id: kr_id,
            key_result_title: "KR1".to_string(),
            progress: 0.10,
            remaining: 0.90,
            target_date: None,
            moonshot_alignment: 0.0,
            moonshot_hits: Vec::new(),
            prompt: "test prompt".to_string(),
        };
        let args = ForageArgs {
            top: 3,
            loop_mode: false,
            interval_secs: 120,
            max_cycles: 1,
            execute: true,
            moonshots: Vec::new(),
            moonshot_file: None,
            moonshot_required: false,
            moonshot_min_alignment: 0.10,
            execution_engine: "swarm".to_string(),
            run_timeout_secs: 900,
            fail_fast: false,
            swarm_strategy: "auto".to_string(),
            swarm_max_subagents: 8,
            swarm_max_steps: 100,
            swarm_subagent_timeout_secs: 300,
            model: Some("openai-codex/gpt-5.1-codex".to_string()),
            json: false,
            no_s3: false,
        };
        let execution_outcome = ExecutionOutcome {
            detail: "swarm execution completed".to_string(),
            changed_files: Vec::new(),
            quality_gates_passed: true,
        };
        record_execution_success_to_okr(&repo, &item, &args, &execution_outcome, 2)
            .await
            .expect("record success");

        let saved = repo
            .get_okr(okr_id)
            .await
            .expect("read okr")
            .expect("okr exists");
        let saved_kr = saved
            .key_results
            .into_iter()
            .find(|k| k.id == kr_id)
            .expect("kr exists");
        assert_eq!(saved_kr.current_value, 10.0);
        assert_eq!(saved_kr.outcomes.len(), 1);
        assert!(
            saved_kr.outcomes[0]
                .evidence
                .iter()
                .any(|entry| entry == "concrete_file_evidence:false")
        );
    }

    #[tokio::test]
    async fn seed_default_okr_populates_empty_repo() {
        let dir = tempdir().expect("create tempdir");
        let repo = crate::okr::OkrRepository::new(dir.path().to_path_buf());

        seed_default_okr_if_empty(&repo)
            .await
            .expect("seed should succeed");

        let okrs = repo.list_okrs().await.expect("list okrs");
        assert_eq!(okrs.len(), 1);
        assert_eq!(
            okrs[0].title,
            "Mission: Autonomous Business-Aligned Execution"
        );
        assert_eq!(okrs[0].status, OkrStatus::Active);
        assert_eq!(okrs[0].key_results.len(), 3);
    }

    #[tokio::test]
    async fn seed_default_okr_is_noop_when_repo_not_empty() {
        let dir = tempdir().expect("create tempdir");
        let repo = crate::okr::OkrRepository::new(dir.path().to_path_buf());

        let mut existing = Okr::new("Existing Objective", "Do not overwrite");
        existing.status = OkrStatus::Active;
        existing.add_key_result(KeyResult::new(existing.id, "KR1", 100.0, "%"));
        let _ = repo.create_okr(existing).await.expect("create existing");

        seed_default_okr_if_empty(&repo)
            .await
            .expect("seed should succeed");

        let okrs = repo.list_okrs().await.expect("list okrs");
        assert_eq!(okrs.len(), 1);
        assert_eq!(okrs[0].title, "Existing Objective");
    }

    #[tokio::test]
    async fn moonshot_seed_creates_okr_when_no_opportunities_exist() {
        let dir = tempdir().expect("create tempdir");
        let repo = crate::okr::OkrRepository::new(dir.path().to_path_buf());

        // Completed objective should not produce forage opportunities.
        let mut completed = Okr::new("Completed Objective", "Already done");
        completed.status = OkrStatus::Completed;
        let mut kr = KeyResult::new(completed.id, "KR done", 100.0, "%");
        kr.update_progress(100.0);
        completed.add_key_result(kr);
        let _ = repo.create_okr(completed).await.expect("create completed");

        let rubric = MoonshotRubric {
            goals: vec![
                "Automate customer acquisition end-to-end".to_string(),
                "Funnel conversion replaces manual sales".to_string(),
            ],
            required: true,
            min_alignment: 0.2,
        };

        let seeded_id = seed_moonshot_okr_if_no_opportunities(&repo, &rubric)
            .await
            .expect("seed should succeed");
        assert!(seeded_id.is_some(), "expected moonshot-derived seed OKR");
        let seeded_id = seeded_id.expect("seeded id");

        let okrs = repo.list_okrs().await.expect("list okrs");
        let seeded = okrs
            .iter()
            .find(|o| o.id == seeded_id)
            .expect("seeded okr exists");
        assert_eq!(seeded.status, OkrStatus::Active);
        assert_eq!(
            seeded.title,
            "Mission: Moonshot-Derived Autonomous Execution"
        );
        assert!(!seeded.key_results.is_empty());
    }

    #[tokio::test]
    async fn moonshot_seed_is_noop_when_open_moonshot_seed_exists() {
        let dir = tempdir().expect("create tempdir");
        let repo = crate::okr::OkrRepository::new(dir.path().to_path_buf());

        let rubric = MoonshotRubric {
            goals: vec!["Tech stack is the moat".to_string()],
            required: true,
            min_alignment: 0.2,
        };
        let first = seed_moonshot_okr_if_no_opportunities(&repo, &rubric)
            .await
            .expect("first seed should succeed");
        assert!(first.is_some());

        let second = seed_moonshot_okr_if_no_opportunities(&repo, &rubric)
            .await
            .expect("second seed should succeed");
        assert!(
            second.is_none(),
            "should not duplicate active moonshot seed"
        );
    }
}

fn collect_opportunities_with_rubric(
    okrs: &[Okr],
    moonshots: &MoonshotRubric,
) -> Vec<ForageOpportunity> {
    codetether_forage_core::collect_opportunities_with_rubric(
        okrs,
        moonshots,
        &mut tetherscript_score::apply,
    )
}
