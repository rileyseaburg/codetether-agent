//! RLM REPL - Execution environment for RLM processing
//!
//! Provides a REPL-like environment where context is loaded as a variable
//! and the LLM can execute code to analyze it.
//!
//! Key feature: llm_query() function for recursive sub-LM calls.
//!
//! When the `functiongemma` feature is active the executor sends RLM tool
//! definitions alongside the analysis prompt.  Responses containing structured
//! `ContentPart::ToolCall` entries are dispatched directly instead of being
//! regex-parsed from code blocks (the legacy DSL path is kept as a fallback).

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::process::Stdio;
use std::sync::Arc;
use std::time::Duration;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::process::{Child, Command};
use tokio::time::timeout;

use crate::provider::{CompletionRequest, ContentPart, Message, Provider, Role};

use crate::cognition::tool_router::{ToolCallRouter, ToolRouterConfig};

use super::context_trace::{ContextEvent, ContextTrace, ContextTraceSummary};
use super::oracle::{FinalPayload, GrepMatch, GrepOracle, GrepPayload, QueryType, TraceStep};
use super::tools::{RlmToolResult, dispatch_tool_call, rlm_tool_definitions};

/// REPL runtime options
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum ReplRuntime {
    /// Native Rust REPL (fastest, uses rhai scripting)
    #[default]
    Rust,
    /// Bun/Node.js JavaScript REPL
    Bun,
    /// Python REPL
    Python,
}

/// REPL instance for RLM processing
pub struct RlmRepl {
    runtime: ReplRuntime,
    context: String,
    context_lines: Vec<String>,
    variables: HashMap<String, String>,
}

/// Result of REPL execution
#[derive(Debug, Clone)]
pub struct ReplResult {
    pub stdout: String,
    pub stderr: String,
    pub final_answer: Option<String>,
}

impl RlmRepl {
    /// Create a new REPL with the given context
    pub fn new(context: String, runtime: ReplRuntime) -> Self {
        let context_lines = context.lines().map(|s| s.to_string()).collect();
        Self {
            runtime,
            context,
            context_lines,
            variables: HashMap::new(),
        }
    }

    /// Get the context
    pub fn context(&self) -> &str {
        &self.context
    }

    /// Get context as lines
    pub fn lines(&self) -> &[String] {
        &self.context_lines
    }

    /// Get first n lines
    pub fn head(&self, n: usize) -> Vec<&str> {
        self.context_lines
            .iter()
            .take(n)
            .map(|s| s.as_str())
            .collect()
    }

    /// Get last n lines
    pub fn tail(&self, n: usize) -> Vec<&str> {
        let start = self.context_lines.len().saturating_sub(n);
        self.context_lines
            .iter()
            .skip(start)
            .map(|s| s.as_str())
            .collect()
    }

    /// Search for lines matching a pattern
    pub fn grep(&self, pattern: &str) -> Vec<(usize, &str)> {
        let re = match regex::Regex::new(pattern) {
            Ok(r) => r,
            Err(_) => {
                // Fall back to simple contains
                return self
                    .context_lines
                    .iter()
                    .enumerate()
                    .filter(|(_, line)| line.contains(pattern))
                    .map(|(i, line)| (i + 1, line.as_str()))
                    .collect();
            }
        };

        self.context_lines
            .iter()
            .enumerate()
            .filter(|(_, line)| re.is_match(line))
            .map(|(i, line)| (i + 1, line.as_str()))
            .collect()
    }

    /// Count occurrences of a pattern
    pub fn count(&self, pattern: &str) -> usize {
        let re = match regex::Regex::new(pattern) {
            Ok(r) => r,
            Err(_) => return self.context.matches(pattern).count(),
        };
        re.find_iter(&self.context).count()
    }

    /// Slice context by character positions
    pub fn slice(&self, start: usize, end: usize) -> &str {
        let total_chars = self.context.chars().count();
        let end = end.min(total_chars);
        let start = start.min(end);
        let start_byte = char_index_to_byte_index(&self.context, start);
        let end_byte = char_index_to_byte_index(&self.context, end);
        &self.context[start_byte..end_byte]
    }

    /// Split context into n chunks
    pub fn chunks(&self, n: usize) -> Vec<String> {
        if n == 0 {
            return vec![self.context.clone()];
        }

        let chunk_size = self.context_lines.len().div_ceil(n);
        self.context_lines
            .chunks(chunk_size)
            .map(|chunk| chunk.join("\n"))
            .collect()
    }

    /// Set a variable
    pub fn set_var(&mut self, name: &str, value: String) {
        self.variables.insert(name.to_string(), value);
    }

    /// Get a variable
    pub fn get_var(&self, name: &str) -> Option<&str> {
        self.variables.get(name).map(|s| s.as_str())
    }

    /// Execute analysis code (interpreted based on runtime)
    ///
    /// For Rust runtime, this uses a simple DSL:
    /// - head(n) - first n lines
    /// - tail(n) - last n lines
    /// - grep("pattern") - search for pattern
    /// - count("pattern") - count matches
    /// - slice(start, end) - slice by chars
    /// - chunks(n) - split into n chunks
    /// - FINAL({json_payload}) - return final answer payload
    pub fn execute(&mut self, code: &str) -> ReplResult {
        match self.runtime {
            ReplRuntime::Rust => self.execute_rust_dsl(code),
            ReplRuntime::Bun | ReplRuntime::Python => {
                // For external runtimes, we'd spawn processes
                // For now, fall back to DSL
                self.execute_rust_dsl(code)
            }
        }
    }

    fn execute_rust_dsl(&mut self, code: &str) -> ReplResult {
        let mut stdout = Vec::new();
        let mut final_answer = None;

        for line in code.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with("//") || line.starts_with('#') {
                continue;
            }

            // Parse and execute commands
            if let Some(result) = self.execute_dsl_line(line) {
                match result {
                    DslResult::Output(s) => stdout.push(s),
                    DslResult::Final(s) => {
                        final_answer = Some(s);
                        break;
                    }
                    DslResult::Error(s) => stdout.push(format!("Error: {}", s)),
                }
            }
        }

