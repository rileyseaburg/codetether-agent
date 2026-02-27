//! FunctionGemma-powered hybrid tool-call router.
//!
//! Sits between the primary LLM response and the tool-extraction step in the
//! session agentic loop.  When the primary LLM returns text-only output that
//! *describes* tool calls without using structured `ContentPart::ToolCall`
//! entries, the router passes the text + available tool definitions through a
//! local FunctionGemma model (via Candle) and emits properly-formatted
//! `ContentPart::ToolCall` entries.
//!
//! **Feature-gated**: this module only compiles when the `functiongemma` cargo
//! feature is enabled.  The binary size is unaffected in default builds.

use crate::provider::{CompletionResponse, ContentPart, FinishReason, ToolDefinition};
use anyhow::{Result, anyhow};
use std::sync::{Arc, Mutex};
use uuid::Uuid;

use super::thinker::{CandleThinker, ThinkerBackend, ThinkerConfig};

// ── Configuration ────────────────────────────────────────────────────────────

/// Environment-variable driven configuration for the tool-call router.
#[derive(Debug, Clone)]
pub struct ToolRouterConfig {
    /// Whether the router is active.  Default: `false`.
    pub enabled: bool,
    /// Filesystem path to the FunctionGemma GGUF model.
    pub model_path: Option<String>,
    /// Filesystem path to the matching tokenizer.json.
    pub tokenizer_path: Option<String>,
    /// Architecture hint (default: `"gemma3"`).
    pub arch: String,
    /// Device preference (auto / cpu / cuda).
    pub device: super::thinker::CandleDevicePreference,
    /// Max tokens for the FunctionGemma response.
    /// FunctionGemma only outputs `<tool_call>` JSON blocks — 128 is generous.
    pub max_tokens: usize,
    /// Temperature for FunctionGemma sampling.
    pub temperature: f32,
}

impl Default for ToolRouterConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            model_path: None,
            tokenizer_path: None,
            arch: "gemma3".to_string(),
            device: super::thinker::CandleDevicePreference::Auto,
            max_tokens: 128,
            temperature: 0.1,
        }
    }
}

impl ToolRouterConfig {
    /// Build from environment variables.
    ///
    /// | Variable | Description |
    /// |----------|-------------|
    /// | `CODETETHER_TOOL_ROUTER_ENABLED` | `true` / `1` to activate |
    /// | `CODETETHER_TOOL_ROUTER_MODEL_PATH` | Path to `.gguf` model |
    /// | `CODETETHER_TOOL_ROUTER_TOKENIZER_PATH` | Path to `tokenizer.json` |
    /// | `CODETETHER_TOOL_ROUTER_ARCH` | Architecture hint (default: `gemma3`) |
    /// | `CODETETHER_TOOL_ROUTER_DEVICE` | `auto` / `cpu` / `cuda` |
    /// | `CODETETHER_TOOL_ROUTER_MAX_TOKENS` | Max decode tokens (default: 512) |
    /// | `CODETETHER_TOOL_ROUTER_TEMPERATURE` | Sampling temp (default: 0.1) |
    /// | `CODETETHER_FUNCTIONGEMMA_DISABLED` | Emergency kill switch. Defaults to `true` |
    pub fn from_env() -> Self {
        let enabled_requested = std::env::var("CODETETHER_TOOL_ROUTER_ENABLED")
            .map(|v| matches!(v.as_str(), "1" | "true" | "yes"))
            .unwrap_or(false);

        // Temporary safety default: keep FunctionGemma disabled unless explicitly
        // unblocked. This prevents local CPU/GPU contention in normal CLI/TUI runs.
        let disabled = std::env::var("CODETETHER_FUNCTIONGEMMA_DISABLED")
            .map(|v| matches!(v.as_str(), "1" | "true" | "yes"))
            .unwrap_or(true);

        let enabled = enabled_requested && !disabled;

        Self {
            enabled,
            model_path: std::env::var("CODETETHER_TOOL_ROUTER_MODEL_PATH").ok(),
            tokenizer_path: std::env::var("CODETETHER_TOOL_ROUTER_TOKENIZER_PATH").ok(),
            arch: std::env::var("CODETETHER_TOOL_ROUTER_ARCH")
                .unwrap_or_else(|_| "gemma3".to_string()),
            device: std::env::var("CODETETHER_TOOL_ROUTER_DEVICE")
                .map(|v| super::thinker::CandleDevicePreference::from_env(&v))
                .unwrap_or(super::thinker::CandleDevicePreference::Auto),
            max_tokens: std::env::var("CODETETHER_TOOL_ROUTER_MAX_TOKENS")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(256),
            temperature: std::env::var("CODETETHER_TOOL_ROUTER_TEMPERATURE")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(0.1),
        }
    }
}

