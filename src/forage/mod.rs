use crate::a2a::types::{Part, TaskState};
use crate::audit::{self, AuditCategory, AuditLog, AuditOutcome};
use crate::bus::s3_sink::{BusS3Sink, BusS3SinkConfig};
use crate::bus::{AgentBus, BusHandle, BusMessage};
use crate::cli::{ForageArgs, RunArgs};
use crate::okr::{
    KeyResult, KrOutcome, KrOutcomeType, Okr, OkrRepository, OkrRun, OkrRunStatus, OkrStatus,
};
use crate::provider::ProviderRegistry;
use crate::swarm::{DecompositionStrategy, ExecutionMode, SwarmConfig, SwarmExecutor};
use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use serde::Serialize;
use serde_json::json;
use std::cmp::Ordering;
use std::collections::{BTreeSet, HashSet};
use std::process::Command;
use std::time::Duration;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize)]
struct ForageOpportunity {
    score: f64,
    okr_id: Uuid,
    okr_title: String,
    okr_status: OkrStatus,
    key_result_id: Uuid,
    key_result_title: String,
    progress: f64,
    remaining: f64,
    target_date: Option<DateTime<Utc>>,
    moonshot_alignment: f64,
    moonshot_hits: Vec<String>,
    prompt: String,
}

#[derive(Debug, Clone, Default)]
struct MoonshotRubric {
    goals: Vec<String>,
    required: bool,
    min_alignment: f64,
}

#[derive(Debug, Clone)]
struct ExecutionOutcome {
    detail: String,
    changed_files: Vec<String>,
    quality_gates_passed: bool,
}

