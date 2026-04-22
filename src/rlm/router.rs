//! RLM Router - Decides when to route content through RLM processing
//!
//! Routes large tool outputs through RLM when they would exceed
//! the model's context window threshold.
//!
//! When the `functiongemma` feature is active the router passes RLM tool
//! definitions alongside the analysis prompt so FunctionGemma can convert
//! text-only LLM responses into structured tool calls.

use super::{RlmChunker, RlmConfig, RlmResult, RlmStats};
use crate::provider::{CompletionRequest, ContentPart, Message, Provider, Role};
use crate::rlm::context_trace::{ContextEvent, ContextTrace};
use crate::session::{RlmCompletion, RlmOutcome, RlmProgressEvent, SessionBus, SessionEvent};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::sync::Arc;
use std::time::Instant;
use tracing::{info, warn};
use uuid::Uuid;

use crate::cognition::tool_router::{ToolCallRouter, ToolRouterConfig};

use super::tools::rlm_tool_definitions;

/// Tools eligible for RLM routing
fn rlm_eligible_tools() -> HashSet<&'static str> {
    // Only tools whose output is genuinely unpredictable in size AND that the
    // agent cannot easily re-query in smaller chunks belong here. Routing
    // replaces raw bytes with an RLM-generated prose summary, which is
    // destructive for content-carrying tools like `read`/`grep`/`bash` — the
    // agent has no way to get the original bytes back and re-reads produce
    // the same summary, causing spin loops. Those tools already expose
    // bounded slicing (offset/limit, line ranges, head/tail).
    [
        // Web tools commonly return megabytes of HTML/JS with no slicing API.
        "webfetch",
        "websearch",
        // Batch aggregates many outputs and can exceed the window in one shot.
        "batch",
    ]
    .iter()
    .copied()
    .collect()
}

/// Context for routing decisions
#[derive(Debug, Clone)]
pub struct RoutingContext {
    pub tool_id: String,
    pub session_id: String,
    pub call_id: Option<String>,
    pub model_context_limit: usize,
    pub current_context_tokens: Option<usize>,
}

/// Result of routing decision
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoutingResult {
    pub should_route: bool,
    pub reason: String,
    pub estimated_tokens: usize,
}

/// Context for auto-processing.
///
/// ## Observability model
///
/// Two orthogonal streams describe an RLM run:
///
/// 1. **[`crate::rlm::context_trace::ContextTrace`] (durable):** the full,
///    ordered record of every iteration, tool call, and model response.
///    Attached to the returned [`RlmResult`] so the JSONL flywheel and
///    post-hoc analysis can replay a run exactly.
/// 2. **Session bus events (ephemeral):** [`SessionEvent::RlmProgress`]
///    at every iteration start and a single terminal
///    [`SessionEvent::RlmComplete`] at exit. Subscribers see the run as
///    it happens but are *not* expected to reconstruct full history
///    from the bus — for that, consume the trace on `RlmResult`.
///
/// ## Opting out of observability
///
/// Leaving `bus` and `trace_id` as `None` opts the call site out of
/// **both** bus emission and upstream trace correlation. This is
/// correct for tests, one-shot CLI runs, and synchronous utility
/// paths, but production call sites should thread an `AppState`-owned
/// bus through so the TUI and audit log see the activity.
pub struct AutoProcessContext<'a> {
    /// Identifier of the tool whose output is being analysed (e.g.
    /// `"grep"`, `"bash"`, or `"session_context"` for compaction).
    pub tool_id: &'a str,
    /// JSON-encoded arguments that produced the tool output.
    pub tool_args: serde_json::Value,
    /// Session ID for logging / correlation.
    pub session_id: &'a str,
    /// Optional abort signal — when `Some(true)` the loop exits early
    /// and emits [`RlmOutcome::Aborted`].
    pub abort: Option<tokio::sync::watch::Receiver<bool>>,
    /// Optional progress callback. Retained for backward compatibility;
    /// prefer subscribing to [`SessionEvent::RlmProgress`] via `bus`.
    pub on_progress: Option<Box<dyn Fn(ProcessProgress) + Send + Sync>>,
    /// Provider used for the root RLM model call.
    pub provider: Arc<dyn Provider>,
    /// Model identifier for the root RLM call.
    pub model: String,
    /// Optional event bus for structured progress / completion events.
    /// See the struct-level docs for the bus-vs-trace split.
    pub bus: Option<SessionBus>,
    /// Optional caller-supplied trace id so upstream events (e.g. a
    /// compaction `CompactionStarted`) can be correlated with the
    /// resulting RLM run. When `None`, a fresh id is generated and
    /// returned on [`RlmResult::trace_id`].
    pub trace_id: Option<Uuid>,
    /// Pre-resolved provider for RLM sub-calls (iterations ≥ 2).
    ///
    /// When `None`, all iterations use [`Self::provider`]. When set,
    /// iteration 1 uses the root provider and subsequent iterations
    /// use this provider, enabling "expensive model for analysis,
    /// cheap model for continuation" splitting.
    ///
    /// Resolution (and fallback-on-failure) is the caller's
    /// responsibility — see
    /// [`Session::resolve_subcall_provider`](crate::session::Session).
    pub subcall_provider: Option<Arc<dyn Provider>>,
    /// Model identifier for the subcall provider. Mirrors
    /// [`Self::subcall_provider`]: when `None`, all iterations use
    /// [`Self::model`].
    pub subcall_model: Option<String>,
}