        ReplResult {
            stdout: stdout.join("\n"),
            stderr: String::new(),
            final_answer,
        }
    }

    pub fn execute_dsl_line(&mut self, line: &str) -> Option<DslResult> {
        // Check for FINAL
        if line.starts_with("FINAL(") || line.starts_with("FINAL!(") {
            let start = line.find('(').unwrap() + 1;
            let end = line.rfind(')').unwrap_or(line.len());
            let answer = line[start..end]
                .trim()
                .trim_matches(|c| c == '"' || c == '\'' || c == '`');
            return Some(DslResult::Final(answer.to_string()));
        }

        // Check for print/console.log
        if line.starts_with("print(")
            || line.starts_with("println!(")
            || line.starts_with("console.log(")
        {
            let start = line.find('(').unwrap() + 1;
            let end = line.rfind(')').unwrap_or(line.len());
            let content = line[start..end]
                .trim()
                .trim_matches(|c| c == '"' || c == '\'' || c == '`');

            // Expand variables
            let expanded = self.expand_expression(content);
            return Some(DslResult::Output(expanded));
        }

        // Check for variable assignment
        if let Some(eq_pos) = line.find('=')
            && !line.contains("==")
            && !line.starts_with("if ")
        {
            let var_name = line[..eq_pos]
                .trim()
                .trim_start_matches("let ")
                .trim_start_matches("const ")
                .trim_start_matches("var ")
                .trim();
            let expr = line[eq_pos + 1..].trim().trim_end_matches(';');

            let value = self.evaluate_expression(expr);
            self.set_var(var_name, value);
            return None;
        }

        // Check for function calls that should output
        if line.starts_with("head(")
            || line.starts_with("tail(")
            || line.starts_with("grep(")
            || line.starts_with("count(")
            || line.starts_with("lines()")
            || line.starts_with("slice(")
            || line.starts_with("chunks(")
            || line.starts_with("ast_query(")
            || line.starts_with("context")
        {
            let result = self.evaluate_expression(line);
            return Some(DslResult::Output(result));
        }

        None
    }

    fn expand_expression(&self, expr: &str) -> String {
        // Simple variable expansion
        let mut result = expr.to_string();

        for (name, value) in &self.variables {
            let patterns = [
                format!("${{{}}}", name),
                format!("${}", name),
                format!("{{{}}}", name),
            ];
            for p in patterns {
                result = result.replace(&p, value);
            }
        }

        // Evaluate embedded expressions
        if result.contains("context.len()") || result.contains("context.length") {
            result = result
                .replace("context.len()", &self.context.len().to_string())
                .replace("context.length", &self.context.len().to_string());
        }

        if result.contains("lines().len()") || result.contains("lines().length") {
            result = result
                .replace("lines().len()", &self.context_lines.len().to_string())
                .replace("lines().length", &self.context_lines.len().to_string());
        }

        result
    }

    pub fn evaluate_expression(&mut self, expr: &str) -> String {
        let expr = expr.trim().trim_end_matches(';');

        // head(n)
        if expr.starts_with("head(") {
            let n = self.extract_number(expr).unwrap_or(10);
            return self.head(n).join("\n");
        }

        // tail(n)
        if expr.starts_with("tail(") {
            let n = self.extract_number(expr).unwrap_or(10);
            return self.tail(n).join("\n");
        }

        // grep("pattern")
        if expr.starts_with("grep(") {
            let pattern = self.extract_string(expr).unwrap_or_default();
            let matches = self.grep(&pattern);
            return matches
                .iter()
                .map(|(i, line)| format!("{}:{}", i, line))
                .collect::<Vec<_>>()
                .join("\n");
        }

        // count("pattern")
        if expr.starts_with("count(") {
            let pattern = self.extract_string(expr).unwrap_or_default();
            return self.count(&pattern).to_string();
        }

        // lines()
        if expr == "lines()" || expr == "lines" {
            return format!("Lines: {}", self.context_lines.len());
        }

        // slice(start, end)
        if expr.starts_with("slice(") {
            let nums = self.extract_numbers(expr);
            if nums.len() >= 2 {
                return self.slice(nums[0], nums[1]).to_string();
            }
        }

        // chunks(n)
        if expr.starts_with("chunks(") || expr.starts_with("chunk(") {
            let n = self.extract_number(expr).unwrap_or(5);
            let chunks = self.chunks(n);
            return format!(
                "[{} chunks of {} lines each]",
                chunks.len(),
                chunks.first().map(|c| c.lines().count()).unwrap_or(0)
            );
        }

        // context
        if expr == "context" || expr.starts_with("context.slice") || expr.starts_with("context[") {
            return format!(
                "[Context: {} chars, {} lines]",
                self.context.len(),
                self.context_lines.len()
            );
        }

        // ast_query("s-expression")
        // Execute a tree-sitter AST query and return formatted results
        if expr.starts_with("ast_query(") {
            let query = self.extract_string(expr).unwrap_or_default();
            return self.execute_ast_query(&query);
        }

        // Variable reference
        if let Some(val) = self.get_var(expr) {
            return val.to_string();
        }

        // String literal
        if (expr.starts_with('"') && expr.ends_with('"'))
            || (expr.starts_with('\'') && expr.ends_with('\''))
        {
            let mut chars = expr.chars();
            let _ = chars.next();
            let _ = chars.next_back();
            return chars.collect();
        }

        expr.to_string()
    }

    fn extract_number(&self, expr: &str) -> Option<usize> {
        let start = expr.find('(')?;
        let end = expr.find(')')?;
        let inner = expr[start + 1..end].trim();
        inner.parse().ok()
    }

    fn extract_numbers(&self, expr: &str) -> Vec<usize> {
        let start = expr.find('(').unwrap_or(0);
        let end = expr.find(')').unwrap_or(expr.len());
        let inner = &expr[start + 1..end];

        inner
            .split(',')
            .filter_map(|s| s.trim().parse().ok())
            .collect()
    }

    fn extract_string(&self, expr: &str) -> Option<String> {
        let start = expr.find('(')?;
        let end = expr.rfind(')')?;
        let inner = expr[start + 1..end].trim();

        // Remove quotes
        let unquoted = inner
            .trim_start_matches(['"', '\'', '`', '/'])
            .trim_end_matches(['"', '\'', '`', '/']);

        Some(unquoted.to_string())
    }

    /// Execute a tree-sitter AST query on the context.
    fn execute_ast_query(&self, query: &str) -> String {
        let mut oracle = super::oracle::TreeSitterOracle::new(self.context.clone());

        match oracle.query(query) {
            Ok(result) => {
                if result.matches.is_empty() {
                    "(no AST matches)".to_string()
                } else {
                    let lines: Vec<String> = result
                        .matches
                        .iter()
                        .map(|m| {
                            let captures_str: Vec<String> = m
                                .captures
                                .iter()
                                .map(|(k, v)| format!("{}={:?}", k, v))
                                .collect();
                            format!("L{}: {} [{}]", m.line, m.text, captures_str.join(", "))
                        })
                        .collect();
                    lines.join("\n")
                }
            }
            Err(e) => format!("AST query error: {}", e),
        }
    }
}

pub enum DslResult {
    Output(String),
    Final(String),
    #[allow(dead_code)]
    Error(String),
}