pub async fn execute(args: ForageArgs) -> Result<()> {
    ensure_audit_log_initialized().await;
    let repo = OkrRepository::from_config().await?;
    let moonshot_rubric = load_moonshot_rubric(&args).await?;
    if args.execute {
        seed_initial_okr_if_empty(&repo, &moonshot_rubric).await?;
    }
    let bus = AgentBus::new().into_arc();
    // S3 archival is optional when --no-s3 is specified
    let require_s3 = !args.no_s3;
    let mut s3_sync_handle = if require_s3 {
        Some(start_required_bus_s3_sink(bus.clone()).await?)
    } else {
        tracing::info!("S3 archival disabled (--no-s3); running forage in local-only mode");
        None
    };
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
        if args.execute && opportunities.is_empty() {
            if let Some(seed_okr_id) =
                seed_moonshot_okr_if_no_opportunities(&repo, &moonshot_rubric).await?
            {
                tracing::info!(
                    okr_id = %seed_okr_id,
                    moonshot_count = moonshot_rubric.goals.len(),
                    "Seeded moonshot-derived OKR because forage found no opportunities"
                );
                opportunities = build_opportunities(&repo, &moonshot_rubric).await?;
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
            println!("\n=== Forage Cycle {} ===", cycle);
            if selected.is_empty() {
                println!(
                    "No OKR opportunities found (active/draft/on_hold with remaining KR work)."
                );
            } else {
                for (idx, item) in selected.iter().enumerate() {
                    println!(
                        "\n{}. [{}] {}",
                        idx + 1,
                        item.okr_status_label(),
                        item.okr_title
                    );
                    println!(
                        "   KR: {} ({:.1}% remaining, score {:.3})",
                        item.key_result_title,
                        item.remaining * 100.0,
                        item.score
                    );
                    println!(
                        "   Progress: {:.1}% complete",
                        item.progress.clamp(0.0, 1.0) * 100.0
                    );
                    if !moonshot_rubric.goals.is_empty() {
                        let hits = if item.moonshot_hits.is_empty() {
                            "none".to_string()
                        } else {
                            item.moonshot_hits.join(" | ")
                        };
                        println!(
                            "   Moonshot alignment: {:.1}% (hits: {})",
                            item.moonshot_alignment * 100.0,
                            hits
                        );
                    }
                }
            }
        }
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

    Ok(())
}

async fn seed_default_okr_if_empty(repo: &OkrRepository) -> Result<()> {
    let existing = repo.list_okrs().await?;
    if !existing.is_empty() {
        return Ok(());
    }

    let mut okr = Okr::new(
        "Mission: Autonomous Business-Aligned Execution",
        "Autonomously execute concrete, behavior-preserving code changes that align to business goals and produce measurable progress.",
    );
    okr.status = OkrStatus::Active;
    let okr_id = okr.id;
    okr.add_key_result(KeyResult::new(okr_id, "Key Result 1", 100.0, "%"));
    okr.add_key_result(KeyResult::new(
        okr_id,
        "Team produces actionable handoff",
        100.0,
        "%",
    ));
    okr.add_key_result(KeyResult::new(okr_id, "No critical errors", 100.0, "%"));

    let _ = repo.create_okr(okr).await?;
    tracing::info!(okr_id = %okr_id, "Seeded default OKR for forage execution");
    Ok(())
}

async fn seed_initial_okr_if_empty(repo: &OkrRepository, moonshots: &MoonshotRubric) -> Result<()> {
    if moonshots.goals.is_empty() {
        return seed_default_okr_if_empty(repo).await;
    }
    let _ = seed_moonshot_okr_if_empty(repo, moonshots).await?;
    Ok(())
}

async fn seed_moonshot_okr_if_empty(
    repo: &OkrRepository,
    moonshots: &MoonshotRubric,
) -> Result<Option<Uuid>> {
    let existing = repo.list_okrs().await?;
    if !existing.is_empty() || moonshots.goals.is_empty() {
        return Ok(None);
    }
    seed_moonshot_okr(repo, moonshots).await.map(Some)
}

async fn seed_moonshot_okr_if_no_opportunities(
    repo: &OkrRepository,
    moonshots: &MoonshotRubric,
) -> Result<Option<Uuid>> {
    if moonshots.goals.is_empty() {
        return Ok(None);
    }

    // If there is already an open moonshot-derived objective with incomplete work,
    // avoid continuously creating new ones.
    let existing = repo.list_okrs().await?;
    let has_open_moonshot_seed = existing.iter().any(|okr| {
        matches!(
            okr.status,
            OkrStatus::Active | OkrStatus::Draft | OkrStatus::OnHold
        ) && okr.title == "Mission: Moonshot-Derived Autonomous Execution"
            && okr
                .key_results
                .iter()
                .any(|kr| kr.progress().clamp(0.0, 1.0) < 1.0)
    });
    if has_open_moonshot_seed {
        return Ok(None);
    }

    seed_moonshot_okr(repo, moonshots).await.map(Some)
}

async fn seed_moonshot_okr(repo: &OkrRepository, moonshots: &MoonshotRubric) -> Result<Uuid> {
    let mut okr = Okr::new(
        "Mission: Moonshot-Derived Autonomous Execution",
        format!(
            "Autogenerated forage objective derived from moonshots.\nMoonshots:\n- {}",
            moonshots.goals.join("\n- ")
        ),
    );
    okr.status = OkrStatus::Active;
    let okr_id = okr.id;

    let max_goals = 8usize;
    for (idx, goal) in moonshots.goals.iter().take(max_goals).enumerate() {
        let mut kr = KeyResult::new(
            okr_id,
            format!("Moonshot {}: {}", idx + 1, truncate_goal(goal, 96)),
            100.0,
            "%",
        );
        kr.description = goal.clone();
        okr.add_key_result(kr);
    }

    if okr.key_results.is_empty() {
        okr.add_key_result(KeyResult::new(
            okr_id,
            "Moonshot alignment delivered",
            100.0,
            "%",
        ));
    }

    let _ = repo.create_okr(okr).await?;
    tracing::info!(
        okr_id = %okr_id,
        moonshot_count = moonshots.goals.len(),
        "Seeded moonshot-derived OKR for forage execution"
    );
    Ok(okr_id)
}

fn truncate_goal(goal: &str, max_chars: usize) -> String {
    let trimmed = goal.trim();
    if trimmed.chars().count() <= max_chars {
        return trimmed.to_string();
    }
    let mut out = trimmed.chars().take(max_chars).collect::<String>();
    out.push_str("...");
    out
}

async fn start_required_bus_s3_sink(
    bus: std::sync::Arc<AgentBus>,
) -> Result<tokio::task::JoinHandle<Result<()>>> {
    let config = BusS3SinkConfig::from_env_or_vault().await.context(
        "Forage requires S3 bus archival. Configure MINIO_*/CODETETHER_CHAT_SYNC_MINIO_* or Vault provider 'chat-sync-minio'.",
    )?;
    let sink = BusS3Sink::from_config(bus, config)
        .await
        .context("Failed to initialize required S3 bus sink for forage")?;
    Ok(tokio::spawn(async move { sink.run().await }))
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
    let Some(mut okr) = repo.get_okr(item.okr_id).await? else {
        anyhow::bail!("OKR {} not found", item.okr_id);
    };

    let Some(kr) = okr
        .key_results
        .iter_mut()
        .find(|kr| kr.id == item.key_result_id)
    else {
        anyhow::bail!("KR {} not found in OKR {}", item.key_result_id, item.okr_id);
    };

    let enforce_concrete_file_evidence =
        args.execution_engine == "swarm" || args.execution_engine == "go";
    let has_file_evidence = !execution_outcome.changed_files.is_empty();
    let quality_gates_passed = execution_outcome.quality_gates_passed;

    // Only increment progress if:
    // 1. File evidence exists (or not required for "run" engine)
    // 2. Quality gates passed (cargo check/test succeeded)
    let should_increment =
        (has_file_evidence || !enforce_concrete_file_evidence) && quality_gates_passed;

    let before_progress = kr.progress();
    if should_increment {
        let increment_ratio = success_progress_increment_ratio(kr);
        if kr.target_value > 0.0 && increment_ratio > 0.0 {
            let increment_value = (kr.target_value * increment_ratio).max(0.0);
            let new_value = (kr.current_value + increment_value).min(kr.target_value);
            kr.update_progress(new_value);
        }
    }
    let after_progress = kr.progress();

    let progress_impact = if quality_gates_passed {
        if should_increment {
            "advanced this KR"
        } else {
            "no concrete changed-file evidence was found; KR progress not incremented"
        }
    } else {
        "quality gates FAILED; KR progress not incremented"
    };

    let mut kr_outcome = KrOutcome::new(
        kr.id,
        format!(
            "Forage cycle {cycle} execution via '{}' {}.",
            args.execution_engine, progress_impact
        ),
    );
    kr_outcome.outcome_type = KrOutcomeType::CodeChange;
    kr_outcome.value = Some((after_progress * 100.0).clamp(0.0, 100.0));
    kr_outcome.source = "forage-runtime".to_string();
    kr_outcome.evidence = vec![
        format!("cycle:{cycle}"),
        format!("engine:{}", args.execution_engine),
        format!("model:{}", args.model.as_deref().unwrap_or("default")),
        format!("score:{:.3}", item.score),
        format!("okr_id:{}", item.okr_id),
        format!("kr_id:{}", item.key_result_id),
        format!("progress_before_pct:{:.2}", before_progress * 100.0),
        format!("progress_after_pct:{:.2}", after_progress * 100.0),
        format!(
            "detail:{}",
            normalize_prompt_field(&execution_outcome.detail, 320)
        ),
        format!("concrete_file_evidence:{}", has_file_evidence),
        format!("quality_gates_passed:{}", quality_gates_passed),
    ];
    let max_files = 40usize;
    for path in execution_outcome.changed_files.iter().take(max_files) {
        kr_outcome.evidence.push(format!("file:{path}"));
    }
    if execution_outcome.changed_files.len() > max_files {
        kr_outcome.evidence.push(format!(
            "files_truncated:{}",
            execution_outcome.changed_files.len() - max_files
        ));
    }
    kr.add_outcome(kr_outcome);

    if matches!(okr.status, OkrStatus::Draft | OkrStatus::OnHold) && after_progress > 0.0 {
        okr.status = OkrStatus::Active;
    }
    if okr.is_complete() {
        okr.status = OkrStatus::Completed;
    }

    let _ = repo.update_okr(okr).await?;
    Ok(())
}

fn success_progress_increment_ratio(kr: &KeyResult) -> f64 {
    if kr.target_value <= 0.0 {
        return 0.0;
    }
    let remaining_ratio = ((kr.target_value - kr.current_value) / kr.target_value).clamp(0.0, 1.0);
    (remaining_ratio * 0.25).clamp(0.05, 0.15)
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

    // Run cargo check (fast type checking)
    let check_output = Command::new("cargo")
        .args(["check", "--message-format=short"])
        .output();

    match check_output {
        Ok(output) => {
            let status = if output.status.success() {
                "PASS"
            } else {
                all_passed = false;
                "FAIL"
            };
            let stderr = String::from_utf8_lossy(&output.stderr);
            let summary = if stderr.len() > 200 {
                format!("{} [...truncated]", &stderr[..200])
            } else if stderr.is_empty() {
                "no errors".to_string()
            } else {
                stderr.to_string()
            };
            results.push(format!("cargo check: {} - {}", status, summary));
        }
        Err(e) => {
            results.push(format!("cargo check: ERROR - {}", e));
            all_passed = false;
        }
    }

    // Run cargo test (if check passed or for important changes)
    let test_output = Command::new("cargo")
        .args(["test", "--quiet", "--", "--nocapture"])
        .output();

    match test_output {
        Ok(output) => {
            let status = if output.status.success() {
                "PASS"
            } else {
                all_passed = false;
                "FAIL"
            };
            let stdout = String::from_utf8_lossy(&output.stdout);
            let summary = if stdout.len() > 300 {
                format!("{} [...truncated]", &stdout[..300])
            } else if stdout.is_empty() {
                "no test output".to_string()
            } else {
                stdout.to_string()
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

async fn execute_opportunity_with_run(
    item: &ForageOpportunity,
    args: &ForageArgs,
) -> Result<String> {
    let run_args = RunArgs {
        message: item.prompt.clone(),
        continue_session: false,
        session: None,
        model: args.model.clone(),
        agent: Some("build".to_string()),
        format: "default".to_string(),
        file: Vec::new(),
    };
    let timeout_secs = args.run_timeout_secs.clamp(30, 86_400);
    match tokio::time::timeout(
        Duration::from_secs(timeout_secs),
        crate::cli::run::execute(run_args),
    )
    .await
    {
        Ok(Ok(())) => Ok("run execution completed".to_string()),
        Ok(Err(err)) => Err(err),
        Err(_) => anyhow::bail!("run execution timed out after {timeout_secs}s"),
    }
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
                    result.subtask_results.iter().map(|r| r.retry_count).sum::<u32>() as usize
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
        println!(
            "   [bus cycle {cycle}] {} :: topic={} sender={}",
            label, env.topic, env.sender_id
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

fn collect_opportunities_with_rubric(
    okrs: &[Okr],
    moonshots: &MoonshotRubric,
) -> Vec<ForageOpportunity> {
    let now = Utc::now();
    let mut items = Vec::new();

    for okr in okrs {
        let status_weight = status_weight(okr.status);
        if status_weight <= 0.0 {
            continue;
        }

        for kr in &okr.key_results {
            let progress = kr.progress().clamp(0.0, 1.0);
            if progress >= 1.0 {
                continue;
            }
            let remaining = (1.0 - progress).clamp(0.0, 1.0);
            let urgency_bonus = urgency_bonus(okr.target_date, now);
            let alignment_context = format!(
                "{} {} {} {}",
                okr.title, okr.description, kr.title, kr.description
            );
            let moonshot_alignment = moonshot_alignment_score(&alignment_context, &moonshots.goals);
            if moonshots.required && moonshot_alignment < moonshots.min_alignment {
                continue;
            }
            let moonshot_hits = matching_moonshots(&alignment_context, &moonshots.goals);
            let moonshot_bonus = if moonshots.goals.is_empty() {
                0.0
            } else {
                (moonshot_alignment * 0.5).min(0.5)
            };
            let score = (remaining * status_weight) + urgency_bonus + moonshot_bonus;
            let prompt = build_execution_prompt(okr, kr, moonshots, moonshot_alignment);

            items.push(ForageOpportunity {
                score,
                okr_id: okr.id,
                okr_title: okr.title.clone(),
                okr_status: okr.status,
                key_result_id: kr.id,
                key_result_title: kr.title.clone(),
                progress,
                remaining,
                target_date: okr.target_date,
                moonshot_alignment,
                moonshot_hits,
                prompt,
            });
        }
    }

    items.sort_by(
        |a, b| match b.score.partial_cmp(&a.score).unwrap_or(Ordering::Equal) {
            Ordering::Equal => b
                .remaining
                .partial_cmp(&a.remaining)
                .unwrap_or(Ordering::Equal),
            other => other,
        },
    );
    items
}

fn tokenize_for_alignment(value: &str) -> HashSet<String> {
    value
        .split(|c: char| !c.is_ascii_alphanumeric())
        .map(|s| s.trim().to_ascii_lowercase())
        .filter(|s| s.len() >= 4)
        .collect()
}

fn moonshot_alignment_score(context: &str, goals: &[String]) -> f64 {
    if goals.is_empty() {
        return 0.0;
    }

    let context_tokens = tokenize_for_alignment(context);
    if context_tokens.is_empty() {
        return 0.0;
    }

    goals
        .iter()
        .map(|goal| {
            let goal_tokens = tokenize_for_alignment(goal);
            if goal_tokens.is_empty() {
                return 0.0;
            }
            let overlap = goal_tokens.intersection(&context_tokens).count() as f64;
            overlap / goal_tokens.len() as f64
        })
        .fold(0.0, f64::max)
}

fn matching_moonshots(context: &str, goals: &[String]) -> Vec<String> {
    if goals.is_empty() {
        return Vec::new();
    }
    let context_tokens = tokenize_for_alignment(context);
    let mut hits = goals
        .iter()
        .filter_map(|goal| {
            let goal_tokens = tokenize_for_alignment(goal);
            if goal_tokens.is_empty() {
                return None;
            }
            let overlap = goal_tokens.intersection(&context_tokens).count();
            if overlap == 0 {
                None
            } else {
                Some((overlap, goal.clone()))
            }
        })
        .collect::<Vec<_>>();
    hits.sort_by(|a, b| b.0.cmp(&a.0));
    hits.into_iter().take(3).map(|(_, goal)| goal).collect()
}

fn status_weight(status: OkrStatus) -> f64 {
    match status {
        OkrStatus::Active => 1.0,
        OkrStatus::Draft => 0.65,
        OkrStatus::OnHold => 0.35,
        OkrStatus::Completed | OkrStatus::Cancelled => 0.0,
    }
}

fn urgency_bonus(target_date: Option<DateTime<Utc>>, now: DateTime<Utc>) -> f64 {
    let Some(target_date) = target_date else {
        return 0.0;
    };

    if target_date <= now {
        return 0.35;
    }

    let days = (target_date - now).num_days();
    if days <= 7 {
        0.25
    } else if days <= 30 {
        0.1
    } else {
        0.0
    }
}

const MAX_PROMPT_FIELD_CHARS: usize = 1_200;

fn normalize_prompt_field(value: &str, max_chars: usize) -> String {
    let compact = value.split_whitespace().collect::<Vec<_>>().join(" ");
    let normalized_max = max_chars.max(32);
    if compact.chars().count() <= normalized_max {
        return compact;
    }

    let mut truncated = compact.chars().take(normalized_max).collect::<String>();
    truncated.push_str(" ...(truncated)");
    truncated
}

fn build_execution_prompt(
    okr: &Okr,
    kr: &KeyResult,
    moonshots: &MoonshotRubric,
    moonshot_alignment: f64,
) -> String {
    let objective = normalize_prompt_field(&okr.title, MAX_PROMPT_FIELD_CHARS);
    let objective_description = normalize_prompt_field(&okr.description, MAX_PROMPT_FIELD_CHARS);
    let key_result = normalize_prompt_field(&kr.title, MAX_PROMPT_FIELD_CHARS);
    let key_result_description = normalize_prompt_field(&kr.description, MAX_PROMPT_FIELD_CHARS);
    let moonshot_section = if moonshots.goals.is_empty() {
        String::new()
    } else {
        format!(
            "\nMoonshot Rubric (strategy filter):\n- {}\nCurrent alignment score: {:.1}%\nDecision rule: prioritize changes that clearly advance one or more moonshots and explain which mission the change moves.",
            moonshots
                .goals
                .iter()
                .map(|g| normalize_prompt_field(g, 160))
                .collect::<Vec<_>>()
                .join("\n- "),
            moonshot_alignment * 100.0
        )
    };

    format!(
        "Business-goal execution task.\n\
Objective: {}\n\
Objective Description: {}\n\
Key Result: {}\n\
KR Description: {}\n\
Current: {:.3} {} | Target: {:.3} {}\n\n\
Execute one concrete, behavior-preserving code change that measurably advances this key result. \
Use tools, validate the change, and report exact evidence tied to the KR.\n\
Focus on local repository changes first; do not do broad web research unless required by the KR.\n\
Return exact changed file paths and at least one verification command result.{}",
        objective,
        objective_description,
        key_result,
        key_result_description,
        kr.current_value,
        kr.unit,
        kr.target_value,
        kr.unit,
        moonshot_section
    )
}

impl ForageOpportunity {
    fn okr_status_label(&self) -> &'static str {
        match self.okr_status {
            OkrStatus::Draft => "draft",
            OkrStatus::Active => "active",
            OkrStatus::Completed => "completed",
            OkrStatus::Cancelled => "cancelled",
            OkrStatus::OnHold => "on_hold",
        }
    }
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
