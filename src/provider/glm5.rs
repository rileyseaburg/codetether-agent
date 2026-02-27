//! GLM-5 FP8 provider for Vast.ai serverless deployments
//!
//! This provider connects to a self-hosted vLLM endpoint running GLM-5 FP8
//! on Vast.ai serverless infrastructure. The endpoint exposes an OpenAI-compatible
//! chat completions API (vLLM default).
//!
//! ## Model: GLM-5-FP8
//!
//! - **Architecture**: 744B parameter Mixture-of-Experts (MoE)
//! - **Active Parameters**: 40B per forward pass
//! - **Quantization**: FP8 for efficient inference
//! - **Hardware**: 8x A100 SXM4 80GB
//! - **Features**: MTP speculative decoding enabled
//!
//! ## Configuration (Vault)
//!
//! Store under `secret/data/codetether/providers/glm5`:
//! ```json
//! {
//!   "api_key": "<vast-endpoint-api-key>",
//!   "base_url": "https://route.vast.ai/<endpoint-id>/<api-key>/v1",
//!   "extra": {
//!     "model_name": "glm-5-fp8"
//!   }
//! }
//! ```
//!
//! ## Model reference format
//!
//! Use `glm5/glm-5-fp8`, `glm5/glm-5`, or just `glm-5-fp8` as the model string.
//!
//! ## Environment variable fallback
//!
//! - `GLM5_API_KEY` — API key for the Vast.ai endpoint
//! - `GLM5_BASE_URL` — Base URL of the vLLM endpoint (required)
//! - `GLM5_MODEL` — Model name override (default: glm-5-fp8)

use super::{
    CompletionRequest, CompletionResponse, ContentPart, FinishReason, Message, ModelInfo, Provider,
    Role, StreamChunk, ToolDefinition, Usage,
};
use anyhow::{Context, Result};
use async_trait::async_trait;
use futures::StreamExt;
use reqwest::Client;
use serde::Deserialize;
use serde_json::{Value, json};

/// Default model name served by the Vast.ai vLLM endpoint.
pub const DEFAULT_MODEL: &str = "glm-5-fp8";

/// GLM-5 FP8 provider targeting a Vast.ai vLLM serverless endpoint.
pub struct Glm5Provider {
    client: Client,
    api_key: String,
    base_url: String,
    /// The actual model name to send in API requests (e.g. "glm-5-fp8").
    model_name: String,
}

impl std::fmt::Debug for Glm5Provider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Glm5Provider")
            .field("base_url", &self.base_url)
            .field("model_name", &self.model_name)
            .field("api_key", &"<REDACTED>")
            .finish()
    }
}

impl Glm5Provider {
    /// Create a new Glm5Provider pointing at `base_url` with `api_key`.
    ///
    /// `base_url` should be the full base URL including `/v1`, e.g.
    /// `https://<vast-endpoint>.vast.ai/v1`.
    pub fn new(api_key: String, base_url: String) -> Result<Self> {
        Self::with_model(api_key, base_url, DEFAULT_MODEL.to_string())
    }

    /// Create with an explicit model name override.
    pub fn with_model(api_key: String, base_url: String, model_name: String) -> Result<Self> {
        let base_url = base_url.trim_end_matches('/').to_string();
        tracing::debug!(
            provider = "glm5",
            base_url = %base_url,
            model = %model_name,
            "Creating GLM-5 FP8 provider"
        );
        Ok(Self {
            client: Client::new(),
            api_key,
            base_url,
            model_name,
        })
    }

    /// Normalize a model string: strip provider prefix (`glm5/`, `glm5:`) and
    /// map aliases to the canonical model name served by vLLM.
    ///
    /// Examples:
    /// - `"glm5/glm-5-fp8"` → `"glm-5-fp8"`
    /// - `"glm5:glm-5-fp8"` → `"glm-5-fp8"`
    /// - `"glm-5-fp8"`      → `"glm-5-fp8"`
    /// - `"glm-5"`          → `"glm-5-fp8"` (alias to fp8 variant)
    /// - `""`               → `DEFAULT_MODEL`
    pub fn normalize_model(model: &str) -> String {
        let stripped = model
            .trim()
            .trim_start_matches("glm5/")
            .trim_start_matches("glm5:");

        match stripped {
            "" | "glm-5" | "glm5" => DEFAULT_MODEL.to_string(),
            other => other.to_string(),
        }
    }