/// Progress update during processing
#[derive(Debug, Clone)]
pub struct ProcessProgress {
    pub iteration: usize,
    pub max_iterations: usize,
    pub status: String,
}

/// RLM Router for large content processing
pub struct RlmRouter;

impl RlmRouter {
    /// Check if a tool output should be routed through RLM
    pub fn should_route(output: &str, ctx: &RoutingContext, config: &RlmConfig) -> RoutingResult {
        let estimated_tokens = RlmChunker::estimate_tokens(output);

        // Mode: off - never route
        if config.mode == "off" {
            return RoutingResult {
                should_route: false,
                reason: "rlm_mode_off".to_string(),
                estimated_tokens,
            };
        }

        // Mode: always - always route for eligible tools
        if config.mode == "always" {
            if !rlm_eligible_tools().contains(ctx.tool_id.as_str()) {
                return RoutingResult {
                    should_route: false,
                    reason: "tool_not_eligible".to_string(),
                    estimated_tokens,
                };
            }
            return RoutingResult {
                should_route: true,
                reason: "rlm_mode_always".to_string(),
                estimated_tokens,
            };
        }

        // Mode: auto - route based on threshold
        let eligible = rlm_eligible_tools().contains(ctx.tool_id.as_str())
            || ctx.tool_id.as_str() == "session_context";

        // Check if output exceeds threshold relative to context window
        let threshold_tokens = (ctx.model_context_limit as f64 * config.threshold) as usize;
        if estimated_tokens > threshold_tokens {
            return RoutingResult {
                should_route: true,
                reason: if eligible {
                    "exceeds_threshold".to_string()
                } else {
                    "exceeds_threshold_ineligible_tool".to_string()
                },
                estimated_tokens,
            };
        }

        // Check if adding this output would cause overflow.
        //
        // Historically this branch also fired based on `current_context_tokens`
        // alone, which silently replaced small, content-carrying tool outputs
        // (e.g. a 2 KB `read`) with an RLM prose summary whenever the prior
        // conversation happened to be large. The agent had no way to
        // distinguish the summary from the file's bytes and would re-read the
        // same path, hitting the same branch, producing the same summary —
        // an unbreakable spin loop. Context-window pressure is the
        // compression pass's responsibility, not the tool-output router's.
        //
        // We now only route on "would overflow" when the *new output itself*
        // is responsible for a large share of the projected total. This
        // preserves protection against a single oversized blob while leaving
        // normal-sized tool results verbatim.
        if let Some(current) = ctx.current_context_tokens {
            let projected_total = current + estimated_tokens;
            let overflow_limit = (ctx.model_context_limit as f64 * 0.8) as usize;
            // Require the new output to contribute at least half of the
            // overflow so we don't punish small reads for old context.
            let output_dominates = estimated_tokens * 2 >= projected_total;
            if projected_total > overflow_limit && output_dominates {
                return RoutingResult {
                    should_route: true,
                    reason: if eligible {
                        "would_overflow".to_string()
                    } else {
                        "would_overflow_ineligible_tool".to_string()
                    },
                    estimated_tokens,
                };
            }
        }

        // If the tool isn't in the preferred set, we normally avoid routing to
        // reduce extra model calls. However, we *must* protect the main session
        // context window from oversized blobs (e.g. raw HTML from web tools).
        if !eligible {
            return RoutingResult {
                should_route: false,
                reason: "tool_not_eligible".to_string(),
                estimated_tokens,
            };
        }

        RoutingResult {
            should_route: false,
            reason: "within_threshold".to_string(),
            estimated_tokens,
        }
    }

