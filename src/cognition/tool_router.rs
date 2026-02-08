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

use super::thinker::{CandleThinker, ThinkerConfig, ThinkerBackend};

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
            max_tokens: 512,
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
    pub fn from_env() -> Self {
        let enabled = std::env::var("CODETETHER_TOOL_ROUTER_ENABLED")
            .map(|v| matches!(v.as_str(), "1" | "true" | "yes"))
            .unwrap_or(false);

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
                .unwrap_or(512),
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
/// FunctionGemma expects tools as a JSON list in the system turn, followed by
/// the user's intent.  The model produces structured JSON function call output.
fn build_functiongemma_prompt(
    assistant_text: &str,
    tools: &[ToolDefinition],
) -> String {
    // Build tool descriptions as a JSON array for the system section.
    let tool_defs: Vec<serde_json::Value> = tools
        .iter()
        .map(|t| {
            serde_json::json!({
                "name": t.name,
                "description": t.description,
                "parameters": t.parameters,
            })
        })
        .collect();

    let tools_json = serde_json::to_string_pretty(&tool_defs).unwrap_or_else(|_| "[]".to_string());

    // FunctionGemma chat template:
    //   <start_of_turn>system
    //   You are a function calling AI model. ...
    //   <end_of_turn>
    //   <start_of_turn>user
    //   <user intent text>
    //   <end_of_turn>
    //   <start_of_turn>model
    format!(
        "<start_of_turn>system\n\
         You are a function calling AI model. You are provided with function \
         signatures within <tools></tools> XML tags. You may call one or more \
         functions to assist with the user query. Don't make assumptions about \
         what values to plug into functions.\n\n\
         <tools>\n{tools_json}\n</tools>\n\n\
         For each function call return a JSON object with function name and \
         arguments within <tool_call></tool_call> XML tags as follows:\n\
         <tool_call>\n{{\"name\": \"function_name\", \"arguments\": {{\"arg1\": \"value1\"}}}}\n</tool_call>\n\
         <end_of_turn>\n\
         <start_of_turn>user\n\
         {assistant_text}\n\
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
            anyhow!(
                "CODETETHER_TOOL_ROUTER_MODEL_PATH is required when the tool router is enabled"
            )
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
    /// - If the response already contains `ContentPart::ToolCall` entries,
    ///   return it **unchanged** (zero overhead path).
    /// - If the response is text-only, run FunctionGemma to convert the text
    ///   into structured tool calls.
    /// - On any internal error, return the **original** response unchanged
    ///   (safe degradation — the router never breaks existing functionality).
    pub async fn maybe_reformat(
        &self,
        response: CompletionResponse,
        tools: &[ToolDefinition],
    ) -> CompletionResponse {
        // Fast path: if the response already has structured tool calls, pass through.
        let has_tool_calls = response
            .message
            .content
            .iter()
            .any(|p| matches!(p, ContentPart::ToolCall { .. }));

        if has_tool_calls {
            return response;
        }

        // No tools were provided — nothing for FunctionGemma to match against.
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

        // Run FunctionGemma in a blocking thread (CPU-bound).
        match self.run_functiongemma(&assistant_text, tools).await {
            Ok(parsed) if !parsed.is_empty() => {
                tracing::info!(
                    num_calls = parsed.len(),
                    "FunctionGemma router produced tool calls from text-only response"
                );
                self.rewrite_response(response, parsed)
            }
            Ok(_) => {
                // FunctionGemma decided no tool calls are needed — pass through.
                response
            }
            Err(e) => {
                tracing::warn!(
                    error = %e,
                    "FunctionGemma router failed; returning original response"
                );
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
        let prompt = build_functiongemma_prompt(assistant_text, tools);
        let runtime = Arc::clone(&self.runtime);

        let output = tokio::task::spawn_blocking(move || {
            let mut guard = runtime
                .lock()
                .map_err(|_| anyhow!("FunctionGemma mutex poisoned"))?;
            // Use the raw prompt — we've already formatted it with the Gemma chat template.
            // The thinker's `think()` wraps in System/User/Assistant roles; we need direct
            // access to the generation loop.  We pass the full prompt as the user message
            // and an empty system prompt so the thinker doesn't re-wrap it.
            guard.think("", &prompt)
        })
        .await
        .map_err(|e| anyhow!("FunctionGemma task join failed: {e}"))??;

        Ok(parse_functiongemma_response(&output.text))
    }

    /// Rewrite the `CompletionResponse` to include parsed tool calls.
    fn rewrite_response(
        &self,
        mut response: CompletionResponse,
        calls: Vec<ParsedToolCall>,
    ) -> CompletionResponse {
        // Keep existing text parts; append tool call parts.
        for call in calls {
            response.message.content.push(ContentPart::ToolCall {
                id: format!("fc_{}", Uuid::new_v4()),
                name: call.name,
                arguments: call.arguments,
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
        assert_eq!(config.max_tokens, 512);
    }
}
