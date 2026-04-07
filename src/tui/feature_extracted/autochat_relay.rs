//! Autochat relay workers and relay checkpoint persistence
//!
//! Multi-agent relay execution: /go Ralph worker, autochat worker,
//! resume from checkpoint, dynamic agent spawning, and handoff logic.

use anyhow::Result;
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::time::Instant;
use tokio::sync::mpsc;
use uuid::Uuid;

use crate::autochat::shared_context::{
    SharedRelayContext, compose_prompt_with_context, distill_context_delta_with_rlm,
    drain_context_updates, publish_context_delta,
};
use crate::autochat::transport::{attach_handoff_receiver, consume_handoff_by_correlation};
use crate::autochat::{
    model_rotation::RelayModelRotation, model_rotation::build_round_robin_model_rotation,
};
use crate::bus::relay::{ProtocolRelayRuntime, RelayAgentProfile};
use crate::config::Config;
use crate::okr::{ApprovalDecision, KeyResult, KrOutcome, KrOutcomeType, Okr, OkrRun, OkrRunStatus};
use crate::provider::{ContentPart, Role};
use crate::ralph::{RalphConfig, RalphLoop};
use crate::rlm::{FinalPayload, RlmExecutor};
use crate::session::{Session, SessionEvent};
use crate::swarm::{DecompositionStrategy, SwarmConfig, SwarmExecutor};


enum AutochatUiEvent {
    Progress(String),
    SystemMessage(String),
    AgentEvent {
        agent_name: String,
        event: Box<SessionEvent>,
    },
    Completed {
        summary: String,
        okr_id: Option<String>,
        okr_run_id: Option<String>,
        relay_id: Option<String>,
    },
}

#[derive(Debug, Clone)]

struct RelayCheckpoint {
    /// Original user task
    task: String,
    /// Primary model reference requested by the user
    model_ref: String,
    /// Ordered list of agent names in relay order
    ordered_agents: Vec<String>,
    /// Session IDs for each agent (agent name → session UUID)
    agent_session_ids: HashMap<String, String>,
    /// Agent profiles: (name, system instructions, capabilities)
    agent_profiles: Vec<(String, String, Vec<String>)>,
    /// Current round (1-based)
    round: usize,
    /// Current agent index within the round
    idx: usize,
    /// The baton text to pass to the next agent
    baton: String,
    /// Total turns completed so far
    turns: usize,
    /// Convergence hit count
    convergence_hits: usize,
    /// Dynamic spawn count
    dynamic_spawn_count: usize,
    /// RLM handoff count
    rlm_handoff_count: usize,
    /// Workspace directory
    workspace_dir: PathBuf,
    /// When the relay was started
    started_at: String,
    /// OKR ID this relay is associated with (if any)
    #[serde(default)]
    okr_id: Option<String>,
    /// OKR run ID this relay is associated with (if any)
    #[serde(default)]
    okr_run_id: Option<String>,
    /// Key result progress cursor: map of kr_id -> current value
    #[serde(default)]
    kr_progress: HashMap<String, f64>,
    /// Shared relay context snapshot built from structured RLM deltas
    #[serde(default)]
    shared_context: SharedRelayContext,
    /// Number of RLM context-delta distillations performed so far
    #[serde(default)]
    rlm_context_count: usize,
    /// Round-robin pool for assigning models to new relay participants
    #[serde(default)]
    model_rotation: RelayModelRotation,
    /// Agent model assignments (agent name -> provider/model)
    #[serde(default)]
    agent_models: HashMap<String, String>,
}

impl RelayCheckpoint {
    fn checkpoint_path() -> Option<PathBuf> {
        crate::config::Config::data_dir().map(|d| d.join("relay_checkpoint.json"))
    }

    async fn save(&self) -> Result<()> {
        if let Some(path) = Self::checkpoint_path() {
            if let Some(parent) = path.parent() {
                tokio::fs::create_dir_all(parent).await?;
            }
            let content = serde_json::to_string_pretty(self)?;
            tokio::fs::write(&path, content).await?;
            tracing::debug!("Relay checkpoint saved");
        }
        Ok(())
    }

    async fn load() -> Option<Self> {
        let path = Self::checkpoint_path()?;
        let content = tokio::fs::read_to_string(&path).await.ok()?;
        serde_json::from_str(&content).ok()
    }

    async fn load_for_workspace(workspace_dir: &Path) -> Option<Self> {
        let checkpoint = Self::load().await?;
        if same_workspace(&checkpoint.workspace_dir, workspace_dir) {
            Some(checkpoint)
        } else {
            tracing::info!(
                checkpoint_workspace = %checkpoint.workspace_dir.display(),
                active_workspace = %workspace_dir.display(),
                "Ignoring relay checkpoint from different workspace"
            );
            None
        }
    }

    async fn delete() {
        if let Some(path) = Self::checkpoint_path() {
            let _ = tokio::fs::remove_file(&path).await;
            tracing::debug!("Relay checkpoint deleted");
        }
    }
}


fn same_workspace(left: &Path, right: &Path) -> bool {
    let left_norm = left.canonicalize().unwrap_or_else(|_| left.to_path_buf());
    let right_norm = right.canonicalize().unwrap_or_else(|_| right.to_path_buf());
    left_norm == right_norm
}

/// Estimate USD cost from model name and token counts.
/// Uses approximate per-million-token pricing for well-known models.

fn resolve_provider_for_model_autochat(
    registry: &std::sync::Arc<crate::provider::ProviderRegistry>,
    model_ref: &str,
) -> Option<(std::sync::Arc<dyn crate::provider::Provider>, String)> {
    crate::autochat::model_rotation::resolve_provider_for_model_autochat(registry, model_ref)
}

#[derive(Debug, Clone, Deserialize)]

