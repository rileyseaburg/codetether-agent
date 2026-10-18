//! Non-interactive run command

mod events;
mod files;
mod jsonl;
mod knowledge;
mod run_execute;
mod sol_workflow;

use super::RunArgs;
use crate::autochat::model_rotation::{RelayModelRotation, build_round_robin_model_rotation};
use crate::autochat::shared_context::{
    SharedRelayContext, compose_prompt_with_context, distill_context_delta_with_rlm,
    drain_context_updates, publish_context_delta,
};
use crate::autochat::transport::{attach_handoff_receiver, consume_handoff_by_correlation};
use crate::bus::{AgentBus, relay::ProtocolRelayRuntime, relay::RelayAgentProfile};
use crate::okr::{ApprovalDecision, Okr, OkrRepository, OkrRun};
use crate::provider::{ContentPart, Message, Role};
use crate::rlm::{FinalPayload, RlmExecutor};
use crate::session::{Session, import_codex_session_by_id};
use anyhow::Result;
use codetether_relay_plan::{
    RelayProfile, build_relay_profiles, default_relay_okr_template, okr_from_planned_draft,
    truncate_with_ellipsis,
};
use codetether_relay_planner::{
    decide_dynamic_spawn_with_registry, plan_okr_draft_with_registry,
    plan_relay_profiles_with_registry,
};
use codetether_run_command::{
    command_with_optional_args, is_easy_go_command, normalize_cli_go_command,
    resolve_autochat_model, validate_easy_go_task,
};
use serde::Serialize;
use std::{collections::HashMap, io::Write};
use uuid::Uuid;

const AUTOCHAT_MAX_AGENTS: usize = crate::autochat::AUTOCHAT_MAX_AGENTS;
const AUTOCHAT_DEFAULT_AGENTS: usize = crate::autochat::AUTOCHAT_DEFAULT_AGENTS;
const AUTOCHAT_MAX_ROUNDS: usize = crate::autochat::AUTOCHAT_MAX_ROUNDS;
const AUTOCHAT_MAX_DYNAMIC_SPAWNS: usize = crate::autochat::AUTOCHAT_MAX_DYNAMIC_SPAWNS;
const AUTOCHAT_SPAWN_CHECK_MIN_CHARS: usize = crate::autochat::AUTOCHAT_SPAWN_CHECK_MIN_CHARS;
const AUTOCHAT_QUICK_DEMO_TASK: &str = crate::autochat::AUTOCHAT_QUICK_DEMO_TASK;
const AUTOCHAT_RLM_THRESHOLD_CHARS: usize = crate::autochat::AUTOCHAT_RLM_THRESHOLD_CHARS;
const AUTOCHAT_RLM_FALLBACK_CHARS: usize = crate::autochat::AUTOCHAT_RLM_FALLBACK_CHARS;
const AUTOCHAT_RLM_HANDOFF_QUERY: &str = crate::autochat::AUTOCHAT_RLM_HANDOFF_QUERY;

/// Guarded UUID parse that logs warnings on invalid input instead of returning NIL UUID.
/// Returns None for invalid UUIDs, allowing callers to skip operations rather than corrupt data.
fn parse_uuid_guarded(s: &str, context: &str) -> Option<Uuid> {
    match s.parse::<Uuid>() {
        Ok(uuid) => Some(uuid),
        Err(e) => {
            tracing::warn!(
                context,
                uuid_str = %s,
                error = %e,
                "Invalid UUID string - skipping operation"
            );
            None
        }
    }
}

#[derive(Debug, Serialize)]
struct AutochatCliResult {
    status: String,
    relay_id: String,
    model: String,
    agent_count: usize,
    turns: usize,
    agents: Vec<String>,
    final_handoff: String,
    summary: String,
    failure: Option<String>,
    shared_context_items: usize,
    rlm_context_count: usize,
}

fn resolve_provider_for_model_autochat(
    registry: &std::sync::Arc<crate::provider::ProviderRegistry>,
    model_ref: &str,
) -> Option<(std::sync::Arc<dyn crate::provider::Provider>, String)> {
    crate::autochat::model_rotation::resolve_provider_for_model_autochat(registry, model_ref)
}