// ── Prompt formatting ────────────────────────────────────────────────────────

/// Serialize tool definitions into FunctionGemma's expected chat template.
///
/// The prompt frames FunctionGemma as a tool-call extractor: given an LLM's
/// text response that *describes* tool usage, produce the corresponding
/// structured `<tool_call>` blocks.
fn build_functiongemma_prompt(assistant_text: &str, tools: &[ToolDefinition]) -> String {
    // Build compact tool descriptions — name + description only (skip full
    // parameter schemas to keep prompt tokens low for the 270M model).
    let tool_lines: String = tools
        .iter()
        .map(|t| format!("- {}: {}", t.name, t.description))
        .collect::<Vec<_>>()
        .join("\n");

    // Build full definitions only for first 5 tools (most likely candidates).
    let detailed_defs: Vec<serde_json::Value> = tools
        .iter()
        .take(5)
        .map(|t| {
            serde_json::json!({
                "name": t.name,
                "description": t.description,
                "parameters": t.parameters,
            })
        })
        .collect();

    let tools_json =
        serde_json::to_string_pretty(&detailed_defs).unwrap_or_else(|_| "[]".to_string());

    // Truncate assistant text to first ~500 chars — the intent is expressed
    // early; the rest is often hallucinated output or markdown formatting.
    let truncated = if assistant_text.len() > 500 {
        &assistant_text[..500]
    } else {
        assistant_text
    };

    format!(
        "<start_of_turn>system\n\
         You are a function calling AI model. You are provided with function \
         signatures within <tools></tools> XML tags. You may call one or more \
         functions to assist with the user query. Don't make assumptions about \
         what values to plug into functions.\n\n\
         <tools>\n{tools_json}\n</tools>\n\n\
         Other available tools:\n{tool_lines}\n\n\
         For each function call return a JSON object with function name and \
         arguments within <tool_call></tool_call> XML tags as follows:\n\
         <tool_call>\n{{\"name\": \"function_name\", \"arguments\": {{\"arg1\": \"value1\"}}}}\n</tool_call>\n\
         <end_of_turn>\n\
         <start_of_turn>user\n\
         The following is an AI assistant's response. It describes wanting to \
         use tools but expressed them as text instead of structured calls. \
         Extract the tool calls the assistant intended to make:\n\n\
         {truncated}\n\
         <end_of_turn>\n\
         <start_of_turn>model\n"
    )
}

// ── Response parsing ─────────────────────────────────────────────────────────

/// A single parsed tool call from FunctionGemma output.
#[derive(Debug, Clone)]
struct ParsedToolCall {
    name: String,
    arguments: String, // JSON string
}