    /// Smart truncate large output with RLM hint
    pub fn smart_truncate(
        output: &str,
        tool_id: &str,
        tool_args: &serde_json::Value,
        max_tokens: usize,
    ) -> (String, bool, usize) {
        let estimated_tokens = RlmChunker::estimate_tokens(output);

        if estimated_tokens <= max_tokens {
            return (output.to_string(), false, estimated_tokens);
        }

        info!(
            tool = tool_id,
            original_tokens = estimated_tokens,
            max_tokens,
            "Smart truncating large output"
        );

        // Calculate how much to keep (roughly 4 chars per token)
        let max_chars = max_tokens * 4;
        let head_chars = (max_chars as f64 * 0.6) as usize;
        let tail_chars = (max_chars as f64 * 0.3) as usize;

        let head: String = output.chars().take(head_chars).collect();
        let tail: String = output
            .chars()
            .rev()
            .take(tail_chars)
            .collect::<String>()
            .chars()
            .rev()
            .collect();

        let omitted_tokens = estimated_tokens
            - RlmChunker::estimate_tokens(&head)
            - RlmChunker::estimate_tokens(&tail);
        let rlm_hint = Self::build_rlm_hint(tool_id, tool_args, estimated_tokens);

        let truncated = format!(
            "{}\n\n[... {} tokens truncated ...]\n\n{}\n\n{}",
            head, omitted_tokens, rlm_hint, tail
        );

        (truncated, true, estimated_tokens)
    }

    fn build_rlm_hint(tool_id: &str, args: &serde_json::Value, tokens: usize) -> String {
        let base = format!(
            "⚠️ OUTPUT TOO LARGE ({} tokens). Use RLM for full analysis:",
            tokens
        );

        match tool_id {
            "read" => {
                let path = args
                    .get("filePath")
                    .and_then(|v| v.as_str())
                    .unwrap_or("...");
                format!(
                    "{}\n```\nrlm({{ query: \"Analyze this file\", content_paths: [\"{}\"] }})\n```",
                    base, path
                )
            }
            "bash" => {
                format!(
                    "{}\n```\nrlm({{ query: \"Analyze this command output\", content: \"<paste or use content_paths>\" }})\n```",
                    base
                )
            }
            "grep" => {
                let pattern = args
                    .get("pattern")
                    .and_then(|v| v.as_str())
                    .unwrap_or("...");
                let include = args.get("include").and_then(|v| v.as_str()).unwrap_or("*");
                format!(
                    "{}\n```\nrlm({{ query: \"Summarize search results for {}\", content_glob: \"{}\" }})\n```",
                    base, pattern, include
                )
            }
            _ => {
                format!(
                    "{}\n```\nrlm({{ query: \"Summarize this output\", content: \"...\" }})\n```",
                    base
                )
            }
        }
    }