fn extract_semantic_handoff_from_rlm(answer: &str) -> String {
    match FinalPayload::parse(answer) {
        FinalPayload::Semantic(payload) => payload.answer,
        _ => answer.trim().to_string(),
    }
}

/// Ralph worker for TUI `/go` approval flow.
///
/// Loads a provider, generates a PRD, runs the Ralph loop, and reports
/// progress back to the TUI via the `AutochatUiEvent` channel.

async fn run_go_ralph_worker(
    tx: mpsc::Sender<AutochatUiEvent>,
    mut okr: crate::okr::Okr,
    mut run: crate::okr::OkrRun,
    task: String,
    model: String,
    bus: Option<std::sync::Arc<crate::bus::AgentBus>>,
    max_concurrent_stories: usize,
) {
    let _ = tx
        .send(AutochatUiEvent::Progress(
            "Loading providers from Vault…".to_string(),
        ))
        .await;

    let registry = match crate::provider::ProviderRegistry::from_vault().await {
        Ok(r) => std::sync::Arc::new(r),
        Err(err) => {
            let _ = tx
                .send(AutochatUiEvent::Completed {
                    summary: format!("❌ Failed to load providers: {err}"),
                    okr_id: Some(okr.id.to_string()),
                    okr_run_id: Some(run.id.to_string()),
                    relay_id: None,
                })
                .await;
            return;
        }
    };

    let (provider, resolved_model) = match resolve_provider_for_model_autochat(&registry, &model) {
        Some(pair) => pair,
        None => {
            let _ = tx
                .send(AutochatUiEvent::Completed {
                    summary: format!("❌ No provider available for model '{model}'"),
                    okr_id: Some(okr.id.to_string()),
                    okr_run_id: Some(run.id.to_string()),
                    relay_id: None,
                })
                .await;
            return;
        }
    };

    let _ = tx
        .send(AutochatUiEvent::Progress(
            "Generating PRD from task and key results…".to_string(),
        ))
        .await;

    let okr_id_str = okr.id.to_string();
    let run_id_str = run.id.to_string();

    match crate::cli::go_ralph::execute_go_ralph(
        &task,
        &mut okr,
        &mut run,
        provider,
        &resolved_model,
        10,
        bus,
        max_concurrent_stories,
        Some(registry.clone()),
    )
    .await
    {
        Ok(result) => {
            // Persist final run state
            if let Ok(repo) = crate::okr::OkrRepository::from_config().await {
                let _ = repo.update_run(run).await;
            }

            let summary = crate::cli::go_ralph::format_go_ralph_result(&result, &task);
            let _ = tx
                .send(AutochatUiEvent::Completed {
                    summary,
                    okr_id: Some(okr_id_str),
                    okr_run_id: Some(run_id_str),
                    relay_id: None,
                })
                .await;
        }
        Err(err) => {
            // Mark run as failed
            run.status = OkrRunStatus::Failed;
            if let Ok(repo) = crate::okr::OkrRepository::from_config().await {
                let _ = repo.update_run(run).await;
            }

            let _ = tx
                .send(AutochatUiEvent::Completed {
                    summary: format!("❌ Ralph execution failed: {err}"),
                    okr_id: Some(okr_id_str),
                    okr_run_id: Some(run_id_str),
                    relay_id: None,
                })
                .await;
        }
    }
}