/// Parse FunctionGemma output into zero or more structured tool calls.
///
/// Expected format:
/// ```text
/// <tool_call>
/// {"name": "read_file", "arguments": {"path": "/tmp/foo.rs"}}
/// </tool_call>
/// ```
///
/// Handles multiple `<tool_call>` blocks in a single response.
fn parse_functiongemma_response(text: &str) -> Vec<ParsedToolCall> {
    let mut calls = Vec::new();

    // Extract everything between <tool_call> and </tool_call>
    let mut remaining = text;
    while let Some(start) = remaining.find("<tool_call>") {
        remaining = &remaining[start + "<tool_call>".len()..];
        if let Some(end) = remaining.find("</tool_call>") {
            let block = remaining[..end].trim();
            remaining = &remaining[end + "</tool_call>".len()..];

            // Try to parse the JSON block
            if let Ok(value) = serde_json::from_str::<serde_json::Value>(block) {
                let name = value
                    .get("name")
                    .and_then(|n| n.as_str())
                    .unwrap_or("")
                    .to_string();
                let arguments = value
                    .get("arguments")
                    .map(|a| serde_json::to_string(a).unwrap_or_else(|_| "{}".to_string()))
                    .unwrap_or_else(|| "{}".to_string());

                if !name.is_empty() {
                    calls.push(ParsedToolCall { name, arguments });
                }
            } else {
                tracing::warn!(
                    block = %block,
                    "FunctionGemma produced unparseable tool_call block"
                );
            }
        } else {
            break; // Unclosed <tool_call> — stop
        }
    }

    calls
}

// ── Router ───────────────────────────────────────────────────────────────────

/// Hybrid tool-call router backed by a local FunctionGemma model.
///
/// Created once at session start; shared via `Arc` across prompt calls.
pub struct ToolCallRouter {
    runtime: Arc<Mutex<CandleThinker>>,
}

impl std::fmt::Debug for ToolCallRouter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ToolCallRouter").finish()
    }
}

impl ToolCallRouter {
    /// Construct from a [`ToolRouterConfig`].
    ///
    /// Returns `None` if the router is disabled or missing required paths.
    pub fn from_config(config: &ToolRouterConfig) -> Result<Option<Self>> {
        if !config.enabled {
            tracing::debug!("FunctionGemma tool router is disabled");
            return Ok(None);
        }

        let model_path = config.model_path.as_ref().ok_or_else(|| {
            anyhow!("CODETETHER_TOOL_ROUTER_MODEL_PATH is required when the tool router is enabled")
        })?;
        let tokenizer_path = config.tokenizer_path.as_ref().ok_or_else(|| {
            anyhow!(
                "CODETETHER_TOOL_ROUTER_TOKENIZER_PATH is required when the tool router is enabled"
            )
        })?;

        // Build a ThinkerConfig configured for the FunctionGemma model
        let thinker_config = ThinkerConfig {
            enabled: true,
            backend: ThinkerBackend::Candle,
            candle_model_path: Some(model_path.clone()),
            candle_tokenizer_path: Some(tokenizer_path.clone()),
            candle_arch: Some(config.arch.clone()),
            candle_device: config.device,
            max_tokens: config.max_tokens,
            temperature: config.temperature,
            ..ThinkerConfig::default()
        };

        let runtime = CandleThinker::new(&thinker_config)?;
        tracing::info!(
            model_path = %model_path,
            arch = %config.arch,
            "FunctionGemma tool-call router initialised"
        );

        Ok(Some(Self {
            runtime: Arc::new(Mutex::new(runtime)),
        }))
    }

