//! Core agentic prompt loop used by [`Session::prompt`](super::super::Session::prompt).
//!
//! This is the non-streaming, non-event-emitting variant. It loads the
//! provider registry from Vault, runs the completion/tool-call loop up to
//! `max_steps` times, and returns the final plain-text answer.
//!
//! The code here was extracted from `src/session/mod.rs` verbatim to keep
//! the Session facade small and keep the loop's behavior stable. Refinements
//! to the loop should be made here (and mirrored in [`prompt_events`](super::prompt_events)
//! where appropriate).

use std::collections::HashSet;
use std::sync::Arc;

use anyhow::Result;
use chrono::Utc;
use serde_json::json;

use crate::audit::{AuditCategory, AuditOutcome, try_audit_log};
use crate::cognition::tool_router::{ToolCallRouter, ToolRouterConfig};
use crate::event_stream::ChatEvent;
use crate::provider::{
    CompletionRequest, ContentPart, Message, ProviderRegistry, Role, parse_model_string,
};
use crate::rlm::router::AutoProcessContext;
use crate::rlm::{RlmChunker, RlmConfig, RlmRouter, RoutingContext};
use crate::tool::ToolRegistry;

use super::super::{DEFAULT_MAX_STEPS, Session, SessionResult};
use super::bootstrap::{
    inject_tool_prompt, list_tools_bootstrap_definition, list_tools_bootstrap_output,
};
use super::build::{
    build_request_requires_tool, is_build_agent, should_force_build_tool_first_retry,
};
use super::compression::{compress_history_keep_last, enforce_context_window};
use super::confirmation::{
    auto_apply_pending_confirmation, pending_confirmation_tool_result_content,
    tool_result_requires_confirmation,
};
use super::defaults::default_model_for_provider;
use super::edit::{detect_stub_in_tool_input, normalize_tool_call_for_execution};
use super::error::{is_prompt_too_long_error, is_retryable_upstream_error};
use super::loop_constants::{
    BUILD_MODE_TOOL_FIRST_MAX_RETRIES, BUILD_MODE_TOOL_FIRST_NUDGE, CODESEARCH_THRASH_NUDGE,
    FORCE_FINAL_ANSWER_NUDGE, MAX_CONSECUTIVE_CODESEARCH_NO_MATCHES, MAX_CONSECUTIVE_SAME_TOOL,
    NATIVE_TOOL_PROMISE_NUDGE, NATIVE_TOOL_PROMISE_RETRY_MAX_RETRIES,
    POST_EDIT_VALIDATION_MAX_RETRIES,
};
use super::markup::normalize_textual_tool_calls;
use super::provider::{
    prefers_temperature_one, resolve_provider_for_session_request,
    should_retry_missing_native_tool_call, temperature_is_deprecated,
};
use super::router::{build_proactive_lsp_context_message, choose_router_target};
use super::runtime::{
    enrich_tool_input_with_runtime_context, is_codesearch_no_match_output, is_interactive_tool,
    is_local_cuda_provider, local_cuda_light_system_prompt,
};
use super::text::extract_text_content;
use super::token::{
    context_window_for_model, estimate_tokens_for_messages, session_completion_max_tokens,
};
use super::validation::{build_validation_report, capture_git_dirty_files, track_touched_files};

