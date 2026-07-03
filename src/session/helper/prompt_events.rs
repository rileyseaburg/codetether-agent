//! Streaming agentic prompt loop with event emission.
//!
//! This is the streaming counterpart to [`prompt`](super::prompt). It accepts
//! a pre-loaded [`ProviderRegistry`], an optional list of image attachments,
//! and an `mpsc::Sender<SessionEvent>` that receives fine-grained events as
//! the agentic loop progresses (tool-call start/complete, text chunks,
//! thinking output, token usage, etc.). It is the main entry point for the
//! TUI.
// TODO: Keep this loop in sync with `prompt.rs` until both prompt loops are
// consolidated into one shared implementation.

use std::{collections::HashSet, sync::Arc};

use anyhow::Result;
use chrono::Utc;
use tokio::sync::mpsc;
use uuid::Uuid;

use crate::cognition::tool_router::{ToolCallRouter, ToolRouterConfig};
use crate::event_stream::ChatEvent;
use crate::provider::{
    CompletionRequest, ContentPart, Message, ProviderRegistry, Role, parse_model_string,
};
use crate::rlm::RlmConfig;

use super::super::{DEFAULT_MAX_STEPS, ImageAttachment, Session, SessionEvent, SessionResult};
use super::bootstrap::list_tools_bootstrap_output;
use super::build::{
    build_request_requires_tool, is_build_agent, should_force_build_tool_first_retry,
};
use super::confirmation::{
    auto_apply_pending_confirmation, pending_confirmation_tool_result_content,
    tool_result_requires_confirmation,
};
use super::defaults::default_model_for_provider;
use super::edit::{detect_stub_in_tool_input, normalize_tool_call_for_execution};
use super::error::is_retryable_upstream_error;
use super::loop_constants::{
    BUILD_MODE_TOOL_FIRST_MAX_RETRIES, BUILD_MODE_TOOL_FIRST_NUDGE, CODESEARCH_THRASH_NUDGE,
    FORCE_FINAL_ANSWER_NUDGE, MAX_CONSECUTIVE_CODESEARCH_NO_MATCHES, MAX_CONSECUTIVE_SAME_TOOL,
    MAX_STEPS_WITHOUT_PROGRESS, MAX_TOTAL_TOOL_CALLS, NATIVE_TOOL_PROMISE_NUDGE,
    NATIVE_TOOL_PROMISE_RETRY_MAX_RETRIES, NO_PROGRESS_NUDGE, POST_EDIT_VALIDATION_MAX_RETRIES,
};
use super::markup::normalize_textual_tool_calls;
use super::provider::{
    resolve_provider_for_session_request, should_retry_missing_native_tool_call,
};
use super::request_state::build_provider_step_state;
use super::router::{build_proactive_lsp_context_message, choose_router_target_bandit};
use super::runtime::{
    enrich_tool_input_with_runtime_context, is_codesearch_no_match_output, is_interactive_tool,
};
use super::text::extract_text_content;
use super::token::session_completion_max_tokens;
use super::tool_audit_detail::{tool_failure_detail, tool_success_detail};
use super::tool_event_emit as events;
use super::validation::{build_validation_report, capture_git_dirty_files, track_touched_files};
use crate::session::{
    bucket_for_messages, delegation_skills, derive_with_policy, effective_policy,
};

