use crate::a2a::types::{Part, TaskState};
use crate::audit::{self, AuditCategory, AuditLog, AuditOutcome};
use crate::bus::{AgentBus, BusHandle, BusMessage};
use crate::cli::{ForageArgs, RunArgs};
use crate::okr::{KeyResult, Okr, OkrRepository, OkrStatus};
use crate::swarm::{DecompositionStrategy, ExecutionMode, SwarmConfig, SwarmExecutor};
use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::Serialize;
use serde_json::json;
use std::cmp::Ordering;
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
    prompt: String,
}

pub async fn execute(args: ForageArgs) -> Result<()> {
    ensure_audit_log_initialized().await;
    let repo = OkrRepository::from_config().await?;
    let bus = AgentBus::new().into_arc();
    // Mirror CLI/TUI behavior: archive forage bus traffic to S3 when configured.
    crate::bus::s3_sink::spawn_bus_s3_sink(bus.clone());
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
        })),
        None,
        None,
    )
    .await;

    let top = args.top.clamp(1, 50);
    let interval_secs = args.interval_secs.clamp(5, 86_400);
    let mut cycle: usize = 0;

    loop {
        cycle = cycle.saturating_add(1);
        let cycle_task_id = format!("forage-cycle-{cycle}");
        let _ = bus_handle.send_task_update(
            &cycle_task_id,
            TaskState::Working,
            Some("scanning OKR opportunities".to_string()),
        );
        let opportunities = build_opportunities(&repo).await?;
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
                    Ok(detail) => {
                        let _ = bus_handle.send_task_update(
                            &exec_task_id,
                            TaskState::Completed,
                            Some(detail.clone()),
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
                                "detail": detail,
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

async fn execute_opportunity(item: &ForageOpportunity, args: &ForageArgs) -> Result<String> {
    match args.execution_engine.as_str() {
        "swarm" => execute_opportunity_with_swarm(item, args).await,
        _ => execute_opportunity_with_run(item, args).await,
    }
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
    let subagent_timeout_secs = args.swarm_subagent_timeout_secs.clamp(30, 86_400);
    let swarm_config = SwarmConfig {
        max_subagents: args.swarm_max_subagents.max(1),
        max_steps_per_subagent: args.swarm_max_steps.max(1),
        subagent_timeout_secs,
        model: args.model.clone().or_else(|| Some("zai/glm-5".to_string())),
        execution_mode: ExecutionMode::LocalThread,
        ..Default::default()
    };
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
                    result.stats.total_retries
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

async fn build_opportunities(repo: &OkrRepository) -> Result<Vec<ForageOpportunity>> {
    let okrs = repo.list_okrs().await?;
    Ok(collect_opportunities(&okrs))
}

fn collect_opportunities(okrs: &[Okr]) -> Vec<ForageOpportunity> {
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
            let score = (remaining * status_weight) + urgency_bonus;
            let prompt = build_execution_prompt(okr, kr);

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

fn build_execution_prompt(okr: &Okr, kr: &KeyResult) -> String {
    format!(
        "Business-goal execution task.\n\
Objective: {}\n\
Objective Description: {}\n\
Key Result: {}\n\
KR Description: {}\n\
Current: {:.3} {} | Target: {:.3} {}\n\n\
Execute one concrete, behavior-preserving code change that measurably advances this key result. \
Use tools, validate the change, and report exact evidence tied to the KR.",
        okr.title,
        okr.description,
        kr.title,
        kr.description,
        kr.current_value,
        kr.unit,
        kr.target_value,
        kr.unit
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
    use super::{collect_opportunities, status_weight, urgency_bonus};
    use crate::okr::{KeyResult, Okr, OkrStatus};
    use chrono::{Duration, Utc};

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
    fn forage_timeout_bounds_are_clamped() {
        let low = 5u64.clamp(30, 86_400);
        let high = 999_999u64.clamp(30, 86_400);
        assert_eq!(low, 30);
        assert_eq!(high, 86_400);
    }
}