    fn convert_messages(messages: &[Message]) -> Vec<Value> {
        messages
            .iter()
            .map(|msg| {
                let role = match msg.role {
                    Role::System => "system",
                    Role::User => "user",
                    Role::Assistant => "assistant",
                    Role::Tool => "tool",
                };

                match msg.role {
                    Role::Tool => {
                        if let Some(ContentPart::ToolResult {
                            tool_call_id,
                            content,
                        }) = msg.content.first()
                        {
                            json!({
                                "role": "tool",
                                "tool_call_id": tool_call_id,
                                "content": content
                            })
                        } else {
                            json!({"role": role, "content": ""})
                        }
                    }
                    Role::Assistant => {
                        let text: String = msg
                            .content
                            .iter()
                            .filter_map(|p| match p {
                                ContentPart::Text { text } => Some(text.clone()),
                                _ => None,
                            })
                            .collect::<Vec<_>>()
                            .join("");

                        let tool_calls: Vec<Value> = msg
                            .content
                            .iter()
                            .filter_map(|p| match p {
                                ContentPart::ToolCall {
                                    id,
                                    name,
                                    arguments,
                                    ..
                                } => {
                                    // vLLM expects arguments as a JSON string
                                    let args_string = serde_json::from_str::<Value>(arguments)
                                        .map(|parsed| {
                                            serde_json::to_string(&parsed)
                                                .unwrap_or_else(|_| "{}".to_string())
                                        })
                                        .unwrap_or_else(|_| {
                                            json!({"input": arguments}).to_string()
                                        });
                                    Some(json!({
                                        "id": id,
                                        "type": "function",
                                        "function": {
                                            "name": name,
                                            "arguments": args_string
                                        }
                                    }))
                                }
                                _ => None,
                            })
                            .collect();

                        let mut msg_json = json!({
                            "role": "assistant",
                            "content": if text.is_empty() { Value::Null } else { json!(text) },
                        });
                        if !tool_calls.is_empty() {
                            msg_json["tool_calls"] = json!(tool_calls);
                        }
                        msg_json
                    }
                    _ => {
                        let text: String = msg
                            .content
                            .iter()
                            .filter_map(|p| match p {
                                ContentPart::Text { text } => Some(text.clone()),
                                _ => None,
                            })
                            .collect::<Vec<_>>()
                            .join("\n");

                        json!({"role": role, "content": text})
                    }
                }
            })
            .collect()
    }

    fn convert_tools(tools: &[ToolDefinition]) -> Vec<Value> {
        tools
            .iter()
            .map(|t| {
                json!({
                    "type": "function",
                    "function": {
                        "name": t.name,
                        "description": t.description,
                        "parameters": t.parameters
                    }
                })
            })
            .collect()
    }

    fn preview_text(text: &str, max_chars: usize) -> &str {
        if max_chars == 0 {
            return "";
        }
        if let Some((idx, _)) = text.char_indices().nth(max_chars) {
            &text[..idx]
        } else {
            text
        }
    }
}

// ─── Response deserialization ────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
struct Glm5Response {
    choices: Vec<Glm5Choice>,
    #[serde(default)]
    usage: Option<Glm5Usage>,
}

#[derive(Debug, Deserialize)]
struct Glm5Choice {
    message: Glm5Message,
    #[serde(default)]
    finish_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Glm5Message {
    #[serde(default)]
    content: Option<String>,
    #[serde(default)]
    tool_calls: Option<Vec<Glm5ToolCall>>,
}

#[derive(Debug, Deserialize)]
struct Glm5ToolCall {
    id: String,
    function: Glm5Function,
}

#[derive(Debug, Deserialize)]
struct Glm5Function {
    name: String,
    arguments: Value,
}

#[derive(Debug, Deserialize)]
struct Glm5Usage {
    #[serde(default)]
    prompt_tokens: usize,
    #[serde(default)]
    completion_tokens: usize,
    #[serde(default)]
    total_tokens: usize,
}

#[derive(Debug, Deserialize)]
struct Glm5Error {
    error: Glm5ErrorDetail,
}

#[derive(Debug, Deserialize)]
struct Glm5ErrorDetail {
    message: String,
    #[serde(default, rename = "type")]
    error_type: Option<String>,
}

// ─── SSE stream types ────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
struct Glm5StreamResponse {
    choices: Vec<Glm5StreamChoice>,
    #[serde(default)]
    usage: Option<Glm5Usage>,
}

#[derive(Debug, Deserialize)]
struct Glm5StreamChoice {
    delta: Glm5StreamDelta,
    #[serde(default)]
    finish_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Glm5StreamDelta {
    #[serde(default)]
    content: Option<String>,
    #[serde(default)]
    tool_calls: Option<Vec<Glm5StreamToolCall>>,
}

#[derive(Debug, Deserialize)]
struct Glm5StreamToolCall {
    #[serde(default)]
    id: Option<String>,
    function: Option<Glm5StreamFunction>,
}

#[derive(Debug, Deserialize)]
struct Glm5StreamFunction {
    #[serde(default)]
    name: Option<String>,
    #[serde(default)]
    arguments: Option<Value>,
}

// ─── Provider impl ───────────────────────────────────────────────────────────

#[async_trait]
impl Provider for Glm5Provider {
    fn name(&self) -> &str {
        "glm5"
    }