async fn run_autochat_worker(
    tx: mpsc::Sender<AutochatUiEvent>,
    bus: std::sync::Arc<crate::bus::AgentBus>,
    fallback_profiles: Vec<(String, String, Vec<String>)>,
    task: String,
    model_ref: String,
    okr_id: Option<Uuid>,
    okr_run_id: Option<Uuid>,
) {
    let _ = tx
        .send(AutochatUiEvent::Progress(
            "Loading providers from Vault…".to_string(),
        ))
        .await;

    let registry = match crate::provider::ProviderRegistry::from_vault().await {
        Ok(registry) => std::sync::Arc::new(registry),
        Err(err) => {
            let _ = tx
                .send(AutochatUiEvent::SystemMessage(format!(
                    "Failed to load providers for /autochat: {err}"
                )))
                .await;
            let _ = tx
                .send(AutochatUiEvent::Completed {
                    summary: "Autochat aborted: provider registry unavailable.".to_string(),
                    okr_id: None,
                    okr_run_id: None,
                    relay_id: None,
                })
                .await;
            return;
        }
    };

    let relay = ProtocolRelayRuntime::new(bus.clone());
    let requested_agents = fallback_profiles.len().clamp(2, AUTOCHAT_MAX_AGENTS);

    let planned_profiles = match plan_relay_profiles_with_registry(
        &task,
        &model_ref,
        requested_agents,
        &registry,
    )
    .await
    {
        Some(planned) => {
            let _ = tx
                .send(AutochatUiEvent::Progress(format!(
                    "Model self-organized relay team ({} agents)…",
                    planned.len()
                )))
                .await;
            planned
        }
        None => {
            let _ = tx
                    .send(AutochatUiEvent::SystemMessage(
                        "Dynamic team planning unavailable; using fallback self-organizing relay profiles."
                            .to_string(),
                    ))
                    .await;
            fallback_profiles
        }
    };

    let mut relay_profiles = Vec::with_capacity(planned_profiles.len());
    let mut ordered_agents = Vec::with_capacity(planned_profiles.len());
    let mut sessions: HashMap<String, Session> = HashMap::new();
    let mut relay_receivers: HashMap<String, crate::bus::BusHandle> = HashMap::new();
    let mut setup_errors: Vec<String> = Vec::new();
    let mut checkpoint_profiles: Vec<(String, String, Vec<String>)> = Vec::new();
    let mut kr_progress: HashMap<String, f64> = HashMap::new();
    let mut agent_models: HashMap<String, String> = HashMap::new();
    let mut model_rotation = build_round_robin_model_rotation(&registry, &model_ref).await;

    // Convert Uuid to String for checkpoint storage
    let okr_id_str = okr_id.map(|id| id.to_string());
    let okr_run_id_str = okr_run_id.map(|id| id.to_string());

    // Load KR targets if OKR is associated
    let kr_targets: HashMap<String, f64> =
        if let (Some(okr_id_val), Some(_run_id)) = (&okr_id_str, &okr_run_id_str) {
            if let Ok(repo) = crate::okr::persistence::OkrRepository::from_config().await {
                if let Ok(okr_uuid) = okr_id_val.parse::<Uuid>() {
                    if let Ok(Some(okr)) = repo.get_okr(okr_uuid).await {
                        okr.key_results
                            .iter()
                            .map(|kr| (kr.id.to_string(), kr.target_value))
                            .collect()
                    } else {
                        HashMap::new()
                    }
                } else {
                    HashMap::new()
                }
            } else {
                HashMap::new()
            }
        } else {
            HashMap::new()
        };

    let _ = tx
        .send(AutochatUiEvent::Progress(
            "Initializing relay agent sessions…".to_string(),
        ))
        .await;

    for (name, instructions, capabilities) in planned_profiles {
        match Session::new().await {
            Ok(mut session) => {
                let assigned_model_ref = model_rotation.next_model_ref(&model_ref);
                session.metadata.model = Some(assigned_model_ref.clone());
                session.agent = name.clone();
                session.bus = Some(bus.clone());
                session.add_message(crate::provider::Message {
                    role: Role::System,
                    content: vec![ContentPart::Text {
                        text: instructions.clone(),
                    }],
                });

                relay_profiles.push(RelayAgentProfile {
                    name: name.clone(),
                    capabilities: capabilities.clone(),
                });
                checkpoint_profiles.push((name.clone(), instructions, capabilities));
                ordered_agents.push(name.clone());
                agent_models.insert(name.clone(), assigned_model_ref);
                sessions.insert(name, session);
                if let Some(agent_name) = ordered_agents.last() {
                    attach_handoff_receiver(&mut relay_receivers, bus.clone(), agent_name);
                }
            }
            Err(err) => {
                setup_errors.push(format!(
                    "Failed creating relay agent session @{name}: {err}"
                ));
            }
        }
    }

    if !setup_errors.is_empty() {
        let _ = tx
            .send(AutochatUiEvent::SystemMessage(format!(
                "Relay setup warnings:\n{}",
                setup_errors.join("\n")
            )))
            .await;
    }

    if ordered_agents.len() < 2 {
        let _ = tx
            .send(AutochatUiEvent::SystemMessage(
                "Autochat needs at least 2 agents to relay.".to_string(),
            ))
            .await;
        let _ = tx
            .send(AutochatUiEvent::Completed {
                summary: "Autochat aborted: insufficient relay participants.".to_string(),
                okr_id: None,
                okr_run_id: None,
                relay_id: None,
            })
            .await;
        return;
    }

    relay.register_agents(&relay_profiles);
    let mut context_receiver = bus.handle(format!("relay-context-{}", relay.relay_id()));
    let mut shared_context = SharedRelayContext::default();

    let _ = tx
        .send(AutochatUiEvent::Progress(format!(
            "Relay {} registered {} agents. Starting handoffs…",
            relay.relay_id(),
            ordered_agents.len()
        )))
        .await;

    let roster_profiles = relay_profiles
        .iter()
        .map(|profile| {
            let capability_summary = if profile.capabilities.is_empty() {
                "skills: dynamic-specialist".to_string()
            } else {
                format!("skills: {}", profile.capabilities.join(", "))
            };
            let model_summary = agent_models
                .get(&profile.name)
                .cloned()
                .unwrap_or_else(|| model_ref.clone());

            format!(
                "• {} — {} • model: {}",
                format_agent_identity(&profile.name),
                capability_summary,
                model_summary
            )
        })
        .collect::<Vec<_>>()
        .join("\n");
    let _ = tx
        .send(AutochatUiEvent::SystemMessage(format!(
            "Relay {id} started • model: {model_ref}\n\nTeam personalities:\n{roster_profiles}",
            id = relay.relay_id()
        )))
        .await;

    let mut baton = format!(
        "Task:\n{task}\n\nStart by proposing an execution strategy and one immediate next step."
    );
    let mut previous_normalized: Option<String> = None;
    let mut convergence_hits = 0usize;
    let mut turns = 0usize;
    let mut rlm_handoff_count = 0usize;
    let mut rlm_context_count = 0usize;
    let mut dynamic_spawn_count = 0usize;
    let mut status = crate::autochat::AUTOCHAT_STATUS_MAX_ROUNDS_REACHED;
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
            let handoff_line = format_relay_handoff_line(relay.relay_id(), round, &from, &to);
            let _ = tx
                .send(AutochatUiEvent::Progress(format!(
                    "Round {round}/{AUTOCHAT_MAX_ROUNDS} • {handoff_line}"
                )))
                .await;
            let consumed_handoff = match consume_handoff_by_correlation(
                &mut relay_receivers,
                &to,
                &correlation_id,
            )
            .await
            {
                Ok(handoff) => handoff,
                Err(err) => {
                    status = "bus_error";
                    failure_note = Some(format!(
                        "Failed to consume handoff for @{to} (correlation={correlation_id}): {err}"
                    ));
                    break 'relay_loop;
                }
            };
            let prompt_input = compose_prompt_with_context(&consumed_handoff, &shared_context);

            let Some(mut session) = sessions.remove(&to) else {
                status = "agent_error";
                failure_note = Some(format!("Relay agent @{to} session was unavailable."));
                break 'relay_loop;
            };

            let (event_tx, mut event_rx) = mpsc::channel(256);
            let registry_for_prompt = registry.clone();
            let baton_for_prompt = prompt_input;

            let join = tokio::spawn(async move {
                let result = session
                    .prompt_with_events(&baton_for_prompt, event_tx, registry_for_prompt)
                    .await;
                (session, result)
            });

            while !join.is_finished() {
                while let Ok(event) = event_rx.try_recv() {
                    if !matches!(event, SessionEvent::SessionSync(_)) {
                        let _ = tx
                            .send(AutochatUiEvent::AgentEvent {
                                agent_name: to.clone(),
                                event: Box::new(event),
                            })
                            .await;
                    }
                }
                tokio::time::sleep(Duration::from_millis(20)).await;
            }

            let (updated_session, result) = match join.await {
                Ok(value) => value,
                Err(err) => {
                    status = "agent_error";
                    failure_note = Some(format!("Relay agent @{to} task join error: {err}"));
                    break 'relay_loop;
                }
            };

            while let Ok(event) = event_rx.try_recv() {
                if !matches!(event, SessionEvent::SessionSync(_)) {
                    let _ = tx
                        .send(AutochatUiEvent::AgentEvent {
                            agent_name: to.clone(),
                            event: Box::new(event),
                        })
                        .await;
                }
            }

            sessions.insert(to.clone(), updated_session);

            let output = match result {
                Ok(response) => response.text,
                Err(err) => {
                    status = "agent_error";
                    failure_note = Some(format!("Relay agent @{to} failed: {err}"));
                    let _ = tx
                        .send(AutochatUiEvent::SystemMessage(format!(
                            "Relay agent @{to} failed: {err}"
                        )))
                        .await;
                    break 'relay_loop;
                }
            };

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
                .unwrap_or(model_ref.as_str());
            let (next_handoff, used_rlm) = prepare_autochat_handoff_with_registry(
                &task,
                &to,
                &output,
                turn_model_ref,
                &registry,
            )
            .await;
            if used_rlm {
                rlm_handoff_count += 1;
            }
            let turn_context_provider =
                resolve_provider_for_model_autochat(&registry, turn_model_ref);
            let (context_delta, used_context_rlm) =
                distill_context_delta_with_rlm(&output, &task, &to, turn_context_provider).await;
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
                    // Only update if progress increased (idempotent)
                    if current > existing {
                        kr_progress.insert(kr_id.clone(), current);
                    }
                }

                // Persist mid-run for real-time visibility (best-effort)
                if let Some(ref run_id_str) = okr_run_id_str
                    && let Ok(repo) = crate::okr::persistence::OkrRepository::from_config().await
                    && let Some(run_uuid) = parse_uuid_guarded(run_id_str, "relay_mid_run_persist")
                    && let Ok(Some(mut run)) = repo.get_run(run_uuid).await
                    && run.is_resumable()
                {
                    run.iterations = turns as u32;
                    for (kr_id, value) in &kr_progress {
                        run.update_kr_progress(kr_id, *value);
                    }
                    run.status = OkrRunStatus::Running;
                    let _ = repo.update_run(run).await;
                }
            }
            let can_attempt_spawn = dynamic_spawn_count < AUTOCHAT_MAX_DYNAMIC_SPAWNS
                && ordered_agents.len() < AUTOCHAT_MAX_AGENTS
                && output.len() >= AUTOCHAT_SPAWN_CHECK_MIN_CHARS;

            if can_attempt_spawn
                && let Some((name, instructions, capabilities, reason)) =
                    decide_dynamic_spawn_with_registry(
                        &task,
                        &model_ref,
                        &output,
                        round,
                        &ordered_agents,
                        &registry,
                    )
                    .await
            {
                match Session::new().await {
                    Ok(mut spawned_session) => {
                        let spawned_model_ref = model_rotation.next_model_ref(&model_ref);
                        spawned_session.metadata.model = Some(spawned_model_ref.clone());
                        spawned_session.agent = name.clone();
                        spawned_session.bus = Some(bus.clone());
                        spawned_session.add_message(crate::provider::Message {
                            role: Role::System,
                            content: vec![ContentPart::Text {
                                text: instructions.clone(),
                            }],
                        });

                        relay.register_agents(&[RelayAgentProfile {
                            name: name.clone(),
                            capabilities: capabilities.clone(),
                        }]);

                        ordered_agents.insert(idx + 1, name.clone());
                        checkpoint_profiles.push((name.clone(), instructions, capabilities));
                        agent_models.insert(name.clone(), spawned_model_ref);
                        sessions.insert(name.clone(), spawned_session);
                        attach_handoff_receiver(&mut relay_receivers, bus.clone(), &name);
                        dynamic_spawn_count += 1;

                        let _ = tx
                            .send(AutochatUiEvent::SystemMessage(format!(
                                "Dynamic spawn: {} joined relay after @{to}.\nReason: {reason}",
                                format_agent_identity(&name)
                            )))
                            .await;
                    }
                    Err(err) => {
                        let _ = tx
                            .send(AutochatUiEvent::SystemMessage(format!(
                                "Dynamic spawn requested but failed to create @{name}: {err}"
                            )))
                            .await;
                    }
                }
            }

            if convergence_hits >= 2 {
                status = "converged";
                break 'relay_loop;
            }

            // Save relay checkpoint so a crash can resume from here
            {
                let agent_session_ids: HashMap<String, String> = sessions
                    .iter()
                    .map(|(name, s)| (name.clone(), s.id.clone()))
                    .collect();
                let next_idx = idx + 1;
                let (ck_round, ck_idx) = if next_idx >= ordered_agents.len() {
                    (round + 1, 0)
                } else {
                    (round, next_idx)
                };
                let checkpoint = RelayCheckpoint {
                    task: task.clone(),
                    model_ref: model_ref.clone(),
                    ordered_agents: ordered_agents.clone(),
                    agent_session_ids,
                    agent_profiles: checkpoint_profiles.clone(),
                    round: ck_round,
                    idx: ck_idx,
                    baton: baton.clone(),
                    turns,
                    convergence_hits,
                    dynamic_spawn_count,
                    rlm_handoff_count,
                    workspace_dir: std::env::current_dir().unwrap_or_default(),
                    started_at: chrono::Utc::now().to_rfc3339(),
                    okr_id: okr_id_str.clone(),
                    okr_run_id: okr_run_id_str.clone(),
                    kr_progress: kr_progress.clone(),
                    shared_context: shared_context.clone(),
                    rlm_context_count,
                    model_rotation: model_rotation.clone(),
                    agent_models: agent_models.clone(),
                };
                if let Err(err) = checkpoint.save().await {
                    tracing::warn!("Failed to save relay checkpoint: {err}");
                }
            }

            idx += 1;
        }
    }

    relay.shutdown_agents(&ordered_agents);

    // Relay completed normally — delete the checkpoint
    RelayCheckpoint::delete().await;

    // Update OKR run with progress if associated
    if let Some(ref run_id_str) = okr_run_id_str
        && let Ok(repo) = crate::okr::persistence::OkrRepository::from_config().await
        && let Some(run_uuid) = parse_uuid_guarded(run_id_str, "relay_completion_persist")
        && let Ok(Some(mut run)) = repo.get_run(run_uuid).await
    {
        // Update KR progress from checkpoint
        for (kr_id, value) in &kr_progress {
            run.update_kr_progress(kr_id, *value);
        }

        // Create outcomes per KR with progress (link to actual KR IDs)
        let relay_id = relay.relay_id().to_string();
        let base_evidence = vec![
            format!("relay:{}", relay_id),
            format!("turns:{}", turns),
            format!("agents:{}", ordered_agents.len()),
            format!("status:{}", status),
            format!("rlm_handoffs:{}", rlm_handoff_count),
            format!("rlm_context_deltas:{}", rlm_context_count),
            format!("shared_context_items:{}", shared_context.item_count()),
            format!("dynamic_spawns:{}", dynamic_spawn_count),
        ];

        // Set outcome type based on status
        let outcome_type = if status == "converged" {
            KrOutcomeType::FeatureDelivered
        } else {
            KrOutcomeType::Evidence
        };

        // Create one outcome per KR, linked to the actual KR ID
        for (kr_id_str, value) in &kr_progress {
            // Parse KR ID with guardrail to prevent NIL UUID linkage
            if let Some(kr_uuid) = parse_uuid_guarded(kr_id_str, "relay_outcome_kr_link") {
                let kr_description = format!(
                    "Relay outcome for KR {}: {} agents, {} turns, status={}",
                    kr_id_str,
                    ordered_agents.len(),
                    turns,
                    status
                );
                run.outcomes.push({
                    let mut outcome = KrOutcome::new(kr_uuid, kr_description).with_value(*value);
                    outcome.run_id = Some(run.id);
                    outcome.outcome_type = outcome_type;
                    outcome.evidence = base_evidence.clone();
                    outcome.source = "autochat relay".to_string();
                    outcome
                });
            }
        }

        // Mark complete or update status based on execution result
        if status == "converged" {
            run.complete();
        } else if status == "agent_error" || status == "bus_error" {
            run.status = OkrRunStatus::Failed;
        } else {
            run.status = OkrRunStatus::Completed;
        }
        // Clear checkpoint ID at completion - checkpoint lifecycle complete
        run.relay_checkpoint_id = None;
        let _ = repo.update_run(run).await;
    }

    let _ = tx
        .send(AutochatUiEvent::Progress(
            "Finalizing relay summary…".to_string(),
        ))
        .await;

    let mut summary = format!(
        "Autochat complete ({status}) — relay {} with {} agents over {} turns.",
        relay.relay_id(),
        ordered_agents.len(),
        turns,
    );
    if let Some(note) = failure_note {
        summary.push_str(&format!("\n\nFailure detail: {note}"));
    }
    if rlm_handoff_count > 0 {
        summary.push_str(&format!("\n\nRLM-normalized handoffs: {rlm_handoff_count}"));
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
    summary.push_str(&format!(
        "\n\nFinal relay handoff:\n{}",
        truncate_with_ellipsis(&baton, 4_000)
    ));
    summary.push_str(&format!(
        "\n\nCleanup: deregistered relay agents and disposed {} autochat worker session(s).",
        sessions.len()
    ));

    let relay_id = relay.relay_id().to_string();
    let okr_id_for_completion = okr_id_str.clone();
    let okr_run_id_for_completion = okr_run_id_str.clone();
    let _ = tx
        .send(AutochatUiEvent::Completed {
            summary,
            okr_id: okr_id_for_completion,
            okr_run_id: okr_run_id_for_completion,
            relay_id: Some(relay_id),
        })
        .await;
}