/// LLM-powered RLM executor
///
/// This is the main entry point for RLM processing. It:
/// 1. Loads context into a REPL environment
/// 2. Lets the LLM write analysis code
/// 3. Executes the code and provides llm_query() for semantic sub-calls
/// 4. Iterates until the LLM returns a FINAL answer
///
/// When FunctionGemma is enabled, the executor passes RLM tool definitions to
/// the provider and dispatches structured tool calls instead of regex-parsing
/// DSL from code blocks.
pub struct RlmExecutor {
    repl: RlmRepl,
    provider: Arc<dyn Provider>,
    model: String,
    analysis_temperature: f32,
    max_iterations: usize,
    sub_queries: Vec<SubQuery>,
    trace_steps: Vec<TraceStep>,
    context_budget_tokens: usize,
    context_trace: ContextTrace,
    verbose: bool,

    tool_router: Option<ToolCallRouter>,
}

/// Record of a sub-LM call
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubQuery {
    pub query: String,
    pub context_slice: Option<String>,
    pub response: String,
    pub tokens_used: usize,
}

impl RlmExecutor {
    /// Create a new RLM executor
    pub fn new(context: String, provider: Arc<dyn Provider>, model: String) -> Self {
        let context_budget_tokens = std::env::var("CODETETHER_RLM_CONTEXT_BUDGET")
            .ok()
            .and_then(|v| v.parse::<usize>().ok())
            .unwrap_or(32_768);

        let tool_router = {
            let cfg = ToolRouterConfig::from_env();
            ToolCallRouter::from_config(&cfg)
                .inspect_err(|e| {
                    tracing::debug!(error = %e, "FunctionGemma router unavailable for RLM");
                })
                .ok()
                .flatten()
        };

        Self {
            repl: RlmRepl::new(context, ReplRuntime::Rust),
            provider,
            model,
            analysis_temperature: 0.3,
            max_iterations: 5, // Keep iterations limited for speed
            sub_queries: Vec::new(),
            trace_steps: Vec::new(),
            context_budget_tokens,
            context_trace: ContextTrace::new(context_budget_tokens),
            verbose: false,

            tool_router,
        }
    }

    /// Set maximum iterations
    pub fn with_max_iterations(mut self, max: usize) -> Self {
        self.max_iterations = max;
        self
    }

    /// Set root analysis temperature for top-level RLM iterations.
    pub fn with_temperature(mut self, temperature: f32) -> Self {
        self.analysis_temperature = temperature.clamp(0.0, 2.0);
        self
    }

    /// Enable or disable verbose mode
    ///
    /// When verbose is true, the context summary will be displayed
    /// at the start of analysis to help users understand what's being analyzed.
    pub fn with_verbose(mut self, verbose: bool) -> Self {
        self.verbose = verbose;
        self
    }

    /// Trace steps captured from the most recent analysis run.
    pub fn trace_steps(&self) -> &[TraceStep] {
        &self.trace_steps
    }

    /// Context trace summary for the most recent analysis run.
    pub fn context_trace_summary(&self) -> ContextTraceSummary {
        self.context_trace.summary()
    }