pub use run_execute::execute;

pub(super) async fn execute_inner(args: RunArgs) -> Result<()> {
    let message = files::with_attachments(args.message.trim(), &args.file)?;
    let jsonl_output = jsonl::enabled(&args.format);
    super::run_checkpoint::validate_auto_continue(args.auto_continue_until)?;

    if message.is_empty() {
        anyhow::bail!("You must provide a message");
    }

    tracing::info!("Running with message: {}", message);

    if args.branches > 1 {
        let runner = crate::swarm::speculative::SpeculativeRunner::new(
            args.branches,
            args.strategies.clone(),
        );
        let cwd = std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
        let specs = runner.build_branches(&cwd, &message)?;
        tracing::info!(branches = %runner.branch_count, "Many-worlds speculative mode enabled");
        for spec in &specs {
            tracing::info!(
                branch = %spec.branch_name,
                strategy = %spec.strategy_prompt,
                "Speculative branch assigned"
            );
        }
        if !jsonl_output {
            println!(
                "[speculative] {} parallel branches queued (collapse controller will race + prune)",
                runner.branch_count
            );
        }
        // TODO: wire through SwarmExecutor when parallel dispatch is ready
    }

    // Load configuration
    let config = super::run_config::load(&args).await;
    let workspace_dir = std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
    let knowledge_snapshot = knowledge::refresh(&workspace_dir).await;

    // Protocol-first relay aliases in CLI:
    // - /go [count] <task>
    // - /autochat [count] <task>
    let easy_go_requested = is_easy_go_command(&message);
    let normalized = normalize_cli_go_command(&message);
    if let Some(rest) = command_with_optional_args(&normalized, "/autochat") {
        let Some(parsed) = crate::autochat::parse_autochat_request(
            rest,
            AUTOCHAT_DEFAULT_AGENTS,
            AUTOCHAT_QUICK_DEMO_TASK,
        ) else {
            anyhow::bail!(
                "Usage: /autochat [count] [--no-prd] <task>\nEasy mode: /go <task>\ncount range: 2-{} (default: {})",
                AUTOCHAT_MAX_AGENTS,
                AUTOCHAT_DEFAULT_AGENTS
            );
        };
        let agent_count = parsed.agent_count;
        let parsed_task = parsed.task;
        let task = if easy_go_requested {
            validate_easy_go_task(&parsed_task)?
        } else {
            normalize_go_task_input(&parsed_task)
        };
        let require_prd = easy_go_requested || !parsed.bypass_prd;
        if require_prd && jsonl_output {
            anyhow::bail!(
                "--format jsonl does not support interactive /go approval; use /autochat --no-prd"
            );
        }

        if !(2..=AUTOCHAT_MAX_AGENTS).contains(&agent_count) {
            anyhow::bail!(
                "Invalid relay size {}. count must be between 2 and {}",
                agent_count,
                AUTOCHAT_MAX_AGENTS
            );
        }

        let model = resolve_autochat_model(
            args.model.as_deref(),
            std::env::var("CODETETHER_DEFAULT_MODEL").ok().as_deref(),
            config.default_model.as_deref(),
            easy_go_requested,
        );

        // PRD-gated execution path (mandatory for /go and default for /autochat)
        if require_prd {
            // For /go, default to max concurrency (run all stories in parallel) unless
            // the user explicitly specified a count. /autochat keeps the requested count.
            let max_concurrent = if easy_go_requested && !parsed.explicit_count {
                AUTOCHAT_MAX_AGENTS
            } else {
                agent_count
            };
            // Create OKR draft (LLM-proposed, with safe fallback)
            let okr_id = Uuid::new_v4();
            let registry_for_draft = crate::provider::ProviderRegistry::shared_from_vault()
                .await
                .ok();

            let (mut okr, draft_note) = if let Some(registry) = &registry_for_draft {
                match plan_okr_draft_with_registry(&task, &model, agent_count, registry).await {
                    Some(planned) => (okr_from_planned_draft(okr_id, &task, planned), None),
                    None => (
                        default_relay_okr_template(okr_id, &task),
                        Some("(OKR: fallback template — model draft parse failed)".to_string()),
                    ),
                }
            } else {
                (
                    default_relay_okr_template(okr_id, &task),
                    Some("(OKR: fallback template — provider unavailable)".to_string()),
                )
            };

            // Create run
            let mut run = OkrRun::new(
                okr_id,
                format!("Run {}", chrono::Local::now().format("%Y-%m-%d %H:%M")),
            );
            let _ = run.submit_for_approval();

            // Show OKR draft
            let command_label = if easy_go_requested {
                "/go"
            } else {
                "/autochat"
            };
            println!("\n⚠️  {command_label} OKR Draft\n");
            println!("Task: {}", truncate_with_ellipsis(&task, 80));
            println!("Agents: {} | Model: {}", agent_count, model);
            if let Some(note) = draft_note {
                println!("{}", note);
            }
            println!("\nObjective: {}", okr.title);
            println!("\nKey Results:");
            for kr in &okr.key_results {
                println!("  • {} (target: {} {})", kr.title, kr.target_value, kr.unit);
            }
            println!("\n");

            // Prompt for approval
            print!("Approve OKR and start relay? [y/n]: ");
            std::io::stdout().flush()?;
            let mut input = String::new();
            std::io::stdin().read_line(&mut input)?;

            let input = input.trim().to_lowercase();
            if input != "y" && input != "yes" {
                run.record_decision(ApprovalDecision::deny(run.id, "User denied via CLI"));
                println!("❌ OKR denied. Relay not started.");
                println!("Use /autochat --no-prd for tactical execution without OKR/PRD tracking.");
                return Ok(());
            }

            println!("✅ OKR approved! Starting Ralph PRD execution...\n");

            // Save OKR and run
            let mut approved_run = run;
            if let Ok(repo) = OkrRepository::from_config().await {
                let _ = repo.create_okr(okr.clone()).await;
                approved_run.record_decision(ApprovalDecision::approve(
                    approved_run.id,
                    "User approved via CLI",
                ));
                approved_run.correlation_id = Some(format!("ralph-{}", Uuid::new_v4()));
                let _ = repo.create_run(approved_run.clone()).await;
                tracing::info!(okr_id = %okr_id, okr_run_id = %approved_run.id, "OKR run approved and saved");
            }

            // Load provider for Ralph execution
            let registry = if let Some(registry) = registry_for_draft {
                registry
            } else {
                crate::provider::ProviderRegistry::shared_from_vault().await?
            };
            let (provider, resolved_model) = resolve_provider_for_model_autochat(&registry, &model)
                .ok_or_else(|| anyhow::anyhow!("No provider available for model '{model}'"))?;

            // Wire bus for training data capture
            let bus = crate::bus::AgentBus::new().into_arc();
            crate::bus::s3_sink::spawn_bus_s3_sink(bus.clone());

            // Execute via Ralph PRD loop — use max_concurrent as concurrency
            let ralph_result = super::go_ralph::execute_go_ralph(
                &task,
                &mut okr,
                &mut approved_run,
                provider,
                &resolved_model,
                10,                     // max iterations
                Some(bus),              // bus for training data
                max_concurrent,         // max concurrent stories
                Some(registry.clone()), // relay registry
            )
            .await?;

            // Persist final run state
            if let Ok(repo) = OkrRepository::from_config().await {
                let _ = repo.update_run(approved_run).await;
            }

            // Display results
            match args.format.as_str() {
                "jsonl" => {
                    let response = super::go_ralph::format_go_ralph_result(&ralph_result, &task);
                    jsonl::write_completed(None, &response, None)?;
                }
                "json" => println!(
                    "{}",
                    serde_json::to_string_pretty(&serde_json::json!({
                        "passed": ralph_result.passed,
                        "total": ralph_result.total,
                        "all_passed": ralph_result.all_passed,
                        "iterations": ralph_result.iterations,
                        "feature_branch": ralph_result.feature_branch,
                        "prd_path": ralph_result.prd_path.display().to_string(),
                        "status": format!("{:?}", ralph_result.status),
                        "stories": ralph_result.stories.iter().map(|s| serde_json::json!({
                            "id": s.id,
                            "title": s.title,
                            "passed": s.passed,
                        })).collect::<Vec<_>>(),
                    }))?
                ),
                _ => {
                    println!(
                        "{}",
                        super::go_ralph::format_go_ralph_result(&ralph_result, &task)
                    );
                }
            }
            return Ok(());
        }

        // Explicit tactical /autochat path (requires --no-prd)
        let relay_result = run_protocol_first_relay(agent_count, &task, &model, None, None).await?;
        match args.format.as_str() {
            "jsonl" => {
                if let Some(failure) = &relay_result.failure {
                    jsonl::write_failed_response(failure, Some(&relay_result.summary))?;
                } else {
                    jsonl::write_completed(None, &relay_result.summary, None)?;
                }
            }
            "json" => println!("{}", serde_json::to_string_pretty(&relay_result)?),
            _ => {
                println!("{}", relay_result.summary);
                if let Some(failure) = &relay_result.failure {
                    eprintln!("\nFailure detail: {}", failure);
                }
                eprintln!(
                    "\n[Relay: {} | Model: {}]",
                    relay_result.relay_id, relay_result.model
                );
            }
        }
        return Ok(());
    }

    // Create or continue session.
    let mut session = if let Some(codex_id) = args.codex_session.clone() {
        tracing::info!("Importing Codex session: {}", codex_id);
        import_codex_session_by_id(&codex_id).await?
    } else if let Some(session_id) = args.session.clone() {
        tracing::info!("Continuing session: {}", session_id);
        Session::resume(&session_id).await?
    } else if args.continue_session {
        match Session::resume_last(Some(&workspace_dir)).await {
            Ok(s) => {
                tracing::info!(
                    session_id = %s.id,
                    workspace = %workspace_dir.display(),
                    "Continuing last workspace session"
                );
                s
            }
            Err(err) => {
                let s = Session::new().await?;
                tracing::warn!(
                    error = %err,
                    session_id = %s.id,
                    workspace = %workspace_dir.display(),
                    "Session lookup failed; created new session"
                );
                s
            }
        }
    } else {
        let s = Session::new().await?;
        tracing::info!("Created new session: {}", s.id);
        s
    };
    if jsonl_output {
        jsonl::write_started(&session.id)?;
    }

    // Set model: CLI arg > env var > config default
    let model = args
        .model
        .clone()
        .or_else(|| std::env::var("CODETETHER_DEFAULT_MODEL").ok())
        .or(config.default_model);

    if let Some(model) = model {
        tracing::info!("Using model: {}", model);
        session.metadata.model = Some(model);
    }

    // Load project memory palace
    let beliefs = crate::memory::palace::load_project_beliefs(&workspace_dir);
    if !beliefs.is_empty() {
        let ctx = crate::memory::palace::belief_context(&beliefs);
        if !ctx.is_empty() {
            tracing::info!(beliefs = beliefs.len(), "Loaded project memory palace");
        }
    }

    session.metadata.knowledge_snapshot = knowledge_snapshot;
    super::run_config::apply_session_policy(&mut session, &args)?;

    // Wire bus for thinking capture + S3 training data
    let bus = AgentBus::new().into_arc();
    crate::bus::set_global(bus.clone());
    crate::bus::s3_sink::spawn_bus_s3_sink(bus.clone());
    session.bus = Some(bus);

    let live_events = events::LiveEvents::start(&session.id, &message, jsonl_output).await?;

    let result = match super::run_loop::execute_prompt_with_resume_events(
        &mut session,
        &message,
        args.max_steps,
        args.auto_continue_until,
        &workspace_dir,
        live_events.sender(),
    )
    .await
    {
        Ok(result) => {
            live_events.finish_success(&result.text).await?;
            result
        }
        Err(error) => {
            let message = error.to_string();
            live_events.finish_failure(&message).await?;
            return Err(error);
        }
    };

    // Output based on format
    match args.format.as_str() {
        "jsonl" => jsonl::write_completed(
            Some(result.session_id.as_str()),
            &result.text,
            jsonl::usage(&session.usage),
        )?,
        "json" => {
            println!("{}", serde_json::to_string_pretty(&result)?);
        }
        _ => {
            println!("{}", result.text);
            // Show session ID for continuation
            eprintln!(
                "\n[Session: {} | Continue with: codetether run -c \"...\"]",
                session.id
            );
        }
    }

    Ok(())
}

