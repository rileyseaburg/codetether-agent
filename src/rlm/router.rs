//! RLM Router - Decides when to route content through RLM processing
//!
//! Routes large tool outputs through RLM when they would exceed
//! the model's context window threshold.

use super::{RlmChunker, RlmConfig, RlmResult, RlmStats};
use crate::provider::{CompletionRequest, ContentPart, Message, Provider, Role};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::sync::Arc;
use std::time::Instant;
use tracing::{info, warn};

/// Tools eligible for RLM routing
fn rlm_eligible_tools() -> HashSet<&'static str> {
    ["read", "glob", "grep", "bash", "search"].iter().copied().collect()
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

/// Context for auto-processing
pub struct AutoProcessContext<'a> {
    pub tool_id: &'a str,
    pub tool_args: serde_json::Value,
    pub session_id: &'a str,
    pub abort: Option<tokio::sync::watch::Receiver<bool>>,
    pub on_progress: Option<Box<dyn Fn(ProcessProgress) + Send + Sync>>,
    pub provider: Arc<dyn Provider>,
    pub model: String,
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
        if !rlm_eligible_tools().contains(ctx.tool_id.as_str()) {
            return RoutingResult {
                should_route: false,
                reason: "tool_not_eligible".to_string(),
                estimated_tokens,
            };
        }

        // Check if output exceeds threshold relative to context window
        let threshold_tokens = (ctx.model_context_limit as f64 * config.threshold) as usize;
        if estimated_tokens > threshold_tokens {
            return RoutingResult {
                should_route: true,
                reason: "exceeds_threshold".to_string(),
                estimated_tokens,
            };
        }

        // Check if adding this output would cause overflow
        if let Some(current) = ctx.current_context_tokens {
            let projected_total = current + estimated_tokens;
            if projected_total > (ctx.model_context_limit as f64 * 0.8) as usize {
                return RoutingResult {
                    should_route: true,
                    reason: "would_overflow".to_string(),
                    estimated_tokens,
                };
            }
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
        let tail: String = output.chars().rev().take(tail_chars).collect::<String>().chars().rev().collect();

        let omitted_tokens = estimated_tokens - RlmChunker::estimate_tokens(&head) - RlmChunker::estimate_tokens(&tail);
        let rlm_hint = Self::build_rlm_hint(tool_id, tool_args, estimated_tokens);

        let truncated = format!(
            "{}\n\n[... {} tokens truncated ...]\n\n{}\n\n{}",
            head, omitted_tokens, rlm_hint, tail
        );

        (truncated, true, estimated_tokens)
    }

    fn build_rlm_hint(tool_id: &str, args: &serde_json::Value, tokens: usize) -> String {
        let base = format!("⚠️ OUTPUT TOO LARGE ({} tokens). Use RLM for full analysis:", tokens);

        match tool_id {
            "read" => {
                let path = args.get("filePath").and_then(|v| v.as_str()).unwrap_or("...");
                format!("{}\n```\nrlm({{ query: \"Analyze this file\", content_paths: [\"{}\"] }})\n```", base, path)
            }
            "bash" => {
                format!("{}\n```\nrlm({{ query: \"Analyze this command output\", content: \"<paste or use content_paths>\" }})\n```", base)
            }
            "grep" => {
                let pattern = args.get("pattern").and_then(|v| v.as_str()).unwrap_or("...");
                let include = args.get("include").and_then(|v| v.as_str()).unwrap_or("*");
                format!("{}\n```\nrlm({{ query: \"Summarize search results for {}\", content_glob: \"{}\" }})\n```", base, pattern, include)
            }
            _ => {
                format!("{}\n```\nrlm({{ query: \"Summarize this output\", content: \"...\" }})\n```", base)
            }
        }
    }

    /// Automatically process large output through RLM
    ///
    /// Based on "Recursive Language Models" (Zhang et al. 2025):
    /// - Context is loaded as a variable in a REPL-like environment
    /// - LLM writes code/queries to analyze, decompose, and recursively sub-call itself
    pub async fn auto_process(output: &str, ctx: AutoProcessContext<'_>, config: &RlmConfig) -> Result<RlmResult> {
        let start = Instant::now();
        let input_tokens = RlmChunker::estimate_tokens(output);

        info!(
            tool = ctx.tool_id,
            input_tokens,
            model = %ctx.model,
            "RLM: Starting auto-processing"
        );

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

        // Build the query based on tool type
        let base_query = Self::build_query_for_tool(ctx.tool_id, &ctx.tool_args);
        let query = format!("{}\n\n## Content Analysis Hints\n{}", base_query, content_hints);

        // Build the RLM system prompt
        let system_prompt = Self::build_rlm_system_prompt(input_tokens, ctx.tool_id, &query);

        let max_iterations = config.max_iterations;
        let max_subcalls = config.max_subcalls;
        let mut iterations = 0;
        let mut subcalls = 0;
        let mut final_answer: Option<String> = None;

        // Build initial exploration prompt
        let exploration = Self::build_exploration_summary(&processed_output, input_tokens);

        // Run iterative analysis
        let mut conversation = vec![
            Message {
                role: Role::User,
                content: vec![ContentPart::Text {
                    text: format!(
                        "{}\n\nHere is the context exploration:\n```\n{}\n```\n\nNow analyze and answer the query.",
                        system_prompt, exploration
                    ),
                }],
            },
        ];

        for i in 0..max_iterations {
            iterations = i + 1;

            if let Some(ref progress) = ctx.on_progress {
                progress(ProcessProgress {
                    iteration: iterations,
                    max_iterations,
                    status: "running".to_string(),
                });
            }

            // Check for abort
            if let Some(ref abort) = ctx.abort {
                if *abort.borrow() {
                    warn!("RLM: Processing aborted");
                    break;
                }
            }

            // Build completion request
            let request = CompletionRequest {
                messages: conversation.clone(),
                tools: Vec::new(),
                model: ctx.model.clone(),
                temperature: Some(0.7),
                top_p: None,
                max_tokens: Some(4000),
                stop: Vec::new(),
            };

            // Call the model
            let response = match ctx.provider.complete(request).await {
                Ok(r) => r,
                Err(e) => {
                    warn!(error = %e, iteration = iterations, "RLM: Model call failed");
                    if iterations > 1 {
                        break; // Use what we have
                    }
                    return Ok(Self::fallback_result(output, ctx.tool_id, &ctx.tool_args, input_tokens));
                }
            };

            let response_text: String = response.message.content
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
                "RLM: Model response"
            );

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
                content: vec![ContentPart::Text { text: response_text }],
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

        // Fallback if no FINAL was produced
        let answer = final_answer.unwrap_or_else(|| {
            warn!(iterations, subcalls, "RLM: No FINAL produced, using fallback");
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

        Ok(RlmResult {
            processed: result,
            stats: RlmStats {
                input_tokens,
                output_tokens: RlmChunker::estimate_tokens(&answer),
                iterations,
                subcalls,
                elapsed_ms,
                compression_ratio,
            },
            success: true,
            error: None,
        })
    }

    fn extract_final(text: &str) -> Option<String> {
        // Look for FINAL("...") or FINAL('...') or FINAL!(...)
        let patterns = [
            r#"FINAL\s*\(\s*["'`]"#,
            r#"FINAL!\s*\(\s*["'`]?"#,
        ];

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

        let head: String = lines.iter().take(30).copied().collect::<Vec<_>>().join("\n");
        let tail: String = lines.iter().rev().take(50).collect::<Vec<_>>().into_iter().rev().copied().collect::<Vec<_>>().join("\n");

        format!(
            "=== CONTEXT EXPLORATION ===\n\
             Total: {} chars, {} lines, ~{} tokens\n\n\
             === FIRST 30 LINES ===\n{}\n\n\
             === LAST 50 LINES ===\n{}\n\
             === END EXPLORATION ===",
            content.len(), total_lines, input_tokens, head, tail
        )
    }

    fn build_rlm_system_prompt(input_tokens: usize, tool_id: &str, query: &str) -> String {
        let context_type = if tool_id == "session_context" { "conversation history" } else { "tool output" };

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

    fn build_enhanced_fallback(output: &str, tool_id: &str, tool_args: &serde_json::Value, input_tokens: usize) -> String {
        let lines: Vec<&str> = output.lines().collect();

        if tool_id == "session_context" {
            // Extract key structural information
            let file_matches: Vec<&str> = lines.iter()
                .filter_map(|l| {
                    if l.contains(".ts") || l.contains(".rs") || l.contains(".py") || l.contains(".json") {
                        Some(*l)
                    } else {
                        None
                    }
                })
                .take(15)
                .collect();

            let tool_calls: Vec<&str> = lines.iter()
                .filter(|l| l.contains("[Tool "))
                .take(10)
                .copied()
                .collect();

            let errors: Vec<&str> = lines.iter()
                .filter(|l| l.to_lowercase().contains("error") || l.to_lowercase().contains("failed"))
                .take(5)
                .copied()
                .collect();

            let head: String = lines.iter().take(30).copied().collect::<Vec<_>>().join("\n");
            let tail: String = lines.iter().rev().take(80).collect::<Vec<_>>().into_iter().rev().copied().collect::<Vec<_>>().join("\n");

            let mut parts = vec![
                "## Context Summary (Fallback Mode)".to_string(),
                format!("*Original: {} tokens - RLM processing produced insufficient output*", input_tokens),
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
            format!("## Fallback Summary\n*RLM processing failed - showing structured excerpt*\n\n{}", truncated)
        }
    }

    fn fallback_result(output: &str, tool_id: &str, tool_args: &serde_json::Value, input_tokens: usize) -> RlmResult {
        let (truncated, _, _) = Self::smart_truncate(output, tool_id, tool_args, 8000);
        let output_tokens = RlmChunker::estimate_tokens(&truncated);

        RlmResult {
            processed: format!("[RLM processing failed, showing truncated output]\n\n{}", truncated),
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
        }
    }
}