    /// Execute RLM analysis with the given query
    pub async fn analyze(&mut self, query: &str) -> Result<RlmAnalysisResult> {
        let start = std::time::Instant::now();
        let mut iterations = 0;
        let mut total_input_tokens = 0;
        let mut total_output_tokens = 0;
        self.sub_queries.clear();
        self.trace_steps.clear();
        self.context_trace = ContextTrace::new(self.context_budget_tokens);

        // Prepare RLM tool definitions for structured dispatch
        let tools = rlm_tool_definitions();

        // Build and optionally display context summary
        let context_summary = format!(
            "=== CONTEXT LOADED ===\n\
             Total: {} chars, {} lines\n\
             Available functions:\n\
             - head(n) - first n lines\n\
             - tail(n) - last n lines\n\
             - grep(\"pattern\") - find lines matching regex\n\
             - count(\"pattern\") - count regex matches\n\
             - slice(start, end) - slice by char position\n\
             - chunks(n) - split into n chunks\n\
             - ast_query(\"s-expr\") - tree-sitter AST query for structural analysis\n\
             - llm_query(\"question\", context?) - ask sub-LM a question\n\
             - FINAL({{json_payload}}) - return structured final payload\n\
             === END CONTEXT INFO ===",
            self.repl.context().len(),
            self.repl.lines().len()
        );

        // Display context summary at the start in verbose mode
        if self.verbose {
            tracing::info!("RLM Context Summary:\n{}", context_summary);
            println!(
                "[RLM] Context loaded: {} chars, {} lines",
                self.repl.context().len(),
                self.repl.lines().len()
            );
        }

        let system_prompt = format!(
            "You are a code analysis assistant. Answer questions by examining the provided context.\n\n\
             CRITICAL OUTPUT CONTRACT:\n\
             - Your final response MUST be exactly one FINAL(<json>) call.\n\
             - Never end with prose and never use FINAL(\"...\").\n\
             - For pattern/grep queries, you MUST run grep/head/tail first. Never guess line numbers or matches.\n\
             - The JSON inside FINAL(...) MUST match one of these shapes:\n\
               1) {{\"kind\":\"grep\",\"file\":\"<path>\",\"pattern\":\"<regex>\",\"matches\":[{{\"line\":123,\"text\":\"...\"}}]}}\n\
               2) {{\"kind\":\"ast\",\"file\":\"<path>\",\"query\":\"<tree-sitter query>\",\"results\":[{{\"name\":\"...\",\"args\":[],\"return_type\":null,\"span\":[1,2]}}]}}\n\
               3) {{\"kind\":\"semantic\",\"file\":\"<path>\",\"answer\":\"...\"}} (only if deterministic payload is impossible)\n\
             - For grep/list/find/count style queries, emit kind=grep with exact line-numbered matches.\n\n\
             Available commands:\n\
             - head(n), tail(n): See first/last n lines\n\
             - grep(\"pattern\"): Search for patterns\n\
             - ast_query(\"s-expr\"): Tree-sitter AST query (e.g., '(function_item name: (identifier) @name)')\n\
             - llm_query(\"question\"): Ask a focused sub-question\n\
             - FINAL({{\"kind\":\"...\", ...}}): Return your final structured payload (REQUIRED)\n\n\
             The context has {} chars across {} lines. A preview follows:\n\n\
             {}\n\n\
             Example final response:\n\
             FINAL({{\"kind\":\"grep\",\"file\":\"src/rlm/repl.rs\",\"pattern\":\"async fn\",\"matches\":[{{\"line\":570,\"text\":\"pub async fn analyze(...)\"}}]}})\n\n\
             Now analyze the context. Use 1-2 commands if needed, then call FINAL() with valid JSON payload.",
            self.repl.context().len(),
            self.repl.lines().len(),
            self.repl.head(25).join("\n")
        );

        let mut messages = vec![
            Message {
                role: Role::System,
                content: vec![ContentPart::Text {
                    text: system_prompt,
                }],
            },
            Message {
                role: Role::User,
                content: vec![ContentPart::Text {
                    text: format!("Analyze and answer: {}", query),
                }],
            },
        ];
        let initial_system = messages[0]
            .content
            .iter()
            .filter_map(|p| match p {
                ContentPart::Text { text } => Some(text.as_str()),
                _ => None,
            })
            .collect::<Vec<_>>()
            .join("");
        self.context_trace.log_event(ContextEvent::SystemPrompt {
            content: "rlm_system_prompt".to_string(),
            tokens: ContextTrace::estimate_tokens(&initial_system),
        });
        self.context_trace.log_event(ContextEvent::ToolCall {
            name: "initial_query".to_string(),
            arguments_preview: truncate_with_ellipsis(query, 160),
            tokens: ContextTrace::estimate_tokens(query),
        });

        let llm_timeout_secs = std::env::var("CODETETHER_RLM_LLM_TIMEOUT_SECS")
            .ok()
            .and_then(|v| v.parse::<u64>().ok())
            .filter(|v| *v > 0)
            .unwrap_or(60);
        let requires_pattern_evidence =
            GrepOracle::classify_query(query) == QueryType::PatternMatch;

        let mut final_answer = None;

        while iterations < self.max_iterations {
            iterations += 1;
            tracing::info!("RLM iteration {}", iterations);

            // Get LLM response with code to execute (with timeout)
            tracing::debug!("Sending LLM request...");
            let response = match tokio::time::timeout(
                std::time::Duration::from_secs(llm_timeout_secs),
                self.provider.complete(CompletionRequest {
                    messages: messages.clone(),
                    tools: tools.clone(),
                    model: self.model.clone(),
                    temperature: Some(self.analysis_temperature),
                    top_p: None,
                    max_tokens: Some(2000),
                    stop: vec![],
                }),
            )
            .await
            {
                Ok(Ok(r)) => {
                    tracing::debug!("LLM response received");
                    r
                }
                Ok(Err(e)) => return Err(e),
                Err(_) => {
                    return Err(anyhow::anyhow!(
                        "LLM request timed out after {} seconds",
                        llm_timeout_secs
                    ));
                }
            };

            // Optionally run FunctionGemma to convert text-only responses into
            // structured tool calls.

            let response = if let Some(ref router) = self.tool_router {
                // RLM executor currently uses the same provider as the main session,
                // which always supports native tool calling.  Pass `true` so
                // FunctionGemma is skipped — it would only waste CPU here.
                router.maybe_reformat(response, &tools, true).await
            } else {
                response
            };

            total_input_tokens += response.usage.prompt_tokens;
            total_output_tokens += response.usage.completion_tokens;
            let assistant_text = response
                .message
                .content
                .iter()
                .filter_map(|p| match p {
                    ContentPart::Text { text } => Some(text.as_str()),
                    _ => None,
                })
                .collect::<Vec<_>>()
                .join("");
            if !assistant_text.is_empty() {
                self.context_trace.log_event(ContextEvent::AssistantCode {
                    code: truncate_with_ellipsis(&assistant_text, 500),
                    tokens: ContextTrace::estimate_tokens(&assistant_text),
                });
            }

            // ── Structured tool-call path ────────────────────────────────
            // If the response (or FunctionGemma rewrite) contains ToolCall
            // entries, dispatch them directly against the REPL.
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
                tracing::info!(
                    count = tool_calls.len(),
                    "RLM: dispatching structured tool calls"
                );

                // Keep the assistant message (including tool call parts) in history
                messages.push(Message {
                    role: Role::Assistant,
                    content: response.message.content.clone(),
                });

                let mut tool_results: Vec<ContentPart> = Vec::new();

                for (call_id, name, arguments) in &tool_calls {
                    self.context_trace.log_event(ContextEvent::ToolCall {
                        name: name.clone(),
                        arguments_preview: truncate_with_ellipsis(arguments, 200),
                        tokens: ContextTrace::estimate_tokens(arguments),
                    });
                    match dispatch_tool_call(name, arguments, &mut self.repl) {
                        Some(RlmToolResult::Final(answer)) => {
                            if requires_pattern_evidence && !self.has_pattern_evidence() {
                                let rejection = "FINAL rejected: pattern query requires grep evidence. Call rlm_grep first, then FINAL with exact line-numbered matches.";
                                self.trace_steps.push(TraceStep {
                                    iteration: iterations,
                                    action: "reject_final(no_grep_evidence)".to_string(),
                                    output: rejection.to_string(),
                                });
                                self.context_trace.log_event(ContextEvent::ToolResult {
                                    tool_call_id: call_id.clone(),
                                    result_preview: rejection.to_string(),
                                    tokens: ContextTrace::estimate_tokens(rejection),
                                });
                                tool_results.push(ContentPart::ToolResult {
                                    tool_call_id: call_id.clone(),
                                    content: rejection.to_string(),
                                });
                                continue;
                            }
                            if self.verbose {
                                println!("[RLM] Final answer received via tool call");
                            }
                            final_answer = Some(answer.clone());
                            self.trace_steps.push(TraceStep {
                                iteration: iterations,
                                action: format!(
                                    "{name}({})",
                                    truncate_with_ellipsis(arguments, 120)
                                ),
                                output: format!("FINAL: {}", truncate_with_ellipsis(&answer, 240)),
                            });
                            self.context_trace.log_event(ContextEvent::Final {
                                answer: truncate_with_ellipsis(&answer, 400),
                                tokens: ContextTrace::estimate_tokens(&answer),
                            });
                            tool_results.push(ContentPart::ToolResult {
                                tool_call_id: call_id.clone(),
                                content: format!("FINAL: {answer}"),
                            });
                            break;
                        }
                        Some(RlmToolResult::Output(output)) => {
                            // Check for the llm_query sentinel
                            if let Ok(sentinel) = serde_json::from_str::<serde_json::Value>(&output)
                                && sentinel
                                    .get("__rlm_llm_query")
                                    .and_then(|v| v.as_bool())
                                    .unwrap_or(false)
                            {
                                let q =
                                    sentinel.get("query").and_then(|v| v.as_str()).unwrap_or("");
                                let ctx_slice = sentinel
                                    .get("context_slice")
                                    .and_then(|v| v.as_str())
                                    .map(|s| s.to_string());
                                let llm_result = self.handle_llm_query_direct(q, ctx_slice).await?;
                                self.trace_steps.push(TraceStep {
                                    iteration: iterations,
                                    action: format!(
                                        "llm_query({})",
                                        truncate_with_ellipsis(q, 120)
                                    ),
                                    output: truncate_with_ellipsis(&llm_result, 240),
                                });
                                self.context_trace.log_event(ContextEvent::ToolResult {
                                    tool_call_id: call_id.clone(),
                                    result_preview: truncate_with_ellipsis(&llm_result, 300),
                                    tokens: ContextTrace::estimate_tokens(&llm_result),
                                });
                                tool_results.push(ContentPart::ToolResult {
                                    tool_call_id: call_id.clone(),
                                    content: llm_result,
                                });
                                continue;
                            }
                            // Normal REPL output
                            if self.verbose {
                                let preview = truncate_with_ellipsis(&output, 200);
                                println!("[RLM] Tool {name} → {}", preview);
                            }
                            self.trace_steps.push(TraceStep {
                                iteration: iterations,
                                action: format!(
                                    "{name}({})",
                                    truncate_with_ellipsis(arguments, 120)
                                ),
                                output: Self::trace_output_for_storage(&output),
                            });
                            self.context_trace.log_event(ContextEvent::ToolResult {
                                tool_call_id: call_id.clone(),
                                result_preview: truncate_with_ellipsis(&output, 300),
                                tokens: ContextTrace::estimate_tokens(&output),
                            });
                            tool_results.push(ContentPart::ToolResult {
                                tool_call_id: call_id.clone(),
                                content: output,
                            });
                        }
                        None => {
                            let unknown = format!("Unknown tool: {name}");
                            self.trace_steps.push(TraceStep {
                                iteration: iterations,
                                action: format!(
                                    "{name}({})",
                                    truncate_with_ellipsis(arguments, 120)
                                ),
                                output: unknown.clone(),
                            });
                            tool_results.push(ContentPart::ToolResult {
                                tool_call_id: call_id.clone(),
                                content: unknown,
                            });
                        }
                    }
                }

                // Send tool results back as a Role::Tool message
                if !tool_results.is_empty() {
                    messages.push(Message {
                        role: Role::Tool,
                        content: tool_results,
                    });
                }

                if final_answer.is_some() {
                    break;
                }
                continue;
            }

            // ── Legacy DSL path (fallback) ───────────────────────────────
            // If no structured tool calls were produced, fall back to the
            // original regex-parsed code-block execution.
            let assistant_text = response
                .message
                .content
                .iter()
                .filter_map(|p| match p {
                    ContentPart::Text { text } => Some(text.as_str()),
                    _ => None,
                })
                .collect::<Vec<_>>()
                .join("");

            // Add assistant message
            messages.push(Message {
                role: Role::Assistant,
                content: vec![ContentPart::Text {
                    text: assistant_text.clone(),
                }],
            });

            // Extract and execute code blocks
            let code = self.extract_code(&assistant_text);
            self.trace_steps.push(TraceStep {
                iteration: iterations,
                action: format!("execute_code({})", truncate_with_ellipsis(&code, 160)),
                output: String::new(),
            });

            // Display execution details in verbose mode
            if self.verbose {
                println!("[RLM] Iteration {}: Executing code:\n{}", iterations, code);
            }

            let execution_result = self.execute_with_llm_query(&code).await?;

            // Display execution results in verbose mode
            if self.verbose {
                if let Some(ref answer) = execution_result.final_answer {
                    println!("[RLM] Final answer received: {}", answer);
                } else if !execution_result.stdout.is_empty() {
                    let preview = truncate_with_ellipsis(&execution_result.stdout, 200);
                    println!("[RLM] Execution output:\n{}", preview);
                }
            }

            // Check for final answer
            if let Some(answer) = &execution_result.final_answer {
                let stdout_has_pattern_evidence = code.contains("grep(")
                    || execution_result.stdout.contains("(no matches)")
                    || !Self::parse_line_numbered_output(&execution_result.stdout).is_empty();
                if requires_pattern_evidence
                    && !self.has_pattern_evidence()
                    && !stdout_has_pattern_evidence
                {
                    let rejection = "FINAL rejected: pattern query requires grep evidence. Run grep/head/tail before FINAL and include exact lines.";
                    self.trace_steps.push(TraceStep {
                        iteration: iterations,
                        action: "reject_final(no_grep_evidence)".to_string(),
                        output: rejection.to_string(),
                    });
                    self.context_trace.log_event(ContextEvent::ExecutionOutput {
                        output: rejection.to_string(),
                        tokens: ContextTrace::estimate_tokens(rejection),
                    });
                    messages.push(Message {
                        role: Role::User,
                        content: vec![ContentPart::Text {
                            text: rejection.to_string(),
                        }],
                    });
                    continue;
                }
                self.context_trace.log_event(ContextEvent::Final {
                    answer: truncate_with_ellipsis(answer, 400),
                    tokens: ContextTrace::estimate_tokens(answer),
                });
                final_answer = Some(answer.clone());
                break;
            }

            self.context_trace.log_event(ContextEvent::ExecutionOutput {
                output: truncate_with_ellipsis(&execution_result.stdout, 400),
                tokens: ContextTrace::estimate_tokens(&execution_result.stdout),
            });
            if let Some(step) = self.trace_steps.last_mut()
                && step.iteration == iterations
                && step.output.is_empty()
            {
                step.output = Self::trace_output_for_storage(&execution_result.stdout);
            }

            // Add execution result as user message for next iteration
            let result_text = if execution_result.stdout.is_empty() {
                "[No output]".to_string()
            } else {
                format!("Execution result:\n{}", execution_result.stdout)
            };

            messages.push(Message {
                role: Role::User,
                content: vec![ContentPart::Text { text: result_text }],
            });
        }