    async fn list_models(&self) -> Result<Vec<ModelInfo>> {
        Ok(vec![
            ModelInfo {
                id: "glm-5-fp8".to_string(),
                name: "GLM-5 FP8 (744B MoE, 40B active)".to_string(),
                provider: "glm5".to_string(),
                context_window: 128_000,
                max_output_tokens: Some(16_384),
                supports_vision: false,
                supports_tools: true,
                supports_streaming: true,
                input_cost_per_million: None,
                output_cost_per_million: None,
            },
            ModelInfo {
                id: "glm-5".to_string(),
                name: "GLM-5 (alias → FP8)".to_string(),
                provider: "glm5".to_string(),
                context_window: 128_000,
                max_output_tokens: Some(16_384),
                supports_vision: false,
                supports_tools: true,
                supports_streaming: true,
                input_cost_per_million: None,
                output_cost_per_million: None,
            },
        ])
    }

    async fn complete(&self, request: CompletionRequest) -> Result<CompletionResponse> {
        let messages = Self::convert_messages(&request.messages);
        let tools = Self::convert_tools(&request.tools);

        // GLM-5 performs well at temperature 1.0
        let temperature = request.temperature.unwrap_or(1.0);
        // Resolve model alias
        let model = Self::normalize_model(&request.model);

        let mut body = json!({
            "model": model,
            "messages": messages,
            "temperature": temperature,
        });

        if !tools.is_empty() {
            body["tools"] = json!(tools);
        }
        if let Some(max) = request.max_tokens {
            body["max_tokens"] = json!(max);
        }

        tracing::debug!(model = %model, endpoint = %self.base_url, "GLM-5 FP8 request");

        let response = self
            .client
            .post(format!("{}/chat/completions", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .context("Failed to send request to GLM-5 FP8 endpoint")?;

        let status = response.status();
        let text = response
            .text()
            .await
            .context("Failed to read GLM-5 FP8 response")?;

        if !status.is_success() {
            if let Ok(err) = serde_json::from_str::<Glm5Error>(&text) {
                anyhow::bail!(
                    "GLM-5 FP8 API error: {} ({:?})",
                    err.error.message,
                    err.error.error_type
                );
            }
            anyhow::bail!("GLM-5 FP8 API error: {} {}", status, text);
        }

        let parsed: Glm5Response = serde_json::from_str(&text).context(format!(
            "Failed to parse GLM-5 FP8 response: {}",
            Self::preview_text(&text, 200)
        ))?;

        let choice = parsed
            .choices
            .first()
            .ok_or_else(|| anyhow::anyhow!("No choices in GLM-5 FP8 response"))?;

        let mut content = Vec::new();
        let mut has_tool_calls = false;

        if let Some(text) = &choice.message.content
            && !text.is_empty()
        {
            content.push(ContentPart::Text { text: text.clone() });
        }

        if let Some(tool_calls) = &choice.message.tool_calls {
            has_tool_calls = !tool_calls.is_empty();
            for tc in tool_calls {
                let arguments = match &tc.function.arguments {
                    Value::String(s) => s.clone(),
                    other => serde_json::to_string(other).unwrap_or_default(),
                };
                content.push(ContentPart::ToolCall {
                    id: tc.id.clone(),
                    name: tc.function.name.clone(),
                    arguments,
                    thought_signature: None,
                });
            }
        }

        let finish_reason = if has_tool_calls {
            FinishReason::ToolCalls
        } else {
            match choice.finish_reason.as_deref() {
                Some("stop") => FinishReason::Stop,
                Some("length") => FinishReason::Length,
                Some("tool_calls") => FinishReason::ToolCalls,
                _ => FinishReason::Stop,
            }
        };

        Ok(CompletionResponse {
            message: Message {
                role: Role::Assistant,
                content,
            },
            usage: Usage {
                prompt_tokens: parsed.usage.as_ref().map(|u| u.prompt_tokens).unwrap_or(0),
                completion_tokens: parsed
                    .usage
                    .as_ref()
                    .map(|u| u.completion_tokens)
                    .unwrap_or(0),
                total_tokens: parsed.usage.as_ref().map(|u| u.total_tokens).unwrap_or(0),
                cache_read_tokens: None,
                cache_write_tokens: None,
            },
            finish_reason,
        })
    }

    async fn complete_stream(
        &self,
        request: CompletionRequest,
    ) -> Result<futures::stream::BoxStream<'static, StreamChunk>> {
        let messages = Self::convert_messages(&request.messages);
        let tools = Self::convert_tools(&request.tools);

        let temperature = request.temperature.unwrap_or(1.0);
        let model = Self::normalize_model(&request.model);

        let mut body = json!({
            "model": model,
            "messages": messages,
            "temperature": temperature,
            "stream": true,
            "stream_options": { "include_usage": true },
        });

        if !tools.is_empty() {
            body["tools"] = json!(tools);
        }
        if let Some(max) = request.max_tokens {
            body["max_tokens"] = json!(max);
        }

        tracing::debug!(
            model = %model,
            endpoint = %self.base_url,
            "GLM-5 FP8 streaming request"
        );

        let response = self
            .client
            .post(format!("{}/chat/completions", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .context("Failed to send streaming request to GLM-5 FP8 endpoint")?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            if let Ok(err) = serde_json::from_str::<Glm5Error>(&text) {
                anyhow::bail!(
                    "GLM-5 FP8 API error: {} ({:?})",
                    err.error.message,
                    err.error.error_type
                );
            }
            anyhow::bail!("GLM-5 FP8 streaming error: {} {}", status, text);
        }

        let stream = response.bytes_stream();
        let mut buffer = String::new();

        Ok(stream
            .flat_map(move |chunk_result| {
                let mut chunks: Vec<StreamChunk> = Vec::new();
                match chunk_result {
                    Ok(bytes) => {
                        let text = String::from_utf8_lossy(&bytes);
                        buffer.push_str(&text);

                        let mut text_buf = String::new();
                        while let Some(line_end) = buffer.find('\n') {
                            let line = buffer[..line_end].trim().to_string();
                            buffer = buffer[line_end + 1..].to_string();

                            if line == "data: [DONE]" {
                                if !text_buf.is_empty() {
                                    chunks.push(StreamChunk::Text(std::mem::take(&mut text_buf)));
                                }
                                chunks.push(StreamChunk::Done { usage: None });
                                continue;
                            }

                            if let Some(data) = line.strip_prefix("data: ")
                                && let Ok(parsed) = serde_json::from_str::<Glm5StreamResponse>(data)
                            {
                                // Capture usage from the final chunk (stream_options)
                                let usage = parsed.usage.as_ref().map(|u| Usage {
                                    prompt_tokens: u.prompt_tokens,
                                    completion_tokens: u.completion_tokens,
                                    total_tokens: u.total_tokens,
                                    cache_read_tokens: None,
                                    cache_write_tokens: None,
                                });

                                if let Some(choice) = parsed.choices.first() {
                                    if let Some(ref content) = choice.delta.content {
                                        text_buf.push_str(content);
                                    }

                                    // Streaming tool calls
                                    if let Some(ref tool_calls) = choice.delta.tool_calls {
                                        if !text_buf.is_empty() {
                                            chunks.push(StreamChunk::Text(std::mem::take(
                                                &mut text_buf,
                                            )));
                                        }
                                        for tc in tool_calls {
                                            if let Some(ref func) = tc.function {
                                                if let Some(ref name) = func.name {
                                                    chunks.push(StreamChunk::ToolCallStart {
                                                        id: tc.id.clone().unwrap_or_default(),
                                                        name: name.clone(),
                                                    });
                                                }
                                                if let Some(ref args) = func.arguments {
                                                    let delta = match args {
                                                        Value::String(s) => s.clone(),
                                                        other => serde_json::to_string(other)
                                                            .unwrap_or_default(),
                                                    };
                                                    if !delta.is_empty() {
                                                        chunks.push(StreamChunk::ToolCallDelta {
                                                            id: tc.id.clone().unwrap_or_default(),
                                                            arguments_delta: delta,
                                                        });
                                                    }
                                                }
                                            }
                                        }
                                    }

                                    if let Some(ref reason) = choice.finish_reason {
                                        if !text_buf.is_empty() {
                                            chunks.push(StreamChunk::Text(std::mem::take(
                                                &mut text_buf,
                                            )));
                                        }
                                        if reason == "tool_calls"
                                            && let Some(ref tcs) = choice.delta.tool_calls
                                            && let Some(tc) = tcs.last()
                                        {
                                            chunks.push(StreamChunk::ToolCallEnd {
                                                id: tc.id.clone().unwrap_or_default(),
                                            });
                                        }
                                        chunks.push(StreamChunk::Done { usage });
                                    }
                                } else if usage.is_some() {
                                    // Usage-only chunk (empty choices)
                                    if !text_buf.is_empty() {
                                        chunks
                                            .push(StreamChunk::Text(std::mem::take(&mut text_buf)));
                                    }
                                    chunks.push(StreamChunk::Done { usage });
                                }
                            }
                        }
                        if !text_buf.is_empty() {
                            chunks.push(StreamChunk::Text(text_buf));
                        }
                    }
                    Err(e) => chunks.push(StreamChunk::Error(e.to_string())),
                }
                futures::stream::iter(chunks)
            })
            .boxed())
    }
}

// ─── Tests ───────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_model_strips_prefix() {
        assert_eq!(Glm5Provider::normalize_model("glm5/glm-5-fp8"), "glm-5-fp8");
        assert_eq!(Glm5Provider::normalize_model("glm5:glm-5-fp8"), "glm-5-fp8");
        assert_eq!(Glm5Provider::normalize_model("glm-5-fp8"), "glm-5-fp8");
    }