fn normalize_for_convergence(text: &str) -> String {
    crate::autochat::normalize_for_convergence(text, 280)
}

fn extract_semantic_handoff_from_rlm(answer: &str) -> String {
    match FinalPayload::parse(answer) {
        FinalPayload::Semantic(payload) => payload.answer,
        _ => answer.trim().to_string(),
    }
}

async fn prepare_autochat_handoff_with_registry(
    task: &str,
    from_agent: &str,
    output: &str,
    model_ref: &str,
    registry: Option<&std::sync::Arc<crate::provider::ProviderRegistry>>,
) -> (String, bool) {
    let mut used_rlm = false;
    let mut relay_payload = if output.len() > AUTOCHAT_RLM_THRESHOLD_CHARS {
        truncate_with_ellipsis(output, AUTOCHAT_RLM_FALLBACK_CHARS)
    } else {
        output.to_string()
    };

    if let Some(registry) = registry
        && let Some((provider, model_name)) =
            resolve_provider_for_model_autochat(registry, model_ref)
    {
        let mut executor =
            RlmExecutor::new(output.to_string(), provider, model_name).with_max_iterations(2);
        match executor.analyze(AUTOCHAT_RLM_HANDOFF_QUERY).await {
            Ok(result) => {
                let normalized = extract_semantic_handoff_from_rlm(&result.answer);
                if !normalized.is_empty() {
                    relay_payload = normalized;
                    used_rlm = true;
                }
            }
            Err(err) => {
                tracing::warn!(
                    error = %err,
                    "CLI RLM handoff normalization failed; using fallback payload"
                );
            }
        }
    }

    (
        format!(
            "Relay task:\n{task}\n\nIncoming handoff from @{from_agent}:\n{relay_payload}\n\n\
             Continue the work from this handoff. Keep your response focused and provide one concrete next-step instruction for the next agent."
        ),
        used_rlm,
    )
}