        let elapsed = start.elapsed();

        let raw_final_text = final_answer.unwrap_or_else(|| "Analysis incomplete".to_string());
        let final_text = self.ensure_structured_final_payload(query, raw_final_text);
        if !matches!(
            self.context_trace.events().back(),
            Some(ContextEvent::Final { .. })
        ) {
            self.context_trace.log_event(ContextEvent::Final {
                answer: truncate_with_ellipsis(&final_text, 400),
                tokens: ContextTrace::estimate_tokens(&final_text),
            });
        }

        Ok(RlmAnalysisResult {
            answer: final_text,
            iterations,
            sub_queries: self.sub_queries.clone(),
            stats: super::RlmStats {
                input_tokens: total_input_tokens,
                output_tokens: total_output_tokens,
                iterations,
                subcalls: self.sub_queries.len(),
                elapsed_ms: elapsed.as_millis() as u64,
                compression_ratio: 1.0,
            },
        })
    }

    fn ensure_structured_final_payload(&mut self, query: &str, raw_final_text: String) -> String {
        let parsed = FinalPayload::parse(&raw_final_text);

        if let Some(canonical) = self.coerce_grep_payload_from_trace(query) {
            let canonical_payload = FinalPayload::parse(&canonical);
            match &parsed {
                FinalPayload::Grep(_) => {
                    if canonical_payload != parsed {
                        let iteration = self.trace_steps.last().map(|s| s.iteration).unwrap_or(1);
                        self.trace_steps.push(TraceStep {
                            iteration,
                            action: "normalize_final_payload(grep_trace)".to_string(),
                            output: truncate_with_ellipsis(&canonical, 240),
                        });
                        self.context_trace.log_event(ContextEvent::Final {
                            answer: truncate_with_ellipsis(&canonical, 400),
                            tokens: ContextTrace::estimate_tokens(&canonical),
                        });
                        tracing::info!(
                            "RLM normalized FINAL(JSON) grep payload using trace evidence"
                        );
                        return canonical;
                    }
                    return raw_final_text;
                }
                FinalPayload::Malformed { .. } => {
                    let iteration = self.trace_steps.last().map(|s| s.iteration).unwrap_or(1);
                    self.trace_steps.push(TraceStep {
                        iteration,
                        action: "coerce_final_payload(grep_trace)".to_string(),
                        output: truncate_with_ellipsis(&canonical, 240),
                    });
                    self.context_trace.log_event(ContextEvent::Final {
                        answer: truncate_with_ellipsis(&canonical, 400),
                        tokens: ContextTrace::estimate_tokens(&canonical),
                    });
                    tracing::info!(
                        "RLM coerced malformed/prose final answer into FINAL(JSON) grep payload"
                    );
                    return canonical;
                }
                _ => {
                    let iteration = self.trace_steps.last().map(|s| s.iteration).unwrap_or(1);
                    self.trace_steps.push(TraceStep {
                        iteration,
                        action: "coerce_final_payload(grep_trace)".to_string(),
                        output: truncate_with_ellipsis(&canonical, 240),
                    });
                    self.context_trace.log_event(ContextEvent::Final {
                        answer: truncate_with_ellipsis(&canonical, 400),
                        tokens: ContextTrace::estimate_tokens(&canonical),
                    });
                    tracing::info!(
                        "RLM coerced non-grep FINAL payload into canonical grep payload using trace evidence"
                    );
                    return canonical;
                }
            }
        }

        raw_final_text
    }

    fn coerce_grep_payload_from_trace(&self, query: &str) -> Option<String> {
        if GrepOracle::classify_query(query) != QueryType::PatternMatch {
            return None;
        }

        let pattern = GrepOracle::infer_pattern(query)?;
        let matches = self.extract_latest_grep_matches()?;
        let file = Self::infer_file_from_query(query).unwrap_or_else(|| "unknown".to_string());

        let payload = FinalPayload::Grep(GrepPayload {
            file,
            pattern,
            matches: matches
                .into_iter()
                .map(|(line, text)| GrepMatch { line, text })
                .collect(),
        });

        serde_json::to_string(&payload).ok()
    }

    fn has_pattern_evidence(&self) -> bool {
        self.trace_steps.iter().any(|step| {
            step.action.contains("grep(")
                || step.action.contains("rlm_grep(")
                || step.output.contains("(no matches)")
                || !Self::parse_line_numbered_output(&step.output).is_empty()
        })
    }

    fn extract_latest_grep_matches(&self) -> Option<Vec<(usize, String)>> {
        for step in self.trace_steps.iter().rev() {
            if !step.action.contains("grep(") && !step.action.contains("rlm_grep(") {
                continue;
            }
            if step.output.trim() == "(no matches)" {
                return Some(Vec::new());
            }
            let parsed = Self::parse_line_numbered_output(&step.output);
            if !parsed.is_empty() {
                return Some(parsed);
            }
        }

        for step in self.trace_steps.iter().rev() {
            let parsed = Self::parse_line_numbered_output(&step.output);
            if !parsed.is_empty() {
                return Some(parsed);
            }
        }

        None
    }

    fn parse_line_numbered_output(output: &str) -> Vec<(usize, String)> {
        output
            .lines()
            .filter_map(|line| {
                let trimmed = line.trim();
                let (line_no, text) = trimmed.split_once(':')?;
                let number = line_no
                    .trim()
                    .trim_start_matches('L')
                    .parse::<usize>()
                    .ok()?;
                Some((number, text.trim_end_matches('\r').to_string()))
            })
            .collect()
    }

    fn infer_file_from_query(query: &str) -> Option<String> {
        let lower = query.to_lowercase();
        let idx = lower.rfind(" in ")?;
        let candidate = query[idx + 4..]
            .split_whitespace()
            .next()
            .unwrap_or_default()
            .trim_matches(|c: char| c == '"' || c == '\'' || c == '`' || c == '.' || c == ',');
        if candidate.is_empty() {
            None
        } else {
            Some(candidate.to_string())
        }
    }

    fn trace_output_for_storage(output: &str) -> String {
        let trimmed = output.trim();
        if trimmed == "(no matches)" || !Self::parse_line_numbered_output(output).is_empty() {
            output.to_string()
        } else {
            truncate_with_ellipsis(output, 240)
        }
    }

    /// Extract code from LLM response
    fn extract_code(&self, text: &str) -> String {
        // Look for fenced code blocks
        let mut code_lines = Vec::new();
        let mut in_code_block = false;

        for line in text.lines() {
            if line.starts_with("```") {
                in_code_block = !in_code_block;
                continue;
            }
            if in_code_block {
                code_lines.push(line);
            }
        }

        if !code_lines.is_empty() {
            return code_lines.join("\n");
        }

        // If no code blocks, look for lines that look like code
        text.lines()
            .filter(|line| {
                let l = line.trim();
                l.starts_with("head(")
                    || l.starts_with("tail(")
                    || l.starts_with("grep(")
                    || l.starts_with("count(")
                    || l.starts_with("llm_query(")
                    || l.starts_with("ast_query(")
                    || l.starts_with("FINAL(")
                    || l.starts_with("let ")
                    || l.starts_with("const ")
                    || l.starts_with("print")
                    || l.starts_with("console.")
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// Execute code with llm_query() support
    async fn execute_with_llm_query(&mut self, code: &str) -> Result<ReplResult> {
        let mut stdout = Vec::new();
        let mut final_answer = None;

        for line in code.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with("//") || line.starts_with('#') {
                continue;
            }

            // Handle llm_query calls specially
            if line.starts_with("llm_query(") || line.contains("= llm_query(") {
                let result = self.handle_llm_query(line).await?;
                stdout.push(result);
                continue;
            }

            // Handle regular REPL commands
            if let Some(result) = self.repl.execute_dsl_line(line) {
                match result {
                    DslResult::Output(s) => stdout.push(s),
                    DslResult::Final(s) => {
                        final_answer = Some(s);
                        break;
                    }
                    DslResult::Error(s) => stdout.push(format!("Error: {}", s)),
                }
            }
        }

        Ok(ReplResult {
            stdout: stdout.join("\n"),
            stderr: String::new(),
            final_answer,
        })
    }

    /// Handle llm_query() calls
    async fn handle_llm_query(&mut self, line: &str) -> Result<String> {
        // Extract query and optional context slice
        let (query, context_slice) = self.parse_llm_query(line);
        self.context_trace.log_event(ContextEvent::LlmQueryResult {
            query: query.clone(),
            response_preview: "[pending]".to_string(),
            tokens: ContextTrace::estimate_tokens(&query),
        });

        // Get the context to send
        let context_to_analyze = context_slice
            .clone()
            .unwrap_or_else(|| self.repl.context().to_string());

        // Truncate context for sub-query to avoid overwhelming the LLM
        let context_chars = context_to_analyze.chars().count();
        let truncated_context = if context_chars > 8000 {
            format!(
                "{}\n[truncated, {} chars total]",
                truncate_with_ellipsis(&context_to_analyze, 7500),
                context_chars
            )
        } else {
            context_to_analyze.clone()
        };

        // Make sub-LM call
        let messages = vec![
            Message {
                role: Role::System,
                content: vec![ContentPart::Text {
                    text: "You are a focused analysis assistant. Answer the question based on the provided context. Be concise.".to_string(),
                }],
            },
            Message {
                role: Role::User,
                content: vec![ContentPart::Text {
                    text: format!("Context:\n{}\n\nQuestion: {}", truncated_context, query),
                }],
            },
        ];

        let response = self
            .provider
            .complete(CompletionRequest {
                messages,
                tools: vec![],
                model: self.model.clone(),
                temperature: Some(0.3),
                top_p: None,
                max_tokens: Some(500),
                stop: vec![],
            })
            .await?;

        let answer = response
            .message
            .content
            .iter()
            .filter_map(|p| match p {
                ContentPart::Text { text } => Some(text.as_str()),
                _ => None,
            })
            .collect::<Vec<_>>()
            .join("");

        // Record the sub-query
        self.sub_queries.push(SubQuery {
            query: query.clone(),
            context_slice,
            response: answer.clone(),
            tokens_used: response.usage.total_tokens,
        });
        self.trace_steps.push(TraceStep {
            iteration: self.sub_queries.len(),
            action: format!("llm_query({})", truncate_with_ellipsis(&query, 120)),
            output: truncate_with_ellipsis(&answer, 240),
        });
        self.context_trace.log_event(ContextEvent::LlmQueryResult {
            query,
            response_preview: truncate_with_ellipsis(&answer, 240),
            tokens: response.usage.total_tokens,
        });

        Ok(format!("llm_query result: {}", answer))
    }

    /// Handle an `rlm_llm_query` tool call from the structured path.
    ///
    /// This is the equivalent of `handle_llm_query` but takes pre-parsed
    /// parameters instead of a raw DSL line.
    async fn handle_llm_query_direct(
        &mut self,
        query: &str,
        context_slice: Option<String>,
    ) -> Result<String> {
        self.context_trace.log_event(ContextEvent::LlmQueryResult {
            query: query.to_string(),
            response_preview: "[pending]".to_string(),
            tokens: ContextTrace::estimate_tokens(query),
        });
        let context_to_analyze = context_slice
            .clone()
            .unwrap_or_else(|| self.repl.context().to_string());

        let context_chars = context_to_analyze.chars().count();
        let truncated_context = if context_chars > 8000 {
            format!(
                "{}\n[truncated, {} chars total]",
                truncate_with_ellipsis(&context_to_analyze, 7500),
                context_chars
            )
        } else {
            context_to_analyze.clone()
        };

        let messages = vec![
            Message {
                role: Role::System,
                content: vec![ContentPart::Text {
                    text: "You are a focused analysis assistant. Answer the question based on the provided context. Be concise.".to_string(),
                }],
            },
            Message {
                role: Role::User,
                content: vec![ContentPart::Text {
                    text: format!("Context:\n{}\n\nQuestion: {}", truncated_context, query),
                }],
            },
        ];

        let response = self
            .provider
            .complete(CompletionRequest {
                messages,
                tools: vec![],
                model: self.model.clone(),
                temperature: Some(0.3),
                top_p: None,
                max_tokens: Some(500),
                stop: vec![],
            })
            .await?;

        let answer = response
            .message
            .content
            .iter()
            .filter_map(|p| match p {
                ContentPart::Text { text } => Some(text.as_str()),
                _ => None,
            })
            .collect::<Vec<_>>()
            .join("");

        self.sub_queries.push(SubQuery {
            query: query.to_string(),
            context_slice,
            response: answer.clone(),
            tokens_used: response.usage.total_tokens,
        });
        self.context_trace.log_event(ContextEvent::LlmQueryResult {
            query: query.to_string(),
            response_preview: truncate_with_ellipsis(&answer, 240),
            tokens: response.usage.total_tokens,
        });

        Ok(format!("llm_query result: {}", answer))
    }

    /// Parse llm_query("question", context?) call
    fn parse_llm_query(&mut self, line: &str) -> (String, Option<String>) {
        // Find the query string
        let start = line.find('(').unwrap_or(0) + 1;
        let end = line.rfind(')').unwrap_or(line.len());
        let args = &line[start..end];

        // Split by comma, but respect quotes
        let mut query = String::new();
        let mut context = None;
        let mut in_quotes = false;
        let mut current = String::new();
        let mut parts = Vec::new();

        for c in args.chars() {
            if c == '"' || c == '\'' {
                in_quotes = !in_quotes;
            } else if c == ',' && !in_quotes {
                parts.push(current.trim().to_string());
                current = String::new();
                continue;
            }
            current.push(c);
        }
        if !current.is_empty() {
            parts.push(current.trim().to_string());
        }

        // First part is the query
        if let Some(q) = parts.first() {
            query = q.trim_matches(|c| c == '"' || c == '\'').to_string();
        }

        // Second part (if present) is context expression
        if let Some(ctx_expr) = parts.get(1) {
            // Evaluate the context expression
            let ctx = self.repl.evaluate_expression(ctx_expr);
            if !ctx.is_empty() && !ctx.starts_with('[') {
                context = Some(ctx);
            }
        }

        (query, context)
    }
}

/// Result of RLM analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RlmAnalysisResult {
    pub answer: String,
    pub iterations: usize,
    pub sub_queries: Vec<SubQuery>,
    pub stats: super::RlmStats,
}