/// Execute a prompt with optional image attachments and stream events to the
/// provided channel.
///
/// See [`Session::prompt_with_events_and_images`](super::super::Session::prompt_with_events_and_images)
/// for the public-facing contract.
pub(crate) async fn run_prompt_with_events(
    session: &mut Session,
    message: &str,
    images: Vec<ImageAttachment>,
    event_tx: mpsc::Sender<SessionEvent>,
    registry: Arc<ProviderRegistry>,
) -> Result<SessionResult> {
    let _ = event_tx.send(SessionEvent::Thinking).await;
    session.resolve_subcall_provider(&registry);

    let providers = registry.list();
    if providers.is_empty() {
        anyhow::bail!(
            "No providers available. Configure provider credentials in HashiCorp Vault (for ChatGPT subscription Codex use `codetether auth codex`; for Copilot use `codetether auth copilot`)."
        );
    }
    tracing::info!("Available providers: {:?}", providers);

    let (provider_name, model_id) = parse_session_model_selector(session, &providers);

    let mut selected_provider =
        resolve_provider_for_session_request(providers.as_slice(), provider_name.as_deref())?
            .to_string();

    let mut provider = registry
        .get(&selected_provider)
        .ok_or_else(|| anyhow::anyhow!("Provider {} not found", selected_provider.clone()))?;

    let mut content_parts = vec![ContentPart::Text {
        text: message.to_string(),
    }];
    for img in &images {
        content_parts.push(ContentPart::Image {
            url: img.data_url.clone(),
            mime_type: img.mime_type.clone(),
        });
    }
    if !images.is_empty() {
        tracing::info!(
            image_count = images.len(),
            "Adding {} image attachment(s) to user message",
            images.len()
        );
    }

    session.add_message(Message {
        role: Role::User,
        content: content_parts,
    });

    if session.title.is_none() {
        session.generate_ai_title(&registry).await?;
    }

    let mut model = if !model_id.is_empty() {
        model_id
    } else {
        default_model_for_provider(&selected_provider)
    };

    let cwd = session
        .metadata
        .directory
        .clone()
        .unwrap_or_else(|| std::env::current_dir().unwrap_or_default());
    // Phase A: oversized-user-message compression now happens inside
    // `derive_context` on a clone. Keeping the original text in
    // `session.messages` means `session_recall` and the MinIO history
    // sink can still see what the user actually typed.
    let mut provider_state =
        build_provider_step_state(Arc::clone(&provider), &selected_provider, &model, &cwd);
    let mut tool_registry = provider_state.tool_registry.clone();
    let mut tool_definitions = provider_state.tool_definitions.clone();
    let mut temperature = provider_state.temperature;
    let mut model_supports_tools = provider_state.model_supports_tools;
    let mut advertised_tool_definitions = provider_state.advertised_tool_definitions.clone();
    let mut system_prompt = provider_state.system_prompt.clone();

    tracing::info!("Using model: {} via provider: {}", model, selected_provider);
    tracing::info!("Available tools: {}", tool_definitions.len());

    let mut final_output = String::new();
    let max_steps = session.max_steps.unwrap_or(DEFAULT_MAX_STEPS);
    let baseline_git_dirty_files = capture_git_dirty_files(&cwd).await;
    let mut touched_files = HashSet::new();
    let mut validation_retry_count: u8 = 0;

    let mut last_tool_sig: Option<String> = None;
    let mut consecutive_same_tool: u32 = 0;
    let mut repeat_guard = super::repeat_guard::RepeatGuard::default();
    let mut consecutive_codesearch_no_matches: u32 = 0;
    let mut build_mode_tool_retry_count: u8 = 0;
    let mut native_tool_promise_retry_count: u8 = 0;
    let mut total_tool_calls: u32 = 0;
    let mut steps_since_last_write: u32 = 0;
    let turn_id = Uuid::new_v4().to_string();

    let tool_router: Option<ToolCallRouter> = {
        let cfg = ToolRouterConfig::from_env();
        match ToolCallRouter::from_config(&cfg) {
            Ok(r) => r,
            Err(e) => {
                tracing::warn!(error = %e, "FunctionGemma tool router init failed; disabled");
                None
            }
        }
    };

    for step in 1..=max_steps {
        tracing::info!(step = step, "Agent step starting");
        let _ = event_tx.send(SessionEvent::Thinking).await;

        super::cost_guard::enforce_cost_budget()?;

        // Phase A: derive the per-step LLM context from a clone of
        // `session.messages` rather than mutating history in place.
        // Experimental strategies, RLM-powered context-window
        // enforcement, and the orphan-pair safety net all run against
        // the clone; the canonical transcript stays append-only.
        let policy = effective_policy(session, &model);
        let mut derived = derive_with_policy(
            session,
            Arc::clone(&provider),
            &model,
            &system_prompt,
            &advertised_tool_definitions,
            Some(&event_tx),
            policy,
            None,
        )
        .await?;

        let mut proactive_lsp_message = build_proactive_lsp_context_message(
            selected_provider.as_str(),
            step,
            &tool_registry,
            &session.messages,
            &cwd,
        )
        .await;
        let bucket = bucket_for_messages(session.history());

        let llm_start = std::time::Instant::now();
        let mut attempt = 0;
        let mut upstream_retry_count: u8 = 0;
        const MAX_UPSTREAM_RETRIES: u8 = 3;
        #[allow(clippy::never_loop)]
        let response = loop {
            attempt += 1;
            let mut messages = vec![Message {
                role: Role::System,
                content: vec![ContentPart::Text {
                    text: system_prompt.clone(),
                }],
            }];
            if let Some(msg) = &proactive_lsp_message {
                messages.push(msg.clone());
            }
            super::rlm_background::resolve_pending(&mut derived.messages);
            messages.extend(derived.messages.clone());
            let request = CompletionRequest {
                messages,
                tools: advertised_tool_definitions.clone(),
                model: model.clone(),
                temperature,
                top_p: None,
                max_tokens: Some(session_completion_max_tokens()),
                stop: Vec::new(),
            };
            let completion_result = super::prompt_call::complete_step(
                &provider,
                request,
                model_supports_tools,
                Some(&event_tx),
            )
            .await;

            match completion_result {
                Ok(r) => {
                    session.metadata.delegation.update(
                        &selected_provider,
                        delegation_skills::MODEL_CALL,
                        bucket,
                        true,
                    );
                    break r;
                }
                Err(e) => {
                    if let Some(keep_last) = super::prompt_too_long::keep_last(&e, attempt) {
                        tracing::warn!(error = %e, keep_last, "Provider rejected prompt as too long; re-deriving with RLM-forced compaction and retrying");
                        derived = derive_with_policy(
                            session,
                            Arc::clone(&provider),
                            &model,
                            &system_prompt,
                            &advertised_tool_definitions,
                            Some(&event_tx),
                            policy,
                            Some(keep_last),
                        )
                        .await?;
                        continue;
                    }
                    if upstream_retry_count < MAX_UPSTREAM_RETRIES
                        && is_retryable_upstream_error(&e)
                    {
                        session.metadata.delegation.update(
                            &selected_provider,
                            delegation_skills::MODEL_CALL,
                            bucket,
                            false,
                        );
                        upstream_retry_count += 1;
                        let backoff_secs = 1u64 << (upstream_retry_count - 1).min(2);
                        tracing::warn!(
                            error = %e,
                            retry = upstream_retry_count,
                            max = MAX_UPSTREAM_RETRIES,
                            backoff_secs,
                            "Retryable upstream provider error; sleeping and retrying"
                        );
                        tokio::time::sleep(std::time::Duration::from_secs(backoff_secs)).await;
                        if let Some((retry_provider, retry_model)) = choose_router_target_bandit(
                            &registry,
                            &session.metadata.delegation,
                            bucket,
                            &selected_provider,
                            &model,
                        ) {
                            tracing::info!(
                                to_provider = %retry_provider,
                                to_model = %retry_model,
                                "Failing over to alternate provider/model"
                            );
                            selected_provider = retry_provider;
                            provider = registry.get(&selected_provider).ok_or_else(|| {
                                anyhow::anyhow!("Provider {} not found", selected_provider.clone())
                            })?;
                            model = retry_model;
                            provider_state = build_provider_step_state(
                                Arc::clone(&provider),
                                &selected_provider,
                                &model,
                                &cwd,
                            );
                            tool_registry = provider_state.tool_registry.clone();
                            tool_definitions = provider_state.tool_definitions.clone();
                            temperature = provider_state.temperature;
                            model_supports_tools = provider_state.model_supports_tools;
                            advertised_tool_definitions =
                                provider_state.advertised_tool_definitions.clone();
                            system_prompt = provider_state.system_prompt.clone();
                            derived = derive_with_policy(
                                session,
                                Arc::clone(&provider),
                                &model,
                                &system_prompt,
                                &advertised_tool_definitions,
                                Some(&event_tx),
                                policy,
                                None,
                            )
                            .await?;
                            proactive_lsp_message = build_proactive_lsp_context_message(
                                selected_provider.as_str(),
                                step,
                                &tool_registry,
                                &session.messages,
                                &cwd,
                            )
                            .await;
                            session.metadata.model = Some(format!("{selected_provider}/{model}"));
                            attempt = 0;
                        }
                        continue;
                    }
                    return Err(e);
                }
            }
        };
        let llm_duration_ms = llm_start.elapsed().as_millis() as u64;

        let response = if let Some(ref router) = tool_router {
            router
                .maybe_reformat(response, &tool_definitions, model_supports_tools)
                .await
        } else {
            response
        };
        let mut response = normalize_textual_tool_calls(response, &tool_definitions);

        crate::telemetry::TOKEN_USAGE.record_model_usage_with_cache(
            &model,
            response.usage.prompt_tokens as u64,
            response.usage.completion_tokens as u64,
            response.usage.cache_read_tokens.unwrap_or(0) as u64,
            response.usage.cache_write_tokens.unwrap_or(0) as u64,
        );

        let _ = event_tx
            .send(SessionEvent::UsageReport {
                prompt_tokens: response.usage.prompt_tokens,
                completion_tokens: response.usage.completion_tokens,
                duration_ms: llm_duration_ms,
                model: model.clone(),
            })
            .await;

        let (mut tool_calls, truncated_tool_ids) =
            super::tool_call_parse::parse_tool_calls(&response.message.content);

        let assistant_text = extract_text_content(&&response.message.content);
        if should_force_build_tool_first_retry(
            &session.agent,
            build_mode_tool_retry_count,
            &tool_definitions,
            &session.messages,
            &cwd,
            &assistant_text,
            !tool_calls.is_empty(),
            BUILD_MODE_TOOL_FIRST_MAX_RETRIES,
        ) {
            build_mode_tool_retry_count += 1;
            tracing::warn!(
                step = step,
                agent = %session.agent,
                retry = build_mode_tool_retry_count,
                "Build mode tool-first guard triggered; retrying with execution nudge"
            );
            session.add_message(Message {
                role: Role::User,
                content: vec![ContentPart::Text {
                    text: BUILD_MODE_TOOL_FIRST_NUDGE.to_string(),
                }],
            });
            continue;
        }
        super::tool_extraction::salvage_prose_tool_call(
            &selected_provider,
            &model,
            &assistant_text,
            &tool_definitions,
            &truncated_tool_ids,
            &mut response,
            &mut tool_calls,
        );
        if should_retry_missing_native_tool_call(
            selected_provider.as_str(),
            &model,
            native_tool_promise_retry_count,
            &tool_definitions,
            &assistant_text,
            !tool_calls.is_empty(),
            NATIVE_TOOL_PROMISE_RETRY_MAX_RETRIES,
        ) {
            native_tool_promise_retry_count += 1;
            tracing::warn!(
                step = step,
                provider = selected_provider,
                model = %model,
                retry = native_tool_promise_retry_count,
                "Model described a tool step without emitting a tool call; retrying with corrective nudge"
            );
            session.add_message(response.message.clone());
            session.add_message(Message {
                role: Role::User,
                content: vec![ContentPart::Text {
                    text: NATIVE_TOOL_PROMISE_NUDGE.to_string(),
                }],
            });
            continue;
        }
        if !tool_calls.is_empty() {
            build_mode_tool_retry_count = 0;
            native_tool_promise_retry_count = 0;
        } else if is_build_agent(&session.agent)
            && build_request_requires_tool(&session.messages, &cwd)
            && build_mode_tool_retry_count >= BUILD_MODE_TOOL_FIRST_MAX_RETRIES
        {
            return Err(anyhow::anyhow!(
                "Build mode could not obtain tool calls for an explicit file-change request after {} retries. \
                 Switch to a tool-capable model and try again.",
                BUILD_MODE_TOOL_FIRST_MAX_RETRIES
            ));
        }

        let mut thinking_text = String::new();
        let mut step_text = String::new();
        for part in &response.message.content {
            match part {
                ContentPart::Thinking { text, .. } => {
                    if !text.is_empty() {
                        thinking_text.push_str(text);
                        thinking_text.push('\n');
                    }
                }
                ContentPart::Text { text } => {
                    if !text.is_empty() {
                        step_text.push_str(text);
                        step_text.push('\n');
                    }
                }
                _ => {}
            }
        }

        if !thinking_text.trim().is_empty() {
            let _ = event_tx
                .send(SessionEvent::ThinkingComplete(
                    thinking_text.trim().to_string(),
                ))
                .await;
            if let Some(ref bus) = session.bus {
                let handle = bus.handle(&session.agent);
                handle.send_with_correlation(
                    format!("agent.{}.thinking", session.agent),
                    crate::bus::BusMessage::AgentThinking {
                        agent_id: session.agent.clone(),
                        thinking: super::live_bus::compact_thinking(thinking_text.trim()),
                        step,
                    },
                    Some(turn_id.clone()),
                );
            }
        }

        if !step_text.trim().is_empty() {
            let trimmed = step_text.trim().to_string();
            let _ = event_tx
                .send(SessionEvent::TextChunk(trimmed.clone()))
                .await;
            let _ = event_tx.send(SessionEvent::TextComplete(trimmed)).await;
            final_output.push_str(&step_text);
        }

        if tool_calls.is_empty() && truncated_tool_ids.is_empty() {
            session.add_message(response.message.clone());
            if is_build_agent(&session.agent) {
                if let Some(report) =
                    build_validation_report(&cwd, &touched_files, &baseline_git_dirty_files).await?
                {
                    validation_retry_count += 1;
                    tracing::warn!(
                        retries = validation_retry_count,
                        issues = report.issue_count,
                        "Post-edit validation found unresolved diagnostics"
                    );
                    if validation_retry_count >= POST_EDIT_VALIDATION_MAX_RETRIES {
                        return Err(anyhow::anyhow!(
                            "Post-edit validation failed after {} attempts.\n\n{}",
                            POST_EDIT_VALIDATION_MAX_RETRIES,
                            report.prompt
                        ));
                    }
                    session.add_message(Message {
                        role: Role::User,
                        content: vec![ContentPart::Text {
                            text: report.prompt,
                        }],
                    });
                    final_output.clear();
                    continue;
                }
            }
            break;
        }

        if !truncated_tool_ids.is_empty() {
            if tool_calls.is_empty() {
                session.add_message(response.message.clone());
            }
            for (tool_id, tool_name) in &truncated_tool_ids {
                let error_content = super::tool_truncation::error_content(tool_name);
                events::complete(
                    &event_tx,
                    tool_id,
                    tool_name,
                    error_content.clone(),
                    false,
                    0,
                )
                .await;
                session.add_message(super::tool_output::tool_result_with_status(
                    tool_id.clone(),
                    tool_name,
                    false,
                    error_content,
                ));
            }
            if tool_calls.is_empty() {
                session.add_message(super::tool_truncation::retry_prompt(&truncated_tool_ids));
                final_output.clear();
                continue;
            }
        }

        {
            let mut sigs: Vec<String> = tool_calls
                .iter()
                .map(|(_, name, args)| format!("{name}:{args}"))
                .collect();
            sigs.sort();
            let sig = sigs.join("|");

            if last_tool_sig.as_deref() == Some(&sig) {
                consecutive_same_tool += 1;
            } else {
                consecutive_same_tool = 1;
                last_tool_sig = Some(sig);
            }

            let force_answer = consecutive_same_tool > MAX_CONSECUTIVE_SAME_TOOL
                || (!model_supports_tools && step >= 3);

            if force_answer {
                tracing::warn!(
                    step = step,
                    consecutive = consecutive_same_tool,
                    "Breaking agent loop: forcing final answer",
                );
                let mut nudge_msg = response.message.clone();
                nudge_msg
                    .content
                    .retain(|p| !matches!(p, ContentPart::ToolCall { .. }));
                if !nudge_msg.content.is_empty() {
                    session.add_message(nudge_msg);
                }
                session.add_message(Message {
                    role: Role::User,
                    content: vec![ContentPart::Text {
                        text: FORCE_FINAL_ANSWER_NUDGE.to_string(),
                    }],
                });
                continue;
            }
        }

        // ── Total tool call budget ──────────────────────────────
        total_tool_calls += tool_calls.len() as u32;
        if total_tool_calls > MAX_TOTAL_TOOL_CALLS {
            tracing::warn!(
                step = step,
                total_tool_calls,
                budget = MAX_TOTAL_TOOL_CALLS,
                "Hard tool-call budget exceeded; terminating agent loop"
            );
            let mut nudge_msg = response.message.clone();
            nudge_msg
                .content
                .retain(|p| !matches!(p, ContentPart::ToolCall { .. }));
            if !nudge_msg.content.is_empty() {
                session.add_message(nudge_msg);
            }
            return Err(anyhow::anyhow!(
                "Agent loop terminated: exceeded maximum tool call budget ({} calls across {} steps).                  The model appears stuck. Review the tool history to understand what went wrong.",
                total_tool_calls,
                step
            ));
        }

        // ── Progress detection (no file writes in N steps) ──────
        let any_write_tool = tool_calls.iter().any(|(_, name, _)| {
            matches!(
                name.as_str(),
                "write" | "edit" | "create_file" | "replace_string_in_file" | "edit_file" | "bash"
            )
        });
        if any_write_tool {
            steps_since_last_write = 0;
        } else {
            steps_since_last_write += 1;
        }
        if steps_since_last_write >= MAX_STEPS_WITHOUT_PROGRESS {
            tracing::warn!(
                step = step,
                steps_since_last_write,
                "No file-writing tools called recently; nudging toward progress or answer"
            );
            session.add_message(Message {
                role: Role::User,
                content: vec![ContentPart::Text {
                    text: NO_PROGRESS_NUDGE.to_string(),
                }],
            });
            steps_since_last_write = 0; // reset so we don't spam the nudge
        }

        session.add_message(response.message.clone());

        tracing::info!(
            step = step,
            num_tools = tool_calls.len(),
            total_tool_calls,
            "Executing tool calls"
        );

        let mut codesearch_thrash_guard_triggered = false;
        if super::tool_parallel::try_execute(
            session,
            &tool_calls,
            &tool_registry,
            &cwd,
            &model,
            Arc::clone(&provider),
            &event_tx,
            &mut consecutive_codesearch_no_matches,
        )
        .await
        {
            continue;
        }
        for (tool_id, tool_name, tool_input) in tool_calls {
            let (tool_name, tool_input) =
                normalize_tool_call_for_execution(&tool_name, &tool_input);
            let args_str = serde_json::to_string(&tool_input).unwrap_or_default();
            events::start(
                &event_tx,
                &tool_id,
                &tool_name,
                super::event_payload::bounded_tool_arguments(&args_str),
            )
            .await;

            tracing::info!(tool = %tool_name, tool_id = %tool_id, "Executing tool");

            if tool_name == "list_tools" {
                let content = list_tools_bootstrap_output(&tool_definitions, &tool_input);
                events::complete(&event_tx, &tool_id, &tool_name, content.clone(), true, 0).await;
                session.add_message(super::tool_output::tool_result_with_status(
                    tool_id, &tool_name, true, content,
                ));
                continue;
            }

            if let Some(ref bus) = session.bus {
                let handle = bus.handle(&session.agent);
                handle.send_with_correlation(
                    format!("agent.{}.tool.request", session.agent),
                    crate::bus::BusMessage::ToolRequest {
                        request_id: tool_id.clone(),
                        agent_id: session.agent.clone(),
                        tool_name: tool_name.clone(),
                        arguments: tool_input.clone(),
                        step,
                    },
                    Some(turn_id.clone()),
                );
            }

            if is_interactive_tool(&tool_name) {
                tracing::warn!(tool = %tool_name, "Blocking interactive tool in session loop");
                let content = "Error: Interactive tool 'question' is disabled in this interface. Ask the user directly in assistant text.".to_string();
                events::complete(&event_tx, &tool_id, &tool_name, content.clone(), false, 0).await;
                session.add_message(super::tool_output::tool_result_with_status(
                    tool_id, &tool_name, false, content,
                ));
                continue;
            }

            if let Some(reason) = detect_stub_in_tool_input(&tool_name, &tool_input)
                .or_else(|| repeat_guard.check(&tool_name, &tool_input))
            {
                tracing::warn!(tool = %tool_name, reason = %reason, "Pre-exec guard blocked tool call");
                let content = format!("Error: {reason}");
                events::complete(&event_tx, &tool_id, &tool_name, content.clone(), false, 0).await;
                session.add_message(super::tool_output::tool_result_with_status(
                    tool_id, &tool_name, false, content,
                ));
                continue;
            }

            let exec_input = enrich_tool_input_with_runtime_context(
                &tool_input,
                &cwd,
                session.metadata.model.as_deref(),
                &session.id,
                &session.agent,
                session.metadata.provenance.as_ref(),
            );
            let (exec_input, blocked_result) =
                super::tool_approval::gate(&event_tx, &tool_id, &tool_name, exec_input)
                    .await
                    .into_parts();
            let exec_start = super::persist::before_tool(session).await;
            let (content, success, tool_metadata) = match blocked_result {
                Some(blocked) => blocked,
                None => {
                    let hb =
                        super::tool_heartbeat::spawn(&event_tx, &tool_id, &tool_name, exec_start);
                    let result = execute_tool(
                        &tool_registry,
                        &tool_name,
                        &exec_input,
                        &session.id,
                        exec_start,
                        Some((&event_tx, &tool_id)),
                    )
                    .await;
                    hb.abort();
                    result
                }
            };

            let requires_confirmation = tool_result_requires_confirmation(tool_metadata.as_ref());
            let (content, success, tool_metadata, requires_confirmation) = if requires_confirmation
                && crate::tool::auto_apply::auto_apply_enabled(&session.metadata)
            {
                let preview_content = content.clone();
                match auto_apply_pending_confirmation(
                    &tool_name,
                    &exec_input,
                    tool_metadata.as_ref(),
                )
                .await
                {
                    Ok(Some((content, success, tool_metadata))) => {
                        tracing::info!(
                            tool = %tool_name,
                            "Auto-applied pending confirmation in TUI session"
                        );
                        (content, success, tool_metadata, false)
                    }
                    Ok(None) => (content, success, tool_metadata, true),
                    Err(error) => (
                        format!(
                            "{}\n\nTUI edit auto-apply failed: {}",
                            pending_confirmation_tool_result_content(&tool_name, &preview_content,),
                            error
                        ),
                        false,
                        tool_metadata,
                        true,
                    ),
                }
            } else {
                (content, success, tool_metadata, requires_confirmation)
            };
            let rendered_content = if requires_confirmation {
                pending_confirmation_tool_result_content(&tool_name, &content)
            } else {
                content.clone()
            };

            if !requires_confirmation {
                track_touched_files(
                    &mut touched_files,
                    &cwd,
                    &tool_name,
                    &tool_input,
                    tool_metadata.as_ref(),
                );
            }

            let duration_ms = exec_start.elapsed().as_millis() as u64;
            let codesearch_no_match =
                is_codesearch_no_match_output(&tool_name, success, &rendered_content);

            if let Some(ref bus) = session.bus {
                let handle = bus.handle(&session.agent);
                handle.send_with_correlation(
                    format!("agent.{}.tool.response", session.agent),
                    crate::bus::BusMessage::ToolResponse {
                        request_id: tool_id.clone(),
                        agent_id: session.agent.clone(),
                        tool_name: tool_name.clone(),
                        result: super::live_bus::compact_tool(&rendered_content),
                        success,
                        step,
                    },
                    Some(turn_id.clone()),
                );
                handle.send_with_correlation(
                    format!("agent.{}.tool.output", session.agent),
                    crate::bus::BusMessage::ToolOutputFull {
                        agent_id: session.agent.clone(),
                        tool_name: tool_name.clone(),
                        output: super::live_bus::compact_tool(&rendered_content),
                        success,
                        step,
                    },
                    Some(turn_id.clone()),
                );
            }

            if let Some(base_dir) = super::archive::event_stream_path() {
                write_tool_event_file(
                    &base_dir,
                    &session.id,
                    &tool_name,
                    success,
                    duration_ms,
                    &rendered_content,
                    session.messages.len() as u64,
                );
            }

            super::tool_metadata_event::send(
                &event_tx,
                &tool_id,
                &tool_name,
                tool_metadata.as_ref(),
            )
            .await;
            events::complete(
                &event_tx,
                &tool_id,
                &tool_name,
                super::event_payload::bounded_tool_output(&rendered_content),
                success,
                duration_ms,
            )
            .await;

            let content = maybe_route_through_rlm(
                &rendered_content,
                &tool_name,
                &tool_input,
                &tool_id,
                &session.id,
                &session.messages,
                &model,
                Arc::clone(&provider),
                &session.metadata.rlm,
                Some(rlm_notify(event_tx.clone())),
            );

            session.add_message(super::tool_output::tool_result_with_status(
                tool_id, &tool_name, success, content,
            ));

            if is_build_agent(&session.agent) {
                if codesearch_no_match {
                    consecutive_codesearch_no_matches += 1;
                } else {
                    consecutive_codesearch_no_matches = 0;
                }

                if consecutive_codesearch_no_matches >= MAX_CONSECUTIVE_CODESEARCH_NO_MATCHES {
                    tracing::warn!(
                        step = step,
                        consecutive_codesearch_no_matches = consecutive_codesearch_no_matches,
                        "Detected codesearch no-match thrash; nudging model to stop variant retries",
                    );
                    session.add_message(Message {
                        role: Role::User,
                        content: vec![ContentPart::Text {
                            text: CODESEARCH_THRASH_NUDGE.to_string(),
                        }],
                    });
                    codesearch_thrash_guard_triggered = true;
                    break;
                }
            }
        }

        if codesearch_thrash_guard_triggered {
            continue;
        }
    }

    session.save().await?;

    super::archive::archive_event_stream_to_s3(&session.id, super::archive::event_stream_path())
        .await;

    let _ = event_tx.send(SessionEvent::Done).await;

    Ok(SessionResult {
        text: super::evidence::gate_final_answer(final_output.trim(), session),
        session_id: session.id.clone(),
    })
}