    /// Automatically process large output through RLM
    ///
    /// Based on "Recursive Language Models" (Zhang et al. 2025):
    /// - Context is loaded as a variable in a REPL-like environment
    /// - LLM writes code/queries to analyze, decompose, and recursively sub-call itself
    ///
    /// When FunctionGemma is enabled, the router sends RLM tool definitions
    /// alongside the analysis prompt and dispatches structured tool calls
    /// returned by the model (or reformatted by FunctionGemma).
    pub async fn auto_process(
        output: &str,
        ctx: AutoProcessContext<'_>,
        config: &RlmConfig,
    ) -> Result<RlmResult> {
        let start = Instant::now();
        let input_tokens = RlmChunker::estimate_tokens(output);

        info!(
            tool = ctx.tool_id,
            input_tokens,
            model = %ctx.model,
            "RLM: Starting auto-processing"
        );

        // Per-run trace id — correlates every progress event and the
        // terminal RlmComplete emission with any caller-side record
        // (e.g. a CompactionStarted event).
        let trace_id = ctx.trace_id.unwrap_or_else(Uuid::new_v4);
        // Generous budget — the trace is diagnostic, not load-bearing.
        let trace_budget = input_tokens.saturating_mul(2).max(4096);
        let mut trace = ContextTrace::new(trace_budget);
        let mut aborted = false;

        // Initialise FunctionGemma router if available

        let tool_router: Option<ToolCallRouter> = {
            let cfg = ToolRouterConfig::from_env();
            ToolCallRouter::from_config(&cfg)
                .inspect_err(|e| {
                    tracing::debug!(error = %e, "FunctionGemma router unavailable for RLM router");
                })
                .ok()
                .flatten()
        };

        // Prepare RLM tool definitions
        let tools = rlm_tool_definitions();

        // Detect content type for smarter processing
        let content_type = RlmChunker::detect_content_type(output);
        let content_hints = RlmChunker::get_processing_hints(content_type);

        info!(content_type = ?content_type, tool = ctx.tool_id, "RLM: Content type detected");

        // For very large contexts, use semantic chunking to preserve important parts
        let processed_output = if input_tokens > 50000 {
            RlmChunker::compress(output, 40000, None)
        } else {
            output.to_string()
        };

        // Create a REPL for structured tool dispatch
        let mut repl =
            super::repl::RlmRepl::new(processed_output.clone(), super::repl::ReplRuntime::Rust);

        // Build the query based on tool type
        let base_query = Self::build_query_for_tool(ctx.tool_id, &ctx.tool_args);
        let query = format!(
            "{}\n\n## Content Analysis Hints\n{}",
            base_query, content_hints
        );

        // Build the RLM system prompt
        let system_prompt = Self::build_rlm_system_prompt(input_tokens, ctx.tool_id, &query);
        trace.log_event(ContextEvent::SystemPrompt {
            content: system_prompt.clone(),
            tokens: ContextTrace::estimate_tokens(&system_prompt),
        });

        let max_iterations = config.max_iterations;
        let max_subcalls = config.max_subcalls;
        let mut iterations = 0;
        let mut subcalls = 0;
        let mut final_answer: Option<String> = None;

        // Build initial exploration prompt
        let exploration = Self::build_exploration_summary(&processed_output, input_tokens);

        // Run iterative analysis
        let mut conversation = vec![Message {
            role: Role::User,
            content: vec![ContentPart::Text {
                text: format!(
                    "{}\n\nHere is the context exploration:\n```\n{}\n```\n\nNow analyze and answer the query.",
                    system_prompt, exploration
                ),
            }],
        }];

        for i in 0..max_iterations {
            iterations = i + 1;
            trace.next_iteration();

            if let Some(bus) = ctx.bus.as_ref() {
                bus.emit(SessionEvent::RlmProgress(RlmProgressEvent {
                    trace_id,
                    iteration: iterations,
                    max_iterations,
                    status: "running".to_string(),
                }));
            }

            if let Some(ref progress) = ctx.on_progress {
                progress(ProcessProgress {
                    iteration: iterations,
                    max_iterations,
                    status: "running".to_string(),
                });
            }

            // Check for abort
            if let Some(ref abort) = ctx.abort
                && *abort.borrow()
            {
                warn!("RLM: Processing aborted");
                aborted = true;
                break;
            }

            // Build completion request — include tool definitions
            let (active_provider, active_model) =
                if iterations > 1 && ctx.subcall_provider.is_some() {
                    (
                        Arc::clone(ctx.subcall_provider.as_ref().unwrap()),
                        ctx.subcall_model
                            .as_deref()
                            .unwrap_or(&ctx.model)
                            .to_string(),
                    )
                } else {
                    (Arc::clone(&ctx.provider), ctx.model.clone())
                };

            let request = CompletionRequest {
                messages: conversation.clone(),
                tools: tools.clone(),
                model: active_model.clone(),
                temperature: Some(0.7),
                top_p: None,
                max_tokens: Some(4000),
                stop: Vec::new(),
            };

            // Call the model
            let response = match active_provider.complete(request).await {
                Ok(r) => r,
                Err(e) => {
                    warn!(error = %e, iteration = iterations, "RLM: Model call failed");
                    if iterations > 1 {
                        break; // Use what we have
                    }
                    if let Some(bus) = ctx.bus.as_ref() {
                        bus.emit(SessionEvent::RlmComplete(RlmCompletion {
                            trace_id,
                            outcome: RlmOutcome::Failed,
                            iterations,
                            subcalls,
                            input_tokens,
                            output_tokens: 0,
                            elapsed_ms: start.elapsed().as_millis() as u64,
                            reason: Some(format!("provider call failed: {e}")),
                            root_model: ctx.model.clone(),
                            subcall_model_used: ctx.subcall_model.clone(),
                        }));
                    }
                    let mut fb =
                        Self::fallback_result(output, ctx.tool_id, &ctx.tool_args, input_tokens);
                    fb.trace = Some(trace);
                    fb.trace_id = Some(trace_id);
                    return Ok(fb);
                }
            };

            // Optionally run FunctionGemma to convert text-only response

            let response = if let Some(ref router) = tool_router {
                // RLM router shares the session's provider which supports
                // native tool calling.  Skip FunctionGemma.
                router.maybe_reformat(response, &tools, true).await
            } else {
                response
            };

            // ── Structured tool-call path ────────────────────────────────
            let tool_calls: Vec<(String, String, String)> = response
                .message
                .content
                .iter()
                .filter_map(|p| match p {
                    ContentPart::ToolCall {
                        id,
                        name,
                        arguments,
                        ..
                    } => Some((id.clone(), name.clone(), arguments.clone())),
                    _ => None,
                })
                .collect();

            if !tool_calls.is_empty() {
                info!(
                    count = tool_calls.len(),
                    iteration = iterations,
                    "RLM router: dispatching structured tool calls"
                );

                conversation.push(Message {
                    role: Role::Assistant,
                    content: response.message.content.clone(),
                });

                let mut tool_results: Vec<ContentPart> = Vec::new();

                for (call_id, name, arguments) in &tool_calls {
                    trace.log_event(ContextEvent::ToolCall {
                        name: name.clone(),
                        arguments_preview: arguments.chars().take(200).collect(),
                        tokens: ContextTrace::estimate_tokens(arguments),
                    });
                    match super::tools::dispatch_tool_call(name, arguments, &mut repl) {
                        Some(super::tools::RlmToolResult::Final(answer)) => {
                            trace.log_event(ContextEvent::ToolResult {
                                tool_call_id: call_id.clone(),
                                result_preview: "FINAL received".to_string(),
                                tokens: 0,
                            });
                            final_answer = Some(answer);
                            tool_results.push(ContentPart::ToolResult {
                                tool_call_id: call_id.clone(),
                                content: "FINAL received".to_string(),
                            });
                            break;
                        }
                        Some(super::tools::RlmToolResult::Output(out)) => {
                            trace.log_event(ContextEvent::ToolResult {
                                tool_call_id: call_id.clone(),
                                result_preview: out.chars().take(200).collect(),
                                tokens: ContextTrace::estimate_tokens(&out),
                            });
                            tool_results.push(ContentPart::ToolResult {
                                tool_call_id: call_id.clone(),
                                content: out,
                            });
                        }
                        None => {
                            trace.log_event(ContextEvent::ToolResult {
                                tool_call_id: call_id.clone(),
                                result_preview: format!("Unknown tool: {name}"),
                                tokens: 0,
                            });
                            tool_results.push(ContentPart::ToolResult {
                                tool_call_id: call_id.clone(),
                                content: format!("Unknown tool: {name}"),
                            });
                        }
                    }
                }

                if !tool_results.is_empty() {
                    conversation.push(Message {
                        role: Role::Tool,
                        content: tool_results,
                    });
                }

                subcalls += 1;
                if final_answer.is_some() || subcalls >= max_subcalls {
                    break;
                }
                continue;
            }

            // ── Legacy text-only path ────────────────────────────────────
            let response_text: String = response
                .message
                .content
                .iter()
                .filter_map(|p| match p {
                    ContentPart::Text { text } => Some(text.clone()),
                    _ => None,
                })
                .collect::<Vec<_>>()
                .join("\n");

            info!(
                iteration = iterations,
                response_len = response_text.len(),
                "RLM: Model response (text-only fallback)"
            );
            trace.log_event(ContextEvent::LlmQueryResult {
                query: query.chars().take(120).collect(),
                response_preview: response_text.chars().take(200).collect(),
                tokens: ContextTrace::estimate_tokens(&response_text),
            });

            // Check for FINAL answer
            if let Some(answer) = Self::extract_final(&response_text) {
                final_answer = Some(answer);
                break;
            }

            // Check for analysis that can be used directly
            if iterations >= 3 && response_text.len() > 500 && !response_text.contains("```") {
                // The model is providing direct analysis, use it
                final_answer = Some(response_text.clone());
                break;
            }

            // Add response to conversation
            conversation.push(Message {
                role: Role::Assistant,
                content: vec![ContentPart::Text {
                    text: response_text,
                }],
            });

            // Prompt for continuation
            conversation.push(Message {
                role: Role::User,
                content: vec![ContentPart::Text {
                    text: "Continue analysis. Call FINAL(\"your answer\") when ready.".to_string(),
                }],
            });

            subcalls += 1;
            if subcalls >= max_subcalls {
                warn!(subcalls, max = max_subcalls, "RLM: Max subcalls reached");
                break;
            }
        }

        if let Some(ref progress) = ctx.on_progress {
            progress(ProcessProgress {
                iteration: iterations,
                max_iterations,
                status: "completed".to_string(),
            });
        }

        // Track whether we got a real answer before consuming final_answer.
        let converged = final_answer.is_some();

        // Produce result, synthesizing one if no FINAL was produced.
        let answer = final_answer.unwrap_or_else(|| {
            warn!(
                iterations,
                subcalls, "RLM: No FINAL produced, using fallback"
            );
            Self::build_enhanced_fallback(output, ctx.tool_id, &ctx.tool_args, input_tokens)
        });

        let output_tokens = RlmChunker::estimate_tokens(&answer);
        let compression_ratio = input_tokens as f64 / output_tokens.max(1) as f64;
        let elapsed_ms = start.elapsed().as_millis() as u64;

        let result = format!(
            "[RLM: {} → {} tokens | {} iterations | {} sub-calls]\n\n{}",
            input_tokens, output_tokens, iterations, subcalls, answer
        );

        info!(
            input_tokens,
            output_tokens,
            iterations,
            subcalls,
            elapsed_ms,
            compression_ratio = format!("{:.1}", compression_ratio),
            "RLM: Processing complete"
        );

        trace.log_event(ContextEvent::Final {
            answer: answer.chars().take(200).collect(),
            tokens: output_tokens,
        });

        // Classify outcome for the durable completion record.
        let outcome = if aborted {
            RlmOutcome::Aborted
        } else if converged {
            RlmOutcome::Converged
        } else {
            RlmOutcome::Exhausted
        };
        let reason = match outcome {
            RlmOutcome::Converged => None,
            RlmOutcome::Aborted => Some("abort signal".to_string()),
            RlmOutcome::Exhausted => Some(format!(
                "no FINAL after {iterations} iterations / {subcalls} subcalls"
            )),
            RlmOutcome::Failed => None, // emitted on the early-return path
        };
        if let Some(bus) = ctx.bus.as_ref() {
            bus.emit(SessionEvent::RlmComplete(RlmCompletion {
                trace_id,
                outcome,
                iterations,
                subcalls,
                input_tokens,
                output_tokens,
                elapsed_ms,
                reason,
                root_model: ctx.model.clone(),
                subcall_model_used: ctx.subcall_model.clone(),
            }));
        }

        Ok(RlmResult {
            processed: result,
            stats: RlmStats {
                input_tokens,
                output_tokens,
                iterations,
                subcalls,
                elapsed_ms,
                compression_ratio,
            },
            success: outcome.is_success(),
            error: None,
            trace: Some(trace),
            trace_id: Some(trace_id),
        })
    }