/// Spawn an external REPL process for Python or Bun
pub struct ExternalRepl {
    child: Child,
    #[allow(dead_code)]
    runtime: ReplRuntime,
}

impl ExternalRepl {
    /// Create a Bun/Node.js REPL
    pub async fn spawn_bun(context: &str) -> Result<Self> {
        let init_script = Self::generate_bun_init(context);

        // Write init script to temp file
        let temp_dir = std::env::temp_dir().join("rlm-repl");
        tokio::fs::create_dir_all(&temp_dir).await?;
        let script_path = temp_dir.join(format!("init_{}.js", std::process::id()));
        tokio::fs::write(&script_path, init_script).await?;

        // Try bun first, fall back to node
        let runtime = if Self::is_bun_available().await {
            "bun"
        } else {
            "node"
        };

        let child = Command::new(runtime)
            .arg(&script_path)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        Ok(Self {
            child,
            runtime: ReplRuntime::Bun,
        })
    }

    async fn is_bun_available() -> bool {
        Command::new("bun")
            .arg("--version")
            .output()
            .await
            .map(|o| o.status.success())
            .unwrap_or(false)
    }

    fn generate_bun_init(context: &str) -> String {
        let escaped = context
            .replace('\\', "\\\\")
            .replace('"', "\\\"")
            .replace('\n', "\\n");

        format!(
            r#"
const readline = require('readline');
const rl = readline.createInterface({{ input: process.stdin, output: process.stdout, terminal: false }});

const context = "{escaped}";

function lines() {{ return context.split("\n"); }}
function head(n = 10) {{ return lines().slice(0, n).join("\n"); }}
function tail(n = 10) {{ return lines().slice(-n).join("\n"); }}
function grep(pattern) {{
    const re = pattern instanceof RegExp ? pattern : new RegExp(pattern, 'gi');
    return lines().filter(l => re.test(l));
}}
function count(pattern) {{
    const re = pattern instanceof RegExp ? pattern : new RegExp(pattern, 'gi');
    return (context.match(re) || []).length;
}}
function FINAL(answer) {{
    console.log("__FINAL__" + String(answer) + "__FINAL_END__");
}}

console.log("READY");

rl.on('line', async (line) => {{
    try {{
        const result = eval(line);
        if (result !== undefined) console.log(result);
    }} catch (e) {{
        console.error("Error:", e.message);
    }}
    console.log("__DONE__");
}});
"#
        )
    }