/// Resume an autochat relay from a persisted checkpoint.
///
/// Reloads agent sessions from disk, reconstructs the relay, and continues
/// from the exact round/index where the previous run was interrupted.
async fn resume_autochat_worker(
    tx: mpsc::Sender<AutochatUiEvent>,
    bus: std::sync::Arc<crate::bus::AgentBus>,
    checkpoint: RelayCheckpoint,
) {
    let _ = tx
        .send(AutochatUiEvent::Progress(
            "Resuming relay — loading providers…".to_string(),
        ))
        .await;

    let registry = match crate::provider::ProviderRegistry::from_vault().await {
        Ok(registry) => std::sync::Arc::new(registry),
        Err(err) => {
            let _ = tx
                .send(AutochatUiEvent::SystemMessage(format!(
                    "Failed to load providers for relay resume: {err}"
                )))
                .await;
            let _ = tx
                .send(AutochatUiEvent::Completed {
                    summary: "Relay resume aborted: provider registry unavailable.".to_string(),
                    okr_id: checkpoint.okr_id.clone(),
                    okr_run_id: checkpoint.okr_run_id.clone(),
                    relay_id: None,
                })
                .await;
            return;
        }
    };

    let relay = ProtocolRelayRuntime::new(bus.clone());
    let task = checkpoint.task;
    let model_ref = checkpoint.model_ref;
    let mut ordered_agents = checkpoint.ordered_agents;
    let mut checkpoint_profiles = checkpoint.agent_profiles;
    let mut baton = checkpoint.baton;
    let mut turns = checkpoint.turns;
    let mut convergence_hits = checkpoint.convergence_hits;
    let mut rlm_handoff_count = checkpoint.rlm_handoff_count;
    let mut rlm_context_count = checkpoint.rlm_context_count;
    let mut dynamic_spawn_count = checkpoint.dynamic_spawn_count;
    let start_round = checkpoint.round;
    let start_idx = checkpoint.idx;
    let okr_run_id_str = checkpoint.okr_run_id.clone();
    let mut kr_progress = checkpoint.kr_progress.clone();
    let mut shared_context = checkpoint.shared_context;
    let mut relay_receivers: HashMap<String, crate::bus::BusHandle> = HashMap::new();
    let mut model_rotation = checkpoint.model_rotation;
    if model_rotation.model_refs.is_empty() {
        model_rotation = build_round_robin_model_rotation(&registry, &model_ref).await;
        model_rotation.cursor = ordered_agents.len();
    }
    let mut agent_models = checkpoint.agent_models;

    // Load KR targets if OKR is associated
    let kr_targets: HashMap<String, f64> =
        if let (Some(okr_id_val), Some(_run_id)) = (&checkpoint.okr_id, &checkpoint.okr_run_id) {
            if let Ok(repo) = crate::okr::persistence::OkrRepository::from_config().await {
                if let Ok(okr_uuid) = okr_id_val.parse::<uuid::Uuid>() {
                    if let Ok(Some(okr)) = repo.get_okr(okr_uuid).await {
                        okr.key_results
                            .iter()
                            .map(|kr| (kr.id.to_string(), kr.target_value))
                            .collect()
                    } else {
                        HashMap::new()
                    }
                } else {
                    HashMap::new()
                }
            } else {
                HashMap::new()
            }
        } else {
            HashMap::new()
        };

    // Persist KR progress immediately after resuming from checkpoint
    if !kr_progress.is_empty()
        && let Some(ref run_id_str) = okr_run_id_str
        && let Ok(repo) = crate::okr::persistence::OkrRepository::from_config().await
        && let Some(run_uuid) = parse_uuid_guarded(run_id_str, "resume_mid_run_persist")
        && let Ok(Some(mut run)) = repo.get_run(run_uuid).await
        && run.is_resumable()
    {
        run.iterations = turns as u32;
        for (kr_id, value) in &kr_progress {
            run.update_kr_progress(kr_id, *value);
        }
        run.status = OkrRunStatus::Running;
        let _ = repo.update_run(run).await;
    }

    // Reload agent sessions from disk
    let mut sessions: HashMap<String, Session> = HashMap::new();
    let mut load_errors: Vec<String> = Vec::new();

    let _ = tx
        .send(AutochatUiEvent::Progress(
            "Reloading agent sessions from disk…".to_string(),
        ))
        .await;

    for (agent_name, session_id) in &checkpoint.agent_session_ids {
        match Session::load(session_id).await {
            Ok(mut session) => {
                session.bus = Some(bus.clone());
                if session.metadata.model.is_none() {
                    let fallback_model = agent_models
                        .get(agent_name)
                        .cloned()
                        .unwrap_or_else(|| model_ref.clone());
                    session.metadata.model = Some(fallback_model);
                }
                if let Some(assigned_model) = session.metadata.model.clone() {
                    agent_models.insert(agent_name.clone(), assigned_model);
                }
                sessions.insert(agent_name.clone(), session);
                attach_handoff_receiver(&mut relay_receivers, bus.clone(), agent_name);
            }
            Err(err) => {
                load_errors.push(format!(
                    "Failed to reload @{agent_name} ({session_id}): {err}"
                ));
            }
        }
    }

    if !load_errors.is_empty() {
        let _ = tx
            .send(AutochatUiEvent::SystemMessage(format!(
                "Session reload warnings:\n{}",
                load_errors.join("\n")
            )))
            .await;
    }

    // Re-register agents with the relay
    let relay_profiles: Vec<RelayAgentProfile> = checkpoint_profiles
        .iter()
        .map(|(name, _, capabilities)| RelayAgentProfile {
            name: name.clone(),
            capabilities: capabilities.clone(),
        })
        .collect();
    relay.register_agents(&relay_profiles);
    let mut context_receiver = bus.handle(format!("relay-context-{}", relay.relay_id()));

    let _ = tx
        .send(AutochatUiEvent::SystemMessage(format!(
            "Resuming relay from round {start_round}, agent index {start_idx}\n\
             Task: {}\n\
             Agents: {}\n\
             Turns completed so far: {turns}",
            truncate_with_ellipsis(&task, 120),
            ordered_agents.join(", ")
        )))
        .await;

    let mut previous_normalized: Option<String> = None;
    let mut status = crate::autochat::AUTOCHAT_STATUS_MAX_ROUNDS_REACHED;
    let mut failure_note: Option<String> = None;

    'relay_loop: for round in start_round..=AUTOCHAT_MAX_ROUNDS {
        let first_idx = if round == start_round { start_idx } else { 0 };
        let mut idx = first_idx;
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
            let handoff_line = format_relay_handoff_line(relay.relay_id(), round, &from, &to);
            let _ = tx
                .send(AutochatUiEvent::Progress(format!(
                    "Round {round}/{AUTOCHAT_MAX_ROUNDS} • {handoff_line} (resumed)"
                )))
                .await;
            let consumed_handoff = match consume_handoff_by_correlation(
                &mut relay_receivers,
                &to,
                &correlation_id,
            )
            .await
            {
                Ok(handoff) => handoff,
                Err(err) => {
                    status = "bus_error";
                    failure_note = Some(format!(
                        "Failed to consume handoff for @{to} (correlation={correlation_id}): {err}"
                    ));
                    break 'relay_loop;
                }
            };
            let prompt_input = compose_prompt_with_context(&consumed_handoff, &shared_context);

            let Some(mut session) = sessions.remove(&to) else {
                status = "agent_error";
                failure_note = Some(format!("Relay agent @{to} session was unavailable."));
                break 'relay_loop;
            };

            let (event_tx, mut event_rx) = mpsc::channel(256);
            let registry_for_prompt = registry.clone();
            let baton_for_prompt = prompt_input;

            let join = tokio::spawn(async move {
                let result = session
                    .prompt_with_events(&baton_for_prompt, event_tx, registry_for_prompt)
                    .await;
                (session, result)
            });

            while !join.is_finished() {
                while let Ok(event) = event_rx.try_recv() {
                    if !matches!(event, SessionEvent::SessionSync(_)) {
                        let _ = tx
                            .send(AutochatUiEvent::AgentEvent {
                                agent_name: to.clone(),
                                event: Box::new(event),
                            })
                            .await;
                    }
                }
                tokio::time::sleep(Duration::from_millis(20)).await;
            }

            let (updated_session, result) = match join.await {
                Ok(value) => value,
                Err(err) => {
                    status = "agent_error";
                    failure_note = Some(format!("Relay agent @{to} task join error: {err}"));
                    break 'relay_loop;
                }
            };

            while let Ok(event) = event_rx.try_recv() {
                if !matches!(event, SessionEvent::SessionSync(_)) {
                    let _ = tx
                        .send(AutochatUiEvent::AgentEvent {
                            agent_name: to.clone(),
                            event: Box::new(event),
                        })
                        .await;
                }
            }

            sessions.insert(to.clone(), updated_session);

            let output = match result {
                Ok(response) => response.text,
                Err(err) => {
                    status = "agent_error";
                    failure_note = Some(format!("Relay agent @{to} failed: {err}"));
                    let _ = tx
                        .send(AutochatUiEvent::SystemMessage(format!(
                            "Relay agent @{to} failed: {err}"
                        )))
                        .await;
                    break 'relay_loop;
                }
            };

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
                .unwrap_or(model_ref.as_str());
            let (next_handoff, used_rlm) = prepare_autochat_handoff_with_registry(
                &task,
                &to,
                &output,
                turn_model_ref,
                &registry,
            )
            .await;
            if used_rlm {
                rlm_handoff_count += 1;
            }
            let turn_context_provider =
                resolve_provider_for_model_autochat(&registry, turn_model_ref);
            let (context_delta, used_context_rlm) =
                distill_context_delta_with_rlm(&output, &task, &to, turn_context_provider).await;
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
                    // Only update if progress increased (idempotent)
                    if current > existing {
                        kr_progress.insert(kr_id.clone(), current);
                    }
                }

                // Persist mid-run for real-time visibility (best-effort)
                if let Some(ref run_id_str) = okr_run_id_str
                    && let Ok(repo) = crate::okr::persistence::OkrRepository::from_config().await
                    && let Some(run_uuid) =
                        parse_uuid_guarded(run_id_str, "resumed_relay_mid_run_persist")
                    && let Ok(Some(mut run)) = repo.get_run(run_uuid).await
                    && run.is_resumable()
                {
                    run.iterations = turns as u32;
                    for (kr_id, value) in &kr_progress {
                        run.update_kr_progress(kr_id, *value);
                    }
                    run.status = OkrRunStatus::Running;
                    let _ = repo.update_run(run).await;
                }
            }

            let can_attempt_spawn = dynamic_spawn_count < AUTOCHAT_MAX_DYNAMIC_SPAWNS
                && ordered_agents.len() < AUTOCHAT_MAX_AGENTS
                && output.len() >= AUTOCHAT_SPAWN_CHECK_MIN_CHARS;

            if can_attempt_spawn
                && let Some((name, instructions, capabilities, reason)) =
                    decide_dynamic_spawn_with_registry(
                        &task,
                        &model_ref,
                        &output,
                        round,
                        &ordered_agents,
                        &registry,
                    )
                    .await
            {
                match Session::new().await {
                    Ok(mut spawned_session) => {
                        let spawned_model_ref = model_rotation.next_model_ref(&model_ref);
                        spawned_session.metadata.model = Some(spawned_model_ref.clone());
                        spawned_session.agent = name.clone();
                        spawned_session.bus = Some(bus.clone());
                        spawned_session.add_message(crate::provider::Message {
                            role: Role::System,
                            content: vec![ContentPart::Text {
                                text: instructions.clone(),
                            }],
                        });

                        relay.register_agents(&[RelayAgentProfile {
                            name: name.clone(),
                            capabilities: capabilities.clone(),
                        }]);

                        ordered_agents.insert(idx + 1, name.clone());
                        checkpoint_profiles.push((name.clone(), instructions, capabilities));
                        agent_models.insert(name.clone(), spawned_model_ref);
                        sessions.insert(name.clone(), spawned_session);
                        attach_handoff_receiver(&mut relay_receivers, bus.clone(), &name);
                        dynamic_spawn_count += 1;

                        let _ = tx
                            .send(AutochatUiEvent::SystemMessage(format!(
                                "Dynamic spawn: {} joined relay after @{to}.\nReason: {reason}",
                                format_agent_identity(&name)
                            )))
                            .await;
                    }
                    Err(err) => {
                        let _ = tx
                            .send(AutochatUiEvent::SystemMessage(format!(
                                "Dynamic spawn requested but failed to create @{name}: {err}"
                            )))
                            .await;
                    }
                }
            }

            if convergence_hits >= 2 {
                status = "converged";
                break 'relay_loop;
            }

            // Save relay checkpoint
            {
                let agent_session_ids: HashMap<String, String> = sessions
                    .iter()
                    .map(|(name, s)| (name.clone(), s.id.clone()))
                    .collect();
                let next_idx = idx + 1;
                let (ck_round, ck_idx) = if next_idx >= ordered_agents.len() {
                    (round + 1, 0)
                } else {
                    (round, next_idx)
                };
                let ck = RelayCheckpoint {
                    task: task.clone(),
                    model_ref: model_ref.clone(),
                    ordered_agents: ordered_agents.clone(),
                    agent_session_ids,
                    agent_profiles: checkpoint_profiles.clone(),
                    round: ck_round,
                    idx: ck_idx,
                    baton: baton.clone(),
                    turns,
                    convergence_hits,
                    dynamic_spawn_count,
                    rlm_handoff_count,
                    workspace_dir: std::env::current_dir().unwrap_or_default(),
                    started_at: chrono::Utc::now().to_rfc3339(),
                    okr_id: checkpoint.okr_id.clone(),
                    okr_run_id: checkpoint.okr_run_id.clone(),
                    kr_progress: kr_progress.clone(),
                    shared_context: shared_context.clone(),
                    rlm_context_count,
                    model_rotation: model_rotation.clone(),
                    agent_models: agent_models.clone(),
                };
                if let Err(err) = ck.save().await {
                    tracing::warn!("Failed to save relay checkpoint: {err}");
                }
            }

            idx += 1;
        }
    }

    relay.shutdown_agents(&ordered_agents);

    // Relay completed normally — delete the checkpoint
    RelayCheckpoint::delete().await;

    // Update OKR run with progress if associated
    if let Some(ref run_id_str) = okr_run_id_str
        && let Ok(repo) = crate::okr::persistence::OkrRepository::from_config().await
        && let Some(run_uuid) = parse_uuid_guarded(run_id_str, "resumed_relay_completion_persist")
        && let Ok(Some(mut run)) = repo.get_run(run_uuid).await
    {
        // Update KR progress from checkpoint
        for (kr_id, value) in &kr_progress {
            run.update_kr_progress(kr_id, *value);
        }

        // Create outcomes per KR with progress (link to actual KR IDs)
        let base_evidence = vec![
            format!("turns:{}", turns),
            format!("agents:{}", ordered_agents.len()),
            format!("status:{}", status),
            "resumed:true".to_string(),
            format!("rlm_handoffs:{}", rlm_handoff_count),
            format!("rlm_context_deltas:{}", rlm_context_count),
            format!("shared_context_items:{}", shared_context.item_count()),
        ];

        let outcome_type = if status == "converged" {
            KrOutcomeType::FeatureDelivered
        } else {
            KrOutcomeType::Evidence
        };

        // Create one outcome per KR, linked to the actual KR ID
        for (kr_id_str, value) in &kr_progress {
            // Parse KR ID with guardrail to prevent NIL UUID linkage
            if let Some(kr_uuid) = parse_uuid_guarded(kr_id_str, "resumed_relay_outcome_kr_link") {
                let kr_description = format!(
                    "Resumed relay outcome for KR {}: {} agents, {} turns, status={}",
                    kr_id_str,
                    ordered_agents.len(),
                    turns,
                    status
                );
                run.outcomes.push({
                    let mut outcome = KrOutcome::new(kr_uuid, kr_description).with_value(*value);
                    outcome.run_id = Some(run.id);
                    outcome.outcome_type = outcome_type;
                    outcome.evidence = base_evidence.clone();
                    outcome.source = "autochat relay (resumed)".to_string();
                    outcome
                });
            }
        }

        // Mark complete or update status based on execution result
        if status == "converged" {
            run.complete();
        } else if status == "agent_error" || status == "bus_error" {
            run.status = OkrRunStatus::Failed;
        } else {
            run.status = OkrRunStatus::Completed;
        }
        // Clear checkpoint ID at completion - checkpoint lifecycle complete
        run.relay_checkpoint_id = None;
        let _ = repo.update_run(run).await;
    }

    let _ = tx
        .send(AutochatUiEvent::Progress(
            "Finalizing resumed relay summary…".to_string(),
        ))
        .await;

    let mut summary = format!(
        "Resumed relay complete ({status}) — {} agents over {} turns.",
        ordered_agents.len(),
        turns,
    );
    if let Some(note) = failure_note {
        summary.push_str(&format!("\n\nFailure detail: {note}"));
    }
    if rlm_handoff_count > 0 {
        summary.push_str(&format!("\n\nRLM-normalized handoffs: {rlm_handoff_count}"));
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
    summary.push_str(&format!(
        "\n\nFinal relay handoff:\n{}",
        truncate_with_ellipsis(&baton, 4_000)
    ));

    let _ = tx
        .send(AutochatUiEvent::Completed {
            summary,
            okr_id: checkpoint.okr_id.clone(),
            okr_run_id: checkpoint.okr_run_id.clone(),
            relay_id: Some(relay.relay_id().to_string()),
        })
        .await;
}

