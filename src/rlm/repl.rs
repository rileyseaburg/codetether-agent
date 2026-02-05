//! RLM REPL - Execution environment for RLM processing
//!
//! Provides a REPL-like environment where context is loaded as a variable
//! and the LLM can execute code to analyze it.
//!
//! Key feature: llm_query() function for recursive sub-LM calls.

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
        let end = end.min(self.context.len());
        let start = start.min(end);
        &self.context[start..end]
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
    /// - FINAL("answer") - return final answer
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
        if let Some(eq_pos) = line.find('=') {
            if !line.contains("==") && !line.starts_with("if ") {
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
        }

        // Check for function calls that should output
        if line.starts_with("head(")
            || line.starts_with("tail(")
            || line.starts_with("grep(")
            || line.starts_with("count(")
            || line.starts_with("lines()")
            || line.starts_with("slice(")
            || line.starts_with("chunks(")
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

        // Variable reference
        if let Some(val) = self.get_var(expr) {
            return val.to_string();
        }

        // String literal
        if (expr.starts_with('"') && expr.ends_with('"'))
            || (expr.starts_with('\'') && expr.ends_with('\''))
        {
            return expr[1..expr.len() - 1].to_string();
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
pub struct RlmExecutor {
    repl: RlmRepl,
    provider: Arc<dyn Provider>,
    model: String,
    max_iterations: usize,
    sub_queries: Vec<SubQuery>,
    verbose: bool,
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
        Self {
            repl: RlmRepl::new(context, ReplRuntime::Rust),
            provider,
            model,
            max_iterations: 5, // Keep iterations limited for speed
            sub_queries: Vec::new(),
            verbose: false,
        }
    }

    /// Set maximum iterations
    pub fn with_max_iterations(mut self, max: usize) -> Self {
        self.max_iterations = max;
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

    /// Execute RLM analysis with the given query
    pub async fn analyze(&mut self, query: &str) -> Result<RlmAnalysisResult> {
        let start = std::time::Instant::now();
        let mut iterations = 0;
        let mut total_input_tokens = 0;
        let mut total_output_tokens = 0;

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
             - llm_query(\"question\", context?) - ask sub-LM a question\n\
             - FINAL(\"answer\") - return final answer\n\
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
             IMPORTANT: You MUST end your response with FINAL(\"your answer\") in 1-3 iterations.\n\n\
             Available commands:\n\
             - head(n), tail(n): See first/last n lines\n\
             - grep(\"pattern\"): Search for patterns\n\
             - llm_query(\"question\"): Ask a focused sub-question\n\
             - FINAL(\"answer\"): Return your final answer (REQUIRED)\n\n\
             The context has {} chars across {} lines. A preview follows:\n\n\
             {}\n\n\
             Now analyze the context. Use 1-2 commands if needed, then call FINAL() with your answer.",
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

        let mut final_answer = None;

        while iterations < self.max_iterations {
            iterations += 1;
            tracing::info!("RLM iteration {}", iterations);

            // Get LLM response with code to execute (with timeout)
            tracing::debug!("Sending LLM request...");
            let response = match tokio::time::timeout(
                std::time::Duration::from_secs(60),
                self.provider.complete(CompletionRequest {
                    messages: messages.clone(),
                    tools: vec![],
                    model: self.model.clone(),
                    temperature: Some(0.3),
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
                Err(_) => return Err(anyhow::anyhow!("LLM request timed out after 60 seconds")),
            };

            total_input_tokens += response.usage.prompt_tokens;
            total_output_tokens += response.usage.completion_tokens;

            // Extract code from response
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
                    let preview = if execution_result.stdout.len() > 200 {
                        format!("{}...", &execution_result.stdout[..200])
                    } else {
                        execution_result.stdout.clone()
                    };
                    println!("[RLM] Execution output:\n{}", preview);
                }
            }

            // Check for final answer
            if let Some(answer) = &execution_result.final_answer {
                final_answer = Some(answer.clone());
                break;
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

        Ok(RlmAnalysisResult {
            answer: final_answer.unwrap_or_else(|| "Analysis incomplete".to_string()),
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

        // Get the context to send
        let context_to_analyze = context_slice
            .clone()
            .unwrap_or_else(|| self.repl.context().to_string());

        // Truncate context for sub-query to avoid overwhelming the LLM
        let truncated_context = if context_to_analyze.len() > 8000 {
            format!(
                "{}...\n[truncated, {} chars total]",
                &context_to_analyze[..7500],
                context_to_analyze.len()
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