    fn extract_final(text: &str) -> Option<String> {
        // Look for FINAL("...") or FINAL('...') or FINAL!(...)
        let patterns = [r#"FINAL\s*\(\s*["'`]"#, r#"FINAL!\s*\(\s*["'`]?"#];

        for _pattern_start in patterns {
            if let Some(start_idx) = text.find("FINAL") {
                let after = &text[start_idx..];

                // Find the opening quote/paren
                if let Some(open_idx) = after.find(['"', '\'', '`']) {
                    let quote_char = after.chars().nth(open_idx)?;
                    let content_start = start_idx + open_idx + 1;

                    // Find matching close
                    let content = &text[content_start..];
                    if let Some(close_idx) = content.find(quote_char) {
                        let answer = &content[..close_idx];
                        if !answer.is_empty() {
                            return Some(answer.to_string());
                        }
                    }
                }
            }
        }

        None
    }

    fn build_exploration_summary(content: &str, input_tokens: usize) -> String {
        let lines: Vec<&str> = content.lines().collect();
        let total_lines = lines.len();

        let head: String = lines
            .iter()
            .take(30)
            .copied()
            .collect::<Vec<_>>()
            .join("\n");
        let tail: String = lines
            .iter()
            .rev()
            .take(50)
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .copied()
            .collect::<Vec<_>>()
            .join("\n");

        format!(
            "=== CONTEXT EXPLORATION ===\n\
             Total: {} chars, {} lines, ~{} tokens\n\n\
             === FIRST 30 LINES ===\n{}\n\n\
             === LAST 50 LINES ===\n{}\n\
             === END EXPLORATION ===",
            content.len(),
            total_lines,
            input_tokens,
            head,
            tail
        )
    }

    fn build_rlm_system_prompt(input_tokens: usize, tool_id: &str, query: &str) -> String {
        let context_type = if tool_id == "session_context" {
            "conversation history"
        } else {
            "tool output"
        };

        format!(
            r#"You are tasked with analyzing large content that cannot fit in a normal context window.

The content is a {} with {} total tokens.

YOUR TASK: {}

## Analysis Strategy

1. First, examine the exploration (head + tail of content) to understand structure
2. Identify the most important information for answering the query
3. Focus on: errors, key decisions, file paths, recent activity
4. Provide a concise but complete answer

When ready, call FINAL("your detailed answer") with your findings.

Be SPECIFIC - include actual file paths, function names, error messages. Generic summaries are not useful."#,
            context_type, input_tokens, query
        )
    }

    fn build_query_for_tool(tool_id: &str, args: &serde_json::Value) -> String {
        match tool_id {
            "read" => {
                let path = args.get("filePath").and_then(|v| v.as_str()).unwrap_or("unknown");
                format!("Summarize the key contents of file \"{}\". Focus on: structure, main functions/classes, important logic. Be concise.", path)
            }
            "bash" => {
                "Summarize the command output. Extract key information, results, errors, warnings. Be concise.".to_string()
            }
            "grep" => {
                let pattern = args.get("pattern").and_then(|v| v.as_str()).unwrap_or("pattern");
                format!("Summarize search results for \"{}\". Group by file, highlight most relevant matches. Be concise.", pattern)
            }
            "glob" => {
                "Summarize the file listing. Group by directory, highlight important files. Be concise.".to_string()
            }
            "webfetch" => {
                let url = args.get("url").and_then(|v| v.as_str()).unwrap_or("unknown");
                format!(
                    "Extract and summarize the main human-readable content from this fetched web page (URL: {}). Ignore scripts, styles, navigation, cookie banners, and boilerplate. Preserve headings, lists, code blocks, and important links. Be concise.",
                    url
                )
            }
            "websearch" => {
                let q = args.get("query").and_then(|v| v.as_str()).unwrap_or("unknown");
                format!(
                    "Summarize these web search results for query: \"{}\". List the most relevant results with titles and URLs, and note why each is relevant. Be concise.",
                    q
                )
            }
            "batch" => {
                "Summarize the batch tool output. Group by sub-call, highlight failures/errors first, then key results. Keep actionable details: URLs, file paths, command outputs, and next steps.".to_string()
            }
            "session_context" => {
                r#"You are a CONTEXT MEMORY SYSTEM. Create a BRIEFING for an AI assistant to continue this conversation.

CRITICAL: The assistant will ONLY see your briefing - it has NO memory of the conversation.

## What to Extract

1. **PRIMARY GOAL**: What is the user ultimately trying to achieve?
2. **CURRENT STATE**: What has been accomplished? Current status?
3. **LAST ACTIONS**: What just happened? (last 3-5 tool calls, their results)
4. **ACTIVE FILES**: Which files were modified?
5. **PENDING TASKS**: What remains to be done?
6. **CRITICAL DETAILS**: File paths, error messages, specific values, decisions made
7. **NEXT STEPS**: What should happen next?

Be SPECIFIC with file paths, function names, error messages."#.to_string()
            }
            _ => "Summarize this output concisely, extracting the most important information.".to_string()
        }
    }

    fn build_enhanced_fallback(
        output: &str,
        tool_id: &str,
        tool_args: &serde_json::Value,
        input_tokens: usize,
    ) -> String {
        let lines: Vec<&str> = output.lines().collect();

        if tool_id == "session_context" {
            // Extract key structural information
            let file_matches: Vec<&str> = lines
                .iter()
                .filter_map(|l| {
                    if l.contains(".ts")
                        || l.contains(".rs")
                        || l.contains(".py")
                        || l.contains(".json")
                    {
                        Some(*l)
                    } else {
                        None
                    }
                })
                .take(15)
                .collect();

            let tool_calls: Vec<&str> = lines
                .iter()
                .filter(|l| l.contains("[Tool "))
                .take(10)
                .copied()
                .collect();

            let errors: Vec<&str> = lines
                .iter()
                .filter(|l| {
                    l.to_lowercase().contains("error") || l.to_lowercase().contains("failed")
                })
                .take(5)
                .copied()
                .collect();

            let head: String = lines
                .iter()
                .take(30)
                .copied()
                .collect::<Vec<_>>()
                .join("\n");
            let tail: String = lines
                .iter()
                .rev()
                .take(80)
                .collect::<Vec<_>>()
                .into_iter()
                .rev()
                .copied()
                .collect::<Vec<_>>()
                .join("\n");

            let mut parts = vec![
                "## Context Summary (Fallback Mode)".to_string(),
                format!(
                    "*Original: {} tokens - RLM processing produced insufficient output*",
                    input_tokens
                ),
                String::new(),
            ];

            if !file_matches.is_empty() {
                parts.push(format!("**Files Mentioned:** {}", file_matches.len()));
            }

            if !tool_calls.is_empty() {
                parts.push(format!("**Recent Tool Calls:** {}", tool_calls.join(", ")));
            }

            if !errors.is_empty() {
                parts.push("**Recent Errors:**".to_string());
                for e in errors {
                    parts.push(format!("- {}", e.chars().take(150).collect::<String>()));
                }
            }

            parts.push(String::new());
            parts.push("### Initial Request".to_string());
            parts.push("```".to_string());
            parts.push(head);
            parts.push("```".to_string());
            parts.push(String::new());
            parts.push("### Recent Activity".to_string());
            parts.push("```".to_string());
            parts.push(tail);
            parts.push("```".to_string());

            parts.join("\n")
        } else {
            let (truncated, _, _) = Self::smart_truncate(output, tool_id, tool_args, 8000);
            format!(
                "## Fallback Summary\n*RLM processing failed - showing structured excerpt*\n\n{}",
                truncated
            )
        }
    }

    fn fallback_result(
        output: &str,
        tool_id: &str,
        tool_args: &serde_json::Value,
        input_tokens: usize,
    ) -> RlmResult {
        let (truncated, _, _) = Self::smart_truncate(output, tool_id, tool_args, 8000);
        let output_tokens = RlmChunker::estimate_tokens(&truncated);

        RlmResult {
            processed: format!(
                "[RLM processing failed, showing truncated output]\n\n{}",
                truncated
            ),
            stats: RlmStats {
                input_tokens,
                output_tokens,
                iterations: 0,
                subcalls: 0,
                elapsed_ms: 0,
                compression_ratio: input_tokens as f64 / output_tokens.max(1) as f64,
            },
            success: false,
            error: Some("Model call failed".to_string()),
            trace: None,
            trace_id: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_route_large_webfetch_output() {
        let output = "a".repeat(200_000);
        let ctx = RoutingContext {
            tool_id: "webfetch".to_string(),
            session_id: "s".to_string(),
            call_id: None,
            model_context_limit: 200_000,
            current_context_tokens: Some(1000),
        };
        let cfg = RlmConfig::default();
        let result = RlmRouter::should_route(&output, &ctx, &cfg);
        assert!(result.should_route);
    }

    #[test]
    fn should_route_large_output_for_unknown_tool_to_protect_context() {
        let output = "a".repeat(200_000);
        let ctx = RoutingContext {
            tool_id: "some_new_tool".to_string(),
            session_id: "s".to_string(),
            call_id: None,
            model_context_limit: 200_000,
            current_context_tokens: Some(1000),
        };
        let cfg = RlmConfig::default();
        let result = RlmRouter::should_route(&output, &ctx, &cfg);
        assert!(result.should_route);
        assert!(result.reason.contains("ineligible") || result.reason.contains("threshold"));
    }
}