/// Split the session's configured model string into `(provider, model_id)`.
fn parse_session_model_selector(session: &Session, providers: &[&str]) -> (Option<String>, String) {
    let Some(ref model_str) = session.metadata.model else {
        return (None, String::new());
    };
    let (prov, model) = parse_model_string(model_str);
    let prov = prov.map(|p| match p {
        "zhipuai" | "z-ai" => "zai",
        other => other,
    });
    if prov.is_some() {
        (prov.map(|s| s.to_string()), model.to_string())
    } else if providers.contains(&model) {
        (Some(model.to_string()), String::new())
    } else {
        (None, model.to_string())
    }
}

pub(super) use super::tool_exec::execute_tool;

/// Write a [`ChatEvent::tool_result`] JSONL record to disk (fire-and-forget).
fn write_tool_event_file(
    base_dir: &std::path::Path,
    session_id: &str,
    tool_name: &str,
    success: bool,
    duration_ms: u64,
    rendered_content: &str,
    seq: u64,
) {
    let workspace = std::env::var("PWD")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|_| std::env::current_dir().unwrap_or_default());
    let event = ChatEvent::tool_result(
        workspace,
        session_id.to_string(),
        tool_name,
        success,
        duration_ms,
        rendered_content,
        seq,
    );
    let event_json = event.to_json();
    let event_size = event_json.len() as u64 + 1;
    let timestamp = Utc::now().format("%Y%m%dT%H%M%SZ");
    let filename = format!(
        "{}-chat-events-{:020}-{:020}.jsonl",
        timestamp,
        seq * 10000,
        (seq + 1) * 10000
    );
    let event_path = base_dir.join(session_id).join(filename);

    tokio::spawn(async move {
        if let Some(parent) = event_path.parent() {
            let _ = tokio::fs::create_dir_all(parent).await;
        }
        if let Ok(mut file) = tokio::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&event_path)
            .await
        {
            use tokio::io::AsyncWriteExt;
            let _ = file.write_all(event_json.as_bytes()).await;
            let _ = file.write_all(b"\n").await;
            tracing::debug!(path = %event_path.display(), size = event_size, "Event stream wrote");
        }
    });
}

/// Route a large tool output through the Recursive Language Model when it
/// exceeds the routing heuristics.
fn maybe_route_through_rlm(
    rendered_content: &str,
    tool_name: &str,
    tool_input: &serde_json::Value,
    tool_id: &str,
    session_id: &str,
    messages: &[Message],
    model: &str,
    provider: Arc<dyn crate::provider::Provider>,
    rlm_config: &RlmConfig,
    notify: Option<super::rlm_background::Notify>,
) -> String {
    super::rlm_background::route_or_defer(
        rendered_content,
        tool_name,
        tool_input,
        tool_id,
        session_id,
        messages,
        model,
        provider,
        rlm_config,
        notify,
    )
}

fn rlm_notify(tx: mpsc::Sender<SessionEvent>) -> super::rlm_background::Notify {
    Arc::new(move |event| {
        let _ = tx.try_send(event);
    })
}