    /// Execute code and get result
    pub async fn execute(&mut self, code: &str) -> Result<ReplResult> {
        let stdin = self
            .child
            .stdin
            .as_mut()
            .ok_or_else(|| anyhow::anyhow!("No stdin"))?;
        let stdout = self
            .child
            .stdout
            .as_mut()
            .ok_or_else(|| anyhow::anyhow!("No stdout"))?;

        stdin.write_all(code.as_bytes()).await?;
        stdin.write_all(b"\n").await?;
        stdin.flush().await?;

        let mut reader = BufReader::new(stdout);
        let mut output = Vec::new();
        let mut final_answer = None;

        loop {
            let mut line = String::new();
            match timeout(Duration::from_secs(30), reader.read_line(&mut line)).await {
                Ok(Ok(0)) | Err(_) => break, // EOF or timeout
                Ok(Ok(_)) => {
                    let line = line.trim();
                    if line == "__DONE__" {
                        break;
                    }
                    if let Some(answer) = Self::extract_final(line) {
                        final_answer = Some(answer);
                        break;
                    }
                    output.push(line.to_string());
                }
                Ok(Err(e)) => return Err(anyhow::anyhow!("Read error: {}", e)),
            }
        }

        Ok(ReplResult {
            stdout: output.join("\n"),
            stderr: String::new(),
            final_answer,
        })
    }

