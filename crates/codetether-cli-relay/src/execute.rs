//! Protocol-first relay orchestration across bus, sessions and shared context.
use crate::Session;
use crate::convergence::normalize_for_convergence;
use anyhow::Result;
use codetether_autochat_core::model_rotation::{
    RelayModelRotation, build_round_robin_model_rotation, resolve_provider_for_model_autochat,
};
use codetether_autochat_core::shared_context::{
    SharedRelayContext, compose_prompt_with_context, drain_context_updates, publish_context_delta,
};
use codetether_autochat_core::transport::{
    attach_handoff_receiver, consume_handoff_by_correlation,
};
use codetether_autochat_core::{
    AUTOCHAT_MAX_AGENTS, AUTOCHAT_MAX_DYNAMIC_SPAWNS, AUTOCHAT_MAX_ROUNDS,
    AUTOCHAT_SPAWN_CHECK_MIN_CHARS,
};
use codetether_bus_core::{
    AgentBus,
    relay::{ProtocolRelayRuntime, RelayAgentProfile},
};
use codetether_relay_plan::build_relay_profiles;
use codetether_relay_planner::{
    decide_dynamic_spawn_with_registry, plan_relay_profiles_with_registry,
};
use codetether_relay_report::AutochatCliResult;
use codetether_relay_rlm::{
    distill_context_delta_with_rlm, prepare_autochat_handoff_with_registry,
};
use std::collections::HashMap;
use uuid::Uuid;

/// Run a bounded multi-agent relay and report its final outcome.
/// # Arguments
/// * `agent_count` - Requested initial team size.
/// * `task` - User task to execute.
/// * `model_ref` - Requested provider/model reference.
/// * `okr_id` - Optional objective whose targets will track progress.
/// * `okr_run_id` - Optional run receiving progress and final evidence.
/// # Returns
/// The final status, team, handoff and summary.
/// # Errors
/// Returns initial session creation errors or an unusably small team.
/// Per-turn bus and session failures are recorded in the returned status.
/// # Examples
/// ```no_run
/// # async fn example<H: codetether_cli_relay::Host>() -> anyhow::Result<()> {
/// let result = codetether_cli_relay::run_protocol_first_relay::<H>(2, "repair", "provider/model", None, None).await?;
/// # Ok(()) }
/// ```
pub async fn run_protocol_first_relay<H: crate::Host>(
    agent_count: usize,
    task: &str,
    model_ref: &str,
    okr_id: Option<Uuid>,
    okr_run_id: Option<Uuid>,
) -> Result<AutochatCliResult> {
    let bus = AgentBus::new().into_arc();

    // Auto-start S3 sink if MinIO is configured
    H::spawn_bus_sink(bus.clone());

    let relay = ProtocolRelayRuntime::new(bus.clone());

    let registry = H::load_registry().await.ok();

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
    let mut sessions: HashMap<String, H::Session> = HashMap::new();
    let mut relay_receivers: HashMap<String, codetether_bus_core::BusHandle> = HashMap::new();
    let mut agent_models: HashMap<String, String> = HashMap::new();
    let mut model_rotation = if let Some(registry) = &registry {
        build_round_robin_model_rotation(registry.as_ref(), model_ref).await
    } else {
        RelayModelRotation::fallback(model_ref)
    };

    for profile in &profiles {
        let assigned_model_ref = model_rotation.next_model_ref(model_ref);
        let mut session = H::Session::new().await?;
        session.configure(
            assigned_model_ref.clone(),
            profile.name.clone(),
            bus.clone(),
            profile.instructions.clone(),
        );
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
    let kr_targets = codetether_relay_okr::progress::targets(okr_id, okr_run_id).await;

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
    let mut status = codetether_autochat_core::AUTOCHAT_STATUS_MAX_ROUNDS_REACHED.to_string();
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
                .and_then(|r| resolve_provider_for_model_autochat(r.as_ref(), turn_model_ref));
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
            codetether_relay_okr::progress::advance(
                &kr_targets,
                &mut kr_progress,
                turns,
                ordered_agents.len(),
                AUTOCHAT_MAX_ROUNDS,
                okr_run_id,
            )
            .await;

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
                match H::Session::new().await {
                    Ok(mut spawned_session) => {
                        let spawned_model_ref = model_rotation.next_model_ref(model_ref);
                        spawned_session.configure(
                            spawned_model_ref.clone(),
                            profile.name.clone(),
                            bus.clone(),
                            profile.instructions.clone(),
                        );

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
    codetether_relay_okr::progress::finish(
        okr_run_id,
        &kr_progress,
        codetether_relay_okr::progress::Report {
            relay_id: relay.relay_id(),
            turns,
            agent_count: ordered_agents.len(),
            status: &status,
            rlm_handoff_count,
            dynamic_spawn_count,
        },
    )
    .await;

    let summary = codetether_relay_report::summary(&codetether_relay_report::Summary {
        status: &status,
        relay_id: relay.relay_id(),
        agent_count: ordered_agents.len(),
        turns,
        final_handoff: &baton,
        failure: failure_note.as_deref(),
        planner_used,
        rlm_handoff_count,
        rlm_context_count,
        shared_context_items: shared_context.item_count(),
        dynamic_spawn_count,
    });

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