async fn run_protocol_first_relay(
    agent_count: usize,
    task: &str,
    model_ref: &str,
    okr_id: Option<Uuid>,
    okr_run_id: Option<Uuid>,
) -> Result<AutochatCliResult> {
    let bus = AgentBus::new().into_arc();

    // Auto-start S3 sink if MinIO is configured
    crate::bus::s3_sink::spawn_bus_s3_sink(bus.clone());

    let relay = ProtocolRelayRuntime::new(bus.clone());

    let registry = crate::provider::ProviderRegistry::shared_from_vault()
        .await
        .ok();

    let mut planner_used = false;
    let profiles = if let Some(registry) = &registry {
        if let Some(planned) =
            plan_relay_profiles_with_registry(task, model_ref, agent_count, registry).await
        {
            planner_used = true;
            planned
        } else {
            build_relay_profiles(agent_count)
        }
    } else {
        build_relay_profiles(agent_count)
    };

    let relay_profiles: Vec<RelayAgentProfile> = profiles
        .iter()
        .map(|profile| RelayAgentProfile {
            name: profile.name.clone(),
            capabilities: profile.capabilities.clone(),
        })
        .collect();

    let mut ordered_agents: Vec<String> = profiles
        .iter()
        .map(|profile| profile.name.clone())
        .collect();
    let mut sessions: HashMap<String, Session> = HashMap::new();
    let mut relay_receivers: HashMap<String, crate::bus::BusHandle> = HashMap::new();
    let mut agent_models: HashMap<String, String> = HashMap::new();
    let mut model_rotation = if let Some(registry) = &registry {
        build_round_robin_model_rotation(registry, model_ref).await
    } else {
        RelayModelRotation::fallback(model_ref)
    };

    for profile in &profiles {
        let assigned_model_ref = model_rotation.next_model_ref(model_ref);
        let mut session = Session::new().await?;
        session.metadata.model = Some(assigned_model_ref.clone());
        session.set_agent_name(profile.name.clone());
        session.bus = Some(bus.clone());
        session.add_message(Message {
            role: Role::System,
            content: vec![ContentPart::Text {
                text: profile.instructions.clone(),
            }],
        });
        agent_models.insert(profile.name.clone(), assigned_model_ref);
        sessions.insert(profile.name.clone(), session);
        attach_handoff_receiver(&mut relay_receivers, bus.clone(), &profile.name);
    }

    if ordered_agents.len() < 2 {
        anyhow::bail!("Autochat needs at least 2 agents to relay.");
    }

    relay.register_agents(&relay_profiles);
    let mut context_receiver = bus.handle(format!("relay-context-{}", relay.relay_id()));
    let mut shared_context = SharedRelayContext::default();

    // Load KR targets if OKR is associated
    let kr_targets: std::collections::HashMap<String, f64> =
        if let (Some(okr_id_val), Some(_run_id)) = (okr_id, okr_run_id) {
            if let Ok(repo) = crate::okr::persistence::OkrRepository::from_config().await {
                if let Ok(Some(okr)) = repo.get_okr(okr_id_val).await {
                    okr.key_results
                        .iter()
                        .map(|kr| (kr.id.to_string(), kr.target_value))
                        .collect()
                } else {
                    std::collections::HashMap::new()
                }
            } else {
                std::collections::HashMap::new()
            }
        } else {
            std::collections::HashMap::new()
        };

    let mut kr_progress: std::collections::HashMap<String, f64> = std::collections::HashMap::new();

    let mut baton = format!(
        "Task:\n{task}\n\nStart by proposing an execution strategy and one immediate next step."
    );
    let mut previous_normalized: Option<String> = None;
    let mut convergence_hits = 0usize;
    let mut turns = 0usize;
    let mut dynamic_spawn_count = 0usize;
    let mut rlm_handoff_count = 0usize;
    let mut rlm_context_count = 0usize;
    let mut status = crate::autochat::AUTOCHAT_STATUS_MAX_ROUNDS_REACHED.to_string();
    let mut failure_note: Option<String> = None;

    'relay_loop: for round in 1..=AUTOCHAT_MAX_ROUNDS {
        let mut idx = 0usize;
        while idx < ordered_agents.len() {
            let to = ordered_agents[idx].clone();
            let from = if idx == 0 {
                if round == 1 {
                    "user".to_string()
                } else {
                    ordered_agents[ordered_agents.len() - 1].clone()
                }
            } else {
                ordered_agents[idx - 1].clone()
            };

            turns += 1;
            let _ =
                drain_context_updates(&mut context_receiver, relay.relay_id(), &mut shared_context);
            let correlation_id = relay.send_handoff(&from, &to, &baton);
            let consumed_handoff = match consume_handoff_by_correlation(
                &mut relay_receivers,
                &to,
                &correlation_id,
            )
            .await
            {
                Ok(handoff) => handoff,
                Err(err) => {
                    status = "bus_error".to_string();
                    failure_note = Some(format!(
                        "Failed to consume handoff for @{to} (correlation={correlation_id}): {err}"
                    ));
                    break 'relay_loop;
                }
            };
            let prompt_input = compose_prompt_with_context(&consumed_handoff, &shared_context);

            let Some(mut session) = sessions.remove(&to) else {
                status = "agent_error".to_string();
                failure_note = Some(format!("Relay agent @{to} session was unavailable."));
                break 'relay_loop;
            };

            let output = match session.prompt(&prompt_input).await {
                Ok(response) => response.text,
                Err(err) => {
                    status = "agent_error".to_string();
                    failure_note = Some(format!("Relay agent @{to} failed: {err}"));
                    sessions.insert(to, session);
                    break 'relay_loop;
                }
            };

            sessions.insert(to.clone(), session);

            let normalized = normalize_for_convergence(&output);
            if previous_normalized.as_deref() == Some(normalized.as_str()) {
                convergence_hits += 1;
            } else {
                convergence_hits = 0;
            }
            previous_normalized = Some(normalized);

            let turn_model_ref = agent_models
                .get(&to)
                .map(String::as_str)
                .unwrap_or(model_ref);
            let (next_handoff, used_rlm) = prepare_autochat_handoff_with_registry(
                task,
                &to,
                &output,
                turn_model_ref,
                registry.as_ref(),
            )
            .await;
            if used_rlm {
                rlm_handoff_count += 1;
            }
            let turn_context_provider = registry
                .as_ref()
                .and_then(|r| resolve_provider_for_model_autochat(r, turn_model_ref));
            let (context_delta, used_context_rlm) =
                distill_context_delta_with_rlm(&output, task, &to, turn_context_provider).await;
            if used_context_rlm {
                rlm_context_count += 1;
            }
            shared_context.merge_delta(&context_delta);
            let publisher = bus.handle(to.clone());
            publish_context_delta(
                &publisher,
                relay.relay_id(),
                &to,
                round,
                turns,
                &context_delta,
            );
            baton = next_handoff;

            // Update KR progress after each turn
            if !kr_targets.is_empty() {
                let max_turns = ordered_agents.len() * AUTOCHAT_MAX_ROUNDS;
                let progress_ratio = (turns as f64 / max_turns as f64).min(1.0);

                for (kr_id, target) in &kr_targets {
                    let current = progress_ratio * target;
                    let existing = kr_progress.get(kr_id).copied().unwrap_or(0.0);
                    if current > existing {
                        kr_progress.insert(kr_id.clone(), current);
                    }
                }

                // Persist mid-run (best-effort)
                if let Some(run_id) = okr_run_id
                    && let Ok(repo) = crate::okr::persistence::OkrRepository::from_config().await
                    && let Ok(Some(mut run)) = repo.get_run(run_id).await
                    && run.is_resumable()
                {
                    run.iterations = turns as u32;
                    for (kr_id, value) in &kr_progress {
                        run.update_kr_progress(kr_id, *value);
                    }
                    run.status = crate::okr::OkrRunStatus::Running;
                    let _ = repo.update_run(run).await;
                }
            }

            let can_attempt_spawn = dynamic_spawn_count < AUTOCHAT_MAX_DYNAMIC_SPAWNS
                && ordered_agents.len() < AUTOCHAT_MAX_AGENTS
                && output.len() >= AUTOCHAT_SPAWN_CHECK_MIN_CHARS;

            if can_attempt_spawn
                && let Some(registry) = &registry
                && let Some((profile, reason)) = decide_dynamic_spawn_with_registry(
                    task,
                    model_ref,
                    &output,
                    round,
                    &ordered_agents,
                    registry,
                )
                .await
            {
                match Session::new().await {
                    Ok(mut spawned_session) => {
                        let spawned_model_ref = model_rotation.next_model_ref(model_ref);
                        spawned_session.metadata.model = Some(spawned_model_ref.clone());
                        spawned_session.set_agent_name(profile.name.clone());
                        spawned_session.bus = Some(bus.clone());
                        spawned_session.add_message(Message {
                            role: Role::System,
                            content: vec![ContentPart::Text {
                                text: profile.instructions.clone(),
                            }],
                        });

                        relay.register_agents(&[RelayAgentProfile {
                            name: profile.name.clone(),
                            capabilities: profile.capabilities.clone(),
                        }]);
                        attach_handoff_receiver(&mut relay_receivers, bus.clone(), &profile.name);

                        ordered_agents.insert(idx + 1, profile.name.clone());
                        agent_models.insert(profile.name.clone(), spawned_model_ref);
                        sessions.insert(profile.name.clone(), spawned_session);
                        dynamic_spawn_count += 1;

                        tracing::info!(
                            agent = %profile.name,
                            reason = %reason,
                            "Dynamic relay spawn accepted"
                        );
                    }
                    Err(err) => {
                        tracing::warn!(
                            agent = %profile.name,
                            error = %err,
                            "Dynamic relay spawn requested but failed"
                        );
                    }
                }
            }

            if convergence_hits >= 2 {
                status = "converged".to_string();
                break 'relay_loop;
            }

            idx += 1;
        }
    }

    relay.shutdown_agents(&ordered_agents);

    // Update OKR run with final progress if associated
    if let Some(run_id) = okr_run_id
        && let Ok(repo) = crate::okr::persistence::OkrRepository::from_config().await
        && let Ok(Some(mut run)) = repo.get_run(run_id).await
    {
        // Update KR progress from execution
        for (kr_id, value) in &kr_progress {
            run.update_kr_progress(kr_id, *value);
        }

        // Create outcomes per KR with progress (link to actual KR IDs)
        let base_evidence = vec![
            format!("relay:{}", relay.relay_id()),
            format!("turns:{}", turns),
            format!("agents:{}", ordered_agents.len()),
            format!("status:{}", status),
            format!("rlm_handoffs:{}", rlm_handoff_count),
            format!("dynamic_spawns:{}", dynamic_spawn_count),
        ];

        let outcome_type = if status == "converged" {
            crate::okr::KrOutcomeType::FeatureDelivered
        } else {
            crate::okr::KrOutcomeType::Evidence
        };

        // Create one outcome per KR, linked to the actual KR ID
        for (kr_id_str, value) in &kr_progress {
            // Parse KR ID with guardrail to prevent NIL UUID linkage
            if let Some(kr_uuid) = parse_uuid_guarded(kr_id_str, "cli_relay_outcome_kr_link") {
                let kr_description = format!(
                    "CLI relay outcome for KR {}: {} agents, {} turns, status={}",
                    kr_id_str,
                    ordered_agents.len(),
                    turns,
                    status
                );
                run.outcomes.push({
                    let mut outcome =
                        crate::okr::KrOutcome::new(kr_uuid, kr_description).with_value(*value);
                    outcome.run_id = Some(run.id);
                    outcome.outcome_type = outcome_type;
                    outcome.evidence = base_evidence.clone();
                    outcome.source = "cli relay".to_string();
                    outcome
                });
            }
        }

        // Mark complete or update status based on execution result
        if status == "converged" {
            run.complete();
        } else if status == "agent_error" || status == "bus_error" {
            run.status = crate::okr::OkrRunStatus::Failed;
        } else {
            run.status = crate::okr::OkrRunStatus::Completed;
        }
        let _ = repo.update_run(run).await;
    }

    let mut summary = format!(
        "Autochat complete ({status}) — relay {} with {} agents over {} turns.\n\nFinal relay handoff:\n{}",
        relay.relay_id(),
        ordered_agents.len(),
        turns,
        truncate_with_ellipsis(&baton, 4_000)
    );
    if let Some(note) = &failure_note {
        summary.push_str(&format!("\n\nFailure detail: {note}"));
    }
    if planner_used {
        summary.push_str("\n\nTeam planning: model-organized profiles.");
    } else {
        summary.push_str("\n\nTeam planning: fallback self-organizing profiles.");
    }
    if rlm_handoff_count > 0 {
        summary.push_str(&format!("\nRLM-normalized handoffs: {rlm_handoff_count}"));
    }
    if rlm_context_count > 0 {
        summary.push_str(&format!("\nRLM context deltas: {rlm_context_count}"));
    }
    if shared_context.item_count() > 0 {
        summary.push_str(&format!(
            "\nShared context items: {}",
            shared_context.item_count()
        ));
    }
    if dynamic_spawn_count > 0 {
        summary.push_str(&format!("\nDynamic relay spawns: {dynamic_spawn_count}"));
    }

    Ok(AutochatCliResult {
        status,
        relay_id: relay.relay_id().to_string(),
        model: model_ref.to_string(),
        agent_count: ordered_agents.len(),
        turns,
        agents: ordered_agents,
        final_handoff: baton,
        summary,
        failure: failure_note,
        shared_context_items: shared_context.item_count(),
        rlm_context_count,
    })
}