    fn extract_final(line: &str) -> Option<String> {
        if line.contains("__FINAL__") {
            let start = line.find("__FINAL__")? + 9;
            let end = line.find("__FINAL_END__")?;
            return Some(line[start..end].to_string());
        }
        None
    }

    /// Kill the REPL process
    pub async fn destroy(&mut self) -> Result<()> {
        tracing::debug!(runtime = ?self.runtime, "Destroying external REPL");
        self.child.kill().await?;
        Ok(())
    }

    /// Get the runtime type used by this REPL
    pub fn runtime(&self) -> ReplRuntime {
        self.runtime
    }
}

fn char_index_to_byte_index(value: &str, char_index: usize) -> usize {
    if char_index == 0 {
        return 0;
    }

    value
        .char_indices()
        .nth(char_index)
        .map(|(idx, _)| idx)
        .unwrap_or(value.len())
}

fn truncate_with_ellipsis(value: &str, max_chars: usize) -> String {
    if max_chars == 0 {
        return String::new();
    }

    let mut chars = value.chars();
    let mut output = String::new();
    for _ in 0..max_chars {
        if let Some(ch) = chars.next() {
            output.push(ch);
        } else {
            return value.to_string();
        }
    }

    if chars.next().is_some() {
        format!("{output}...")
    } else {
        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_repl_head_tail() {
        let context = (1..=100)
            .map(|i| format!("line {}", i))
            .collect::<Vec<_>>()
            .join("\n");
        let repl = RlmRepl::new(context, ReplRuntime::Rust);

        let head = repl.head(5);
        assert_eq!(head.len(), 5);
        assert_eq!(head[0], "line 1");

        let tail = repl.tail(5);
        assert_eq!(tail.len(), 5);
        assert_eq!(tail[4], "line 100");
    }

    #[test]
    fn test_repl_grep() {
        let context = "error: something failed\ninfo: all good\nerror: another failure".to_string();
        let repl = RlmRepl::new(context, ReplRuntime::Rust);

        let matches = repl.grep("error");
        assert_eq!(matches.len(), 2);
    }

    #[test]
    fn test_repl_execute_final() {
        let context = "test content".to_string();
        let mut repl = RlmRepl::new(context, ReplRuntime::Rust);

        let result = repl.execute(r#"FINAL("This is the answer")"#);
        assert_eq!(result.final_answer, Some("This is the answer".to_string()));
    }

    #[test]
    fn test_parse_line_numbered_output() {
        let parsed =
            RlmExecutor::parse_line_numbered_output("570:async fn analyze\nL1038:async fn x");
        assert_eq!(parsed.len(), 2);
        assert_eq!(parsed[0], (570, "async fn analyze".to_string()));
        assert_eq!(parsed[1], (1038, "async fn x".to_string()));
    }

    #[test]
    fn test_infer_file_from_query() {
        let file = RlmExecutor::infer_file_from_query(
            "Find all occurrences of 'async fn' in src/rlm/repl.rs",
        );
        assert_eq!(file.as_deref(), Some("src/rlm/repl.rs"));
    }

    #[test]
    fn test_repl_chunks() {
        let context = (1..=100)
            .map(|i| format!("line {}", i))
            .collect::<Vec<_>>()
            .join("\n");
        let repl = RlmRepl::new(context, ReplRuntime::Rust);

        let chunks = repl.chunks(5);
        assert_eq!(chunks.len(), 5);
    }
}