    #[test]
    fn normalize_model_aliases_glm5_to_fp8() {
        assert_eq!(Glm5Provider::normalize_model("glm-5"), DEFAULT_MODEL);
        assert_eq!(Glm5Provider::normalize_model("glm5"), DEFAULT_MODEL);
        assert_eq!(Glm5Provider::normalize_model(""), DEFAULT_MODEL);
        assert_eq!(Glm5Provider::normalize_model("glm5/glm-5"), DEFAULT_MODEL);
        assert_eq!(Glm5Provider::normalize_model("glm5:glm-5"), DEFAULT_MODEL);
    }

    #[test]
    fn normalize_model_preserves_other_variants() {
        assert_eq!(
            Glm5Provider::normalize_model("glm5/glm-5-int4"),
            "glm-5-int4"
        );
    }

    #[test]
    fn convert_messages_serializes_tool_arguments_as_json_string() {
        let messages = vec![Message {
            role: Role::Assistant,
            content: vec![ContentPart::ToolCall {
                id: "call_1".to_string(),
                name: "bash".to_string(),
                arguments: "{\"command\":\"ls\"}".to_string(),
                thought_signature: None,
            }],
        }];

        let converted = Glm5Provider::convert_messages(&messages);
        let args = converted[0]["tool_calls"][0]["function"]["arguments"]
            .as_str()
            .expect("arguments must be a string");

        assert_eq!(args, r#"{"command":"ls"}"#);
    }

    #[test]
    fn convert_messages_wraps_invalid_tool_arguments() {
        let messages = vec![Message {
            role: Role::Assistant,
            content: vec![ContentPart::ToolCall {
                id: "call_1".to_string(),
                name: "bash".to_string(),
                arguments: "command=ls".to_string(),
                thought_signature: None,
            }],
        }];

        let converted = Glm5Provider::convert_messages(&messages);
        let args = converted[0]["tool_calls"][0]["function"]["arguments"]
            .as_str()
            .expect("arguments must be a string");
        let parsed: Value =
            serde_json::from_str(args).expect("wrapped arguments must be valid JSON");

        assert_eq!(parsed, json!({"input": "command=ls"}));
    }
}