    /// Conditionally reformat a `CompletionResponse`.
    ///
    /// - If the model natively supports tool calling, return **unchanged**.
    /// - If the response already contains `ContentPart::ToolCall` entries,
    ///   return it **unchanged**.
    /// - First, try to parse `<tool_call>` blocks directly from the text
    ///   (zero-cost; works when the model was prompted to output them).
    /// - If no direct parse, fall back to FunctionGemma inference.
    /// - On any internal error, return the **original** response unchanged.
    pub async fn maybe_reformat(
        &self,
        response: CompletionResponse,
        tools: &[ToolDefinition],
        model_supports_tools: bool,
    ) -> CompletionResponse {
        if model_supports_tools {
            tracing::trace!("Skipping tool router: model supports native tool calling");
            return response;
        }

        // Fast path: if the response already has structured tool calls, pass through.
        let has_tool_calls = response
            .message
            .content
            .iter()
            .any(|p| matches!(p, ContentPart::ToolCall { .. }));

        if has_tool_calls {
            return response;
        }

        // No tools were provided — nothing to match against.
        if tools.is_empty() {
            return response;
        }

        // Collect assistant text from the response.
        let assistant_text: String = response
            .message
            .content
            .iter()
            .filter_map(|p| match p {
                ContentPart::Text { text } => Some(text.as_str()),
                _ => None,
            })
            .collect::<Vec<_>>()
            .join("\n");

        if assistant_text.trim().is_empty() {
            return response;
        }

        // ── Direct parse: try to extract <tool_call> blocks from the text. ──
        // This is zero-cost and works when the system prompt instructed the
        // model to output structured tool calls.
        let direct_calls = parse_functiongemma_response(&assistant_text);
        if !direct_calls.is_empty() {
            // Validate that parsed tool names actually exist in the tool set.
            let valid_calls: Vec<ParsedToolCall> = direct_calls
                .into_iter()
                .filter(|c| tools.iter().any(|t| t.name == c.name))
                .collect();
            if !valid_calls.is_empty() {
                tracing::info!(
                    num_calls = valid_calls.len(),
                    "Direct parse extracted tool calls from text response"
                );
                return self.rewrite_response(response, valid_calls);
            }
        }

        // ── Fallback: FunctionGemma inference (GPU ~4s, CPU ~65s). ──
        tracing::info!(
            num_tools = tools.len(),
            "Running FunctionGemma tool extraction ({} tool definitions)",
            tools.len()
        );

        let fut = self.run_functiongemma(&assistant_text, tools);
        match tokio::time::timeout(std::time::Duration::from_secs(90), fut).await {
            Ok(Ok(parsed)) if !parsed.is_empty() => {
                tracing::info!(
                    num_calls = parsed.len(),
                    "FunctionGemma router produced tool calls from text-only response"
                );
                self.rewrite_response(response, parsed)
            }
            Ok(Ok(_)) => {
                // FunctionGemma decided no tool calls are needed — pass through.
                response
            }
            Ok(Err(e)) => {
                tracing::warn!(
                    error = %e,
                    "FunctionGemma router failed; returning original response"
                );
                response
            }
            Err(_elapsed) => {
                tracing::warn!("FunctionGemma timed out after 90s; returning original response");
                response
            }
        }
    }

    /// Run the FunctionGemma model in a blocking thread.
    async fn run_functiongemma(
        &self,
        assistant_text: &str,
        tools: &[ToolDefinition],
    ) -> Result<Vec<ParsedToolCall>> {
        // Sort tools so that ones whose names appear in the text come first —
        // build_functiongemma_prompt gives the first 5 full parameter schemas.
        let text_lower = assistant_text.to_lowercase();
        let mut sorted_tools: Vec<ToolDefinition> = tools.to_vec();
        sorted_tools.sort_by_key(|t| {
            if text_lower.contains(&t.name.to_lowercase()) {
                0
            } else {
                1
            }
        });
        // Also prioritize common action tools (bash, read, write, grep, list)
        // that models often describe in text without naming explicitly.
        let action_tools = ["bash", "read", "write", "grep", "list", "search"];
        sorted_tools.sort_by_key(|t| {
            let name_lower = t.name.to_lowercase();
            if text_lower.contains(&name_lower) {
                0
            } else if action_tools.iter().any(|a| name_lower.contains(a)) {
                1
            } else {
                2
            }
        });

        let prompt = build_functiongemma_prompt(assistant_text, &sorted_tools);
        tracing::debug!(prompt_len = prompt.len(), "FunctionGemma prompt built");
        let runtime = Arc::clone(&self.runtime);

        let output = tokio::task::spawn_blocking(move || {
            let mut guard = runtime
                .lock()
                .map_err(|_| anyhow!("FunctionGemma mutex poisoned"))?;
            // Use think_raw — the prompt is already formatted with the Gemma
            // chat template by build_functiongemma_prompt.
            guard.think_raw(&prompt)
        })
        .await
        .map_err(|e| anyhow!("FunctionGemma task join failed: {e}"))??;

        tracing::debug!(
            raw_output = %output.text,
            completion_tokens = output.completion_tokens.unwrap_or(0),
            "FunctionGemma raw output"
        );

        Ok(parse_functiongemma_response(&output.text))
    }