/// Execute a prompt against the session and return a [`SessionResult`].
///
/// See [`Session::prompt`](super::super::Session::prompt) for the public-facing contract.
pub(crate) async fn run_prompt(session: &mut Session, message: &str) -> Result<SessionResult> {
    let registry = ProviderRegistry::from_vault().await?;

    let providers = registry.list();
    if providers.is_empty() {
        anyhow::bail!(
            "No providers available. Configure provider credentials in HashiCorp Vault (for ChatGPT subscription Codex use `codetether auth codex`; for Copilot use `codetether auth copilot`)."
        );
    }

    tracing::info!("Available providers: {:?}", providers);

    let (provider_name, model_id) = parse_session_model_selector(session, &providers);

    let selected_provider =
        resolve_provider_for_session_request(providers.as_slice(), provider_name.as_deref())?;

    let provider = registry
        .get(selected_provider)
        .ok_or_else(|| anyhow::anyhow!("Provider {} not found", selected_provider))?;

    session.add_message(Message {
        role: Role::User,
        content: vec![ContentPart::Text {
            text: message.to_string(),
        }],
    });

    if session.title.is_none() {
        session.generate_title().await?;
    }

    let model = if !model_id.is_empty() {
        model_id
    } else {
        default_model_for_provider(selected_provider)
    };

    compress_user_message_if_oversized(session, &provider, &model, message).await;

    let tool_registry = ToolRegistry::with_provider_arc(Arc::clone(&provider), model.clone());
    let tool_definitions: Vec<_> = tool_registry
        .definitions()
        .into_iter()
        .filter(|tool| !is_interactive_tool(&tool.name))
        .collect();

    let temperature = if temperature_is_deprecated(&model) {
        None
    } else if prefers_temperature_one(&model) {
        Some(1.0)
    } else {
        Some(0.7)
    };

    tracing::info!("Using model: {} via provider: {}", model, selected_provider);
    tracing::info!("Available tools: {}", tool_definitions.len());

    let model_supports_tools = !matches!(
        selected_provider,
        "gemini-web" | "local-cuda" | "local_cuda" | "localcuda"
    );
    let advertised_tool_definitions = if model_supports_tools {
        tool_definitions.clone()
    } else {
        vec![list_tools_bootstrap_definition()]
    };

    let cwd = session
        .metadata
        .directory
        .clone()
        .unwrap_or_else(|| std::env::current_dir().unwrap_or_default());
    let system_prompt = if is_local_cuda_provider(selected_provider) {
        local_cuda_light_system_prompt()
    } else {
        crate::agent::builtin::build_system_prompt(&cwd)
    };
    let system_prompt = if !model_supports_tools && !advertised_tool_definitions.is_empty() {
        inject_tool_prompt(&system_prompt, &advertised_tool_definitions)
    } else {
        system_prompt
    };

    let max_steps = session.max_steps.unwrap_or(DEFAULT_MAX_STEPS);
    let mut final_output = String::new();
    let baseline_git_dirty_files = capture_git_dirty_files(&cwd).await;
    let mut touched_files = HashSet::new();
    let mut validation_retry_count: u8 = 0;

    let mut last_tool_sig: Option<String> = None;
    let mut consecutive_same_tool: u32 = 0;
    let mut consecutive_codesearch_no_matches: u32 = 0;
    let mut build_mode_tool_retry_count: u8 = 0;
    let mut native_tool_promise_retry_count: u8 = 0;

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

        enforce_context_window(
            session,
            Arc::clone(&provider),
            &model,
            &system_prompt,
            &advertised_tool_definitions,
        )
        .await?;

        let proactive_lsp_message = build_proactive_lsp_context_message(
            selected_provider,
            step,
            &tool_registry,
            &session.messages,
            &cwd,
        )
        .await;

        let mut attempt = 0;
        let mut upstream_retry_count: u8 = 0;
        const MAX_UPSTREAM_RETRIES: u8 = 3;
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
            messages.extend(session.messages.clone());

            let request = CompletionRequest {
                messages,
                tools: advertised_tool_definitions.clone(),
                model: model.clone(),
                temperature,
                top_p: None,
                max_tokens: Some(session_completion_max_tokens()),
                stop: Vec::new(),
            };

            match provider.complete(request).await {
                Ok(r) => break r,
                Err(e) => {
                    if attempt == 1 && is_prompt_too_long_error(&e) {
                        tracing::warn!(error = %e, "Provider rejected prompt as too long; forcing extra compression and retrying");
                        let _ = compress_history_keep_last(
                            session,
                            Arc::clone(&provider),
                            &model,
                            6,
                            "prompt_too_long_retry",
                        )
                        .await?;
                        continue;
                    }
                    if upstream_retry_count < MAX_UPSTREAM_RETRIES
                        && is_retryable_upstream_error(&e)
                    {
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
                        if let Some((retry_provider, retry_model)) =
                            choose_router_target(&registry, selected_provider, &model)
                        {
                            tracing::info!(
                                to_provider = %retry_provider,
                                to_model = %retry_model,
                                "Failing over to alternate provider/model"
                            );
                            session.metadata.model =
                                Some(format!("{retry_provider}/{retry_model}"));
                            attempt = 0;
                        }
                        continue;
                    }
                    return Err(e);
                }
            }
        };

        let response = if let Some(ref router) = tool_router {
            router
                .maybe_reformat(response, &tool_definitions, model_supports_tools)
                .await
        } else {
            response
        };
        let response = normalize_textual_tool_calls(response, &tool_definitions);

        crate::telemetry::TOKEN_USAGE.record_model_usage(
            &model,
            response.usage.prompt_tokens as u64,
            response.usage.completion_tokens as u64,
        );

        let mut truncated_tool_ids: Vec<(String, String)> = Vec::new();
        let tool_calls: Vec<(String, String, serde_json::Value)> = response
            .message
            .content
            .iter()
            .filter_map(|part| {
                if let ContentPart::ToolCall {
                    id,
                    name,
                    arguments,
                    ..
                } = part
                {
                    match serde_json::from_str::<serde_json::Value>(arguments) {
                        Ok(args) => Some((id.clone(), name.clone(), args)),
                        Err(e) => {
                            tracing::warn!(
                                tool = %name,
                                tool_call_id = %id,
                                args_len = arguments.len(),
                                error = %e,
                                "Tool call arguments failed to parse (likely truncated by max_tokens)"
                            );
                            truncated_tool_ids.push((id.clone(), name.clone()));
                            None
                        }
                    }
                } else {
                    None
                }
            })
            .collect();

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
        if should_retry_missing_native_tool_call(
            selected_provider,
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

        let mut step_text = String::new();

        for part in &response.message.content {
            match part {
                ContentPart::Text { text } if !text.is_empty() => {
                    step_text.push_str(text);
                    step_text.push('\n');
                }
                ContentPart::Thinking { text } if !text.is_empty() => {
                    if let Some(ref bus) = session.bus {
                        let handle = bus.handle(&session.agent);
                        handle.send(
                            format!("agent.{}.thinking", session.agent),
                            crate::bus::BusMessage::AgentThinking {
                                agent_id: session.agent.clone(),
                                thinking: text.clone(),
                                step,
                            },
                        );
                    }
                }
                _ => {}
            }
        }

        if !step_text.trim().is_empty() {
            final_output.push_str(&step_text);
        }

        if tool_calls.is_empty() && truncated_tool_ids.is_empty() {
            session.add_message(response.message.clone());
            if is_build_agent(&session.agent) {
                if let Some(report) =
                    build_validation_report(&cwd, &touched_files, &baseline_git_dirty_files)
                        .await?
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
                let error_content = format!(
                    "Error: Your tool call to `{tool_name}` was truncated — the arguments \
                     JSON was cut off mid-string (likely hit the max_tokens limit). \
                     Please retry with a shorter approach: use the `write` tool to write \
                     content in smaller pieces, or reduce the size of your arguments."
                );
                session.add_message(Message {
                    role: Role::Tool,
                    content: vec![ContentPart::ToolResult {
                        tool_call_id: tool_id.clone(),
                        content: error_content,
                    }],
                });
            }
            if tool_calls.is_empty() {
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

        session.add_message(response.message.clone());

        tracing::info!(
            step = step,
            num_tools = tool_calls.len(),
            "Executing tool calls"
        );

        let mut codesearch_thrash_guard_triggered = false;
        for (tool_id, tool_name, tool_input) in tool_calls {
            let (tool_name, tool_input) =
                normalize_tool_call_for_execution(&tool_name, &tool_input);
            tracing::info!(tool = %tool_name, tool_id = %tool_id, "Executing tool");

            if tool_name == "list_tools" {
                let content = list_tools_bootstrap_output(&tool_definitions, &tool_input);
                session.add_message(Message {
                    role: Role::Tool,
                    content: vec![ContentPart::ToolResult {
                        tool_call_id: tool_id,
                        content,
                    }],
                });
                continue;
            }

            if let Some(ref bus) = session.bus {
                let handle = bus.handle(&session.agent);
                handle.send(
                    format!("agent.{}.tool.request", session.agent),
                    crate::bus::BusMessage::ToolRequest {
                        request_id: tool_id.clone(),
                        agent_id: session.agent.clone(),
                        tool_name: tool_name.clone(),
                        arguments: tool_input.clone(),
                        step,
                    },
                );
            }

            if is_interactive_tool(&tool_name) {
                tracing::warn!(tool = %tool_name, "Blocking interactive tool in session loop");
                session.add_message(Message {
                    role: Role::Tool,
                    content: vec![ContentPart::ToolResult {
                        tool_call_id: tool_id,
                        content: "Error: Interactive tool 'question' is disabled in this interface. Ask the user directly in assistant text.".to_string(),
                    }],
                });
                continue;
            }

            if let Some(reason) = detect_stub_in_tool_input(&tool_name, &tool_input) {
                tracing::warn!(tool = %tool_name, reason = %reason, "Blocking suspected stubbed edit");
                session.add_message(Message {
                    role: Role::Tool,
                    content: vec![ContentPart::ToolResult {
                        tool_call_id: tool_id,
                        content: format!(
                            "Error: Refactor guard rejected this edit: {reason}. \
                             Provide concrete, behavior-preserving implementation (no placeholders/stubs)."
                        ),
                    }],
                });
                continue;
            }

            let exec_start = std::time::Instant::now();
            let exec_input = enrich_tool_input_with_runtime_context(
                &tool_input,
                &cwd,
                session.metadata.model.as_deref(),
                &session.id,
                &session.agent,
                session.metadata.provenance.as_ref(),
            );
            let (content, success, tool_metadata) = execute_tool(
                &tool_registry,
                &tool_name,
                &exec_input,
                &session.id,
                exec_start,
            )
            .await;

            let requires_confirmation =
                tool_result_requires_confirmation(tool_metadata.as_ref());
            let (content, success, tool_metadata, requires_confirmation) =
                if requires_confirmation && session.metadata.auto_apply_edits {
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
                                pending_confirmation_tool_result_content(
                                    &tool_name,
                                    &preview_content,
                                ),
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
                handle.send(
                    format!("agent.{}.tool.response", session.agent),
                    crate::bus::BusMessage::ToolResponse {
                        request_id: tool_id.clone(),
                        agent_id: session.agent.clone(),
                        tool_name: tool_name.clone(),
                        result: rendered_content.clone(),
                        success,
                        step,
                    },
                );
                handle.send(
                    format!("agent.{}.tool.output", session.agent),
                    crate::bus::BusMessage::ToolOutputFull {
                        agent_id: session.agent.clone(),
                        tool_name: tool_name.clone(),
                        output: rendered_content.clone(),
                        success,
                        step,
                    },
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

            let content = maybe_route_through_rlm(
                &rendered_content,
                &tool_name,
                &tool_input,
                &tool_id,
                &session.id,
                &session.messages,
                &model,
                Arc::clone(&provider),
            )
            .await;

            session.add_message(Message {
                role: Role::Tool,
                content: vec![ContentPart::ToolResult {
                    tool_call_id: tool_id,
                    content,
                }],
            });

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

    super::archive::archive_event_stream_to_s3(
        &session.id,
        super::archive::event_stream_path(),
    )
    .await;

    Ok(SessionResult {
        text: final_output.trim().to_string(),
        session_id: session.id.clone(),
    })
}

/// Split the session's configured model string into `(provider, model_id)`.
fn parse_session_model_selector(
    session: &Session,
    providers: &[&str],
) -> (Option<String>, String) {
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

/// Apply RLM compression to the most recent user message when it blows past
/// the context-window heuristic threshold.
async fn compress_user_message_if_oversized(
    session: &mut Session,
    provider: &Arc<dyn crate::provider::Provider>,
    model: &str,
    message: &str,
) {
    let ctx_window = context_window_for_model(model);
    let msg_tokens = RlmChunker::estimate_tokens(message);
    let threshold = (ctx_window as f64 * 0.35) as usize;
    if msg_tokens <= threshold {
        return;
    }

    tracing::info!(
        msg_tokens,
        threshold,
        ctx_window,
        "RLM: User message exceeds context threshold, compressing"
    );

    let auto_ctx = AutoProcessContext {
        tool_id: "session_context",
        tool_args: serde_json::json!({}),
        session_id: &session.id,
        abort: None,
        on_progress: None,
        provider: Arc::clone(provider),
        model: model.to_string(),
    };
    let rlm_config = RlmConfig::default();
    match RlmRouter::auto_process(message, auto_ctx, &rlm_config).await {
        Ok(result) => {
            tracing::info!(
                input_tokens = result.stats.input_tokens,
                output_tokens = result.stats.output_tokens,
                "RLM: User message compressed"
            );
            if let Some(last) = session.messages.last_mut() {
                last.content = vec![ContentPart::Text {
                    text: format!(
                        "[Original message: {} tokens, compressed via RLM]\n\n{}\n\n---\nOriginal request prefix:\n{}",
                        msg_tokens,
                        result.processed,
                        message.chars().take(500).collect::<String>()
                    ),
                }];
            }
        }
        Err(e) => {
            tracing::warn!(error = %e, "RLM: Failed to compress user message, using truncation");
            let max_chars = threshold * 4;
            let truncated = RlmChunker::compress(message, max_chars / 4, None);
            if let Some(last) = session.messages.last_mut() {
                last.content = vec![ContentPart::Text { text: truncated }];
            }
        }
    }
}

/// Execute a tool call and emit the corresponding audit entry.
async fn execute_tool(
    tool_registry: &ToolRegistry,
    tool_name: &str,
    exec_input: &serde_json::Value,
    session_id: &str,
    exec_start: std::time::Instant,
) -> (
    String,
    bool,
    Option<std::collections::HashMap<String, serde_json::Value>>,
) {
    if let Some(tool) = tool_registry.get(tool_name) {
        match tool.execute(exec_input.clone()).await {
            Ok(result) => {
                let duration_ms = exec_start.elapsed().as_millis() as u64;
                tracing::info!(tool = %tool_name, success = result.success, "Tool execution completed");
                if let Some(audit) = try_audit_log() {
                    audit
                        .log_with_correlation(
                            AuditCategory::ToolExecution,
                            format!("tool:{}", tool_name),
                            if result.success {
                                AuditOutcome::Success
                            } else {
                                AuditOutcome::Failure
                            },
                            None,
                            Some(json!({
                                "duration_ms": duration_ms,
                                "output_len": result.output.len()
                            })),
                            None,
                            None,
                            None,
                            Some(session_id.to_string()),
                        )
                        .await;
                }
                (result.output, result.success, Some(result.metadata))
            }
            Err(e) => {
                let duration_ms = exec_start.elapsed().as_millis() as u64;
                tracing::warn!(tool = %tool_name, error = %e, "Tool execution failed");
                if let Some(audit) = try_audit_log() {
                    audit
                        .log_with_correlation(
                            AuditCategory::ToolExecution,
                            format!("tool:{}", tool_name),
                            AuditOutcome::Failure,
                            None,
                            Some(json!({ "duration_ms": duration_ms, "error": e.to_string() })),
                            None,
                            None,
                            None,
                            Some(session_id.to_string()),
                        )
                        .await;
                }
                (format!("Error: {}", e), false, None)
            }
        }
    } else {
        tracing::warn!(tool = %tool_name, "Tool not found");
        if let Some(audit) = try_audit_log() {
            audit
                .log_with_correlation(
                    AuditCategory::ToolExecution,
                    format!("tool:{}", tool_name),
                    AuditOutcome::Failure,
                    None,
                    Some(json!({ "error": "unknown_tool" })),
                    None,
                    None,
                    None,
                    Some(session_id.to_string()),
                )
                .await;
        }
        (format!("Error: Unknown tool '{}'", tool_name), false, None)
    }
}

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
        }
    });
}

/// Route a large tool output through the Recursive Language Model when it
/// exceeds the routing heuristics.
async fn maybe_route_through_rlm(
    rendered_content: &str,
    tool_name: &str,
    tool_input: &serde_json::Value,
    tool_id: &str,
    session_id: &str,
    messages: &[Message],
    model: &str,
    provider: Arc<dyn crate::provider::Provider>,
) -> String {
    let ctx_window = context_window_for_model(model);
    let current_tokens = estimate_tokens_for_messages(messages);
    let routing_ctx = RoutingContext {
        tool_id: tool_name.to_string(),
        session_id: session_id.to_string(),
        call_id: Some(tool_id.to_string()),
        model_context_limit: ctx_window,
        current_context_tokens: Some(current_tokens),
    };
    let rlm_config = RlmConfig::default();
    let routing = RlmRouter::should_route(rendered_content, &routing_ctx, &rlm_config);
    if !routing.should_route {
        return rendered_content.to_string();
    }

    tracing::info!(
        tool = %tool_name,
        reason = %routing.reason,
        estimated_tokens = routing.estimated_tokens,
        "RLM: Routing large tool output"
    );
    let auto_ctx = AutoProcessContext {
        tool_id: tool_name,
        tool_args: tool_input.clone(),
        session_id,
        abort: None,
        on_progress: None,
        provider: Arc::clone(&provider),
        model: model.to_string(),
    };
    match RlmRouter::auto_process(rendered_content, auto_ctx, &rlm_config).await {
        Ok(result) => {
            tracing::info!(
                input_tokens = result.stats.input_tokens,
                output_tokens = result.stats.output_tokens,
                iterations = result.stats.iterations,
                "RLM: Processing complete"
            );
            result.processed
        }
        Err(e) => {
            tracing::warn!(error = %e, "RLM: auto_process failed, using smart_truncate");
            let (truncated, _, _) = RlmRouter::smart_truncate(
                rendered_content,
                tool_name,
                tool_input,
                ctx_window / 4,
            );
            truncated
        }
    }
}