    /// Rewrite the `CompletionResponse` to replace text with structured tool calls.
    ///
    /// The original text is **removed** so the model sees a pure tool-call
    /// assistant turn.  On the follow-up turn it will receive the tool results
    /// and compose a proper answer – rather than ignoring them because it
    /// already gave a complete text response.
    fn rewrite_response(
        &self,
        mut response: CompletionResponse,
        calls: Vec<ParsedToolCall>,
    ) -> CompletionResponse {
        // Strip <tool_call>...</tool_call> blocks from text parts but keep
        // the surrounding reasoning text so the model retains its chain of
        // thought on subsequent turns.
        let re = regex::Regex::new(r"(?s)<tool_call>.*?</tool_call>").unwrap();
        for part in &mut response.message.content {
            if let ContentPart::Text { text } = part {
                let cleaned = re.replace_all(text, "").trim().to_string();
                *text = cleaned;
            }
        }
        // Remove now-empty text parts.
        response
            .message
            .content
            .retain(|p| !matches!(p, ContentPart::Text { text } if text.is_empty()));

        for call in calls {
            response.message.content.push(ContentPart::ToolCall {
                id: format!("fc_{}", Uuid::new_v4()),
                name: call.name,
                arguments: call.arguments,
                thought_signature: None,
            });
        }

        // Signal the session loop that tool calls are present.
        response.finish_reason = FinishReason::ToolCalls;
        response
    }
}

// ── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_single_tool_call() {
        let text = r#"<tool_call>
{"name": "read_file", "arguments": {"path": "/tmp/foo.rs"}}
</tool_call>"#;
        let calls = parse_functiongemma_response(text);
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].name, "read_file");
        assert!(calls[0].arguments.contains("/tmp/foo.rs"));
    }

    #[test]
    fn parse_multiple_tool_calls() {
        let text = r#"I'll read both files.
<tool_call>
{"name": "read_file", "arguments": {"path": "a.rs"}}
</tool_call>
<tool_call>
{"name": "read_file", "arguments": {"path": "b.rs"}}
</tool_call>"#;
        let calls = parse_functiongemma_response(text);
        assert_eq!(calls.len(), 2);
        assert_eq!(calls[0].name, "read_file");
        assert_eq!(calls[1].name, "read_file");
    }

    #[test]
    fn parse_no_tool_calls() {
        let text = "I cannot help with that request.";
        let calls = parse_functiongemma_response(text);
        assert!(calls.is_empty());
    }

    #[test]
    fn parse_malformed_json_skipped() {
        let text = r#"<tool_call>
not valid json
</tool_call>
<tool_call>
{"name": "list_dir", "arguments": {"path": "."}}
</tool_call>"#;
        let calls = parse_functiongemma_response(text);
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].name, "list_dir");
    }

    #[test]
    fn parse_empty_name_skipped() {
        let text = r#"<tool_call>
{"name": "", "arguments": {}}
</tool_call>"#;
        let calls = parse_functiongemma_response(text);
        assert!(calls.is_empty());
    }

    #[test]
    fn prompt_contains_tool_definitions() {
        let tools = vec![ToolDefinition {
            name: "read_file".to_string(),
            description: "Read a file".to_string(),
            parameters: serde_json::json!({
                "type": "object",
                "properties": {
                    "path": { "type": "string" }
                },
                "required": ["path"]
            }),
        }];
        let prompt = build_functiongemma_prompt("Please read foo.rs", &tools);
        assert!(prompt.contains("<start_of_turn>system"));
        assert!(prompt.contains("read_file"));
        assert!(prompt.contains("<tools>"));
        assert!(prompt.contains("Please read foo.rs"));
        assert!(prompt.contains("<start_of_turn>model"));
    }

    #[test]
    fn config_defaults_disabled() {
        let config = ToolRouterConfig::default();
        assert!(!config.enabled);
        assert_eq!(config.arch, "gemma3");
        assert_eq!(config.max_tokens, 128);
    }
}
