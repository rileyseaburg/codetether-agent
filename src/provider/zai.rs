//! Z.AI provider implementation (direct API)
//!
//! GLM-5, GLM-4.7, and other Z.AI foundation models via api.z.ai.
//! Z.AI (formerly ZhipuAI) offers OpenAI-compatible chat completions with
//! reasoning/thinking support via the `reasoning_content` field.

use super::{
    CompletionRequest, CompletionResponse, ContentPart, FinishReason, Message, ModelInfo, Provider,
    Role, StreamChunk, ToolDefinition, Usage,
};
use anyhow::{Context, Result};
use async_trait::async_trait;
use futures::StreamExt;
use once_cell::sync::Lazy;
use regex::Regex;
use reqwest::Client;
use serde::Deserialize;
use serde_json::{Value, json};
use std::collections::HashMap;

pub const DEFAULT_BASE_URL: &str = "https://api.z.ai/api/paas/v4";
const CODING_BASE_URL: &str = "https://api.z.ai/api/coding/paas/v4";
const PONY_ALPHA_2_MODEL: &str = "pony-alpha-2";

pub struct ZaiProvider {
    client: Client,
    api_key: String,
    base_url: String,
}

#[derive(Debug, Default)]
struct ZaiStreamToolState {
    stream_id: String,
    name: Option<String>,
    started: bool,
    finished: bool,
}

impl std::fmt::Debug for ZaiProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ZaiProvider")
            .field("base_url", &self.base_url)
            .field("api_key", &"<REDACTED>")
            .finish()
    }
}

impl ZaiProvider {
    pub fn with_base_url(api_key: String, base_url: String) -> Result<Self> {
        tracing::debug!(
            provider = "zai",
            base_url = %base_url,
            api_key_len = api_key.len(),
            "Creating Z.AI provider with custom base URL"
        );
        Ok(Self {
            client: Client::new(),
            api_key,
            base_url,
        })
    }

    fn request_base_url(&self, model: &str) -> &str {
        if model.eq_ignore_ascii_case(PONY_ALPHA_2_MODEL) {
            CODING_BASE_URL
        } else {
            &self.base_url
        }
    }

    /// Fetch available models from the Z.AI /models endpoint.
    /// Returns an empty vec on any failure (network, parse, auth) so callers
    /// can fall back to the hardcoded catalog.
    async fn discover_models_from_api(&self) -> Vec<ModelInfo> {
        // Always hit the standard API endpoint for model discovery,
        // even if base_url points at the coding endpoint.
        let discovery_url = if self.base_url.contains("/coding/") {
            self.base_url.replace("/coding/", "/")
        } else {
            self.base_url.clone()
        };
        let url = format!("{discovery_url}/models");
        let response = match self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await
        {
            Ok(r) => r,
            Err(e) => {
                tracing::debug!(
                    url = %url,
                    error = %e,
                    "Z.AI /models discovery request failed"
                );
                return Vec::new();
            }
        };

        if !response.status().is_success() {
            tracing::debug!(
                url = %url,
                status = %response.status(),
                "Z.AI /models endpoint returned non-success"
            );
            return Vec::new();
        }

        let payload: Value = match response.json().await {
            Ok(p) => p,
            Err(e) => {
                tracing::debug!(
                    url = %url,
                    error = %e,
                    "Failed to parse Z.AI /models response"
                );
                return Vec::new();
            }
        };

        let models = payload
            .get("data")
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
            .filter_map(|entry| {
                let id = match entry {
                    Value::String(s) => s.trim().to_string(),
                    Value::Object(_) => entry.get("id").and_then(Value::as_str)?.trim().to_string(),
                    _ => return None,
                };
                if id.is_empty() {
                    return None;
                }
                let name = entry
                    .get("name")
                    .and_then(Value::as_str)
                    .map(str::trim)
                    .filter(|n| !n.is_empty())
                    .unwrap_or(&id)
                    .to_string();
                Some(ModelInfo {
                    id,
                    name,
                    provider: "zai".to_string(),
                    context_window: 200_000,
                    max_output_tokens: Some(128_000),
                    supports_vision: false,
                    supports_tools: true,
                    supports_streaming: true,
                    input_cost_per_million: None,
                    output_cost_per_million: None,
                })
            })
            .collect::<Vec<_>>();

        if models.is_empty() {
            tracing::debug!(url = %url, "Z.AI /models returned no model ids");
        } else {
            tracing::info!(count = models.len(), "Z.AI /models discovery succeeded");
        }
        models
    }

    fn normalize_tool_arguments(arguments: &str) -> String {
        // The live Z.AI endpoint rejects object-typed historical arguments in
        // assistant.tool_calls and accepts OpenAI-style JSON strings instead.
        if let Ok(parsed) = serde_json::from_str::<Value>(arguments) {
            if parsed.is_object() {
                return serde_json::to_string(&parsed).unwrap_or_else(|_| "{}".to_string());
            }
            return json!({"input": parsed}).to_string();
        }

        if let Some(salvaged) = Self::salvage_json_object(arguments) {
            return serde_json::to_string(&salvaged).unwrap_or_else(|_| "{}".to_string());
        }

        json!({"input": arguments}).to_string()
    }

    fn salvage_json_object(arguments: &str) -> Option<Value> {
        let trimmed = arguments.trim();
        if !trimmed.starts_with('{') {
            return None;
        }

        static RE_SIMPLE_PAIR: Lazy<Regex> = Lazy::new(|| {
            // Matches simple JSON key/value pairs where the value is a primitive
            // or a quoted string. This is intentionally conservative.
            Regex::new(
                r#"(?s)\"(?P<k>[^\"\\]*(?:\\.[^\"\\]*)*)\"\s*:\s*(?P<v>\"(?:\\.|[^\"])*\"|true|false|null|-?\d+(?:\.\d+)?(?:[eE][+-]?\d+)?)"#,
            )
            .expect("invalid regex")
        });

        let mut map = serde_json::Map::new();
        for caps in RE_SIMPLE_PAIR.captures_iter(trimmed) {
            let key = caps.name("k")?.as_str();
            let val_str = caps.name("v")?.as_str();
            if let Ok(val) = serde_json::from_str::<Value>(val_str) {
                map.insert(key.to_string(), val);
            }
        }

        if map.is_empty() {
            None
        } else {
            Some(Value::Object(map))
        }
    }

    fn convert_messages(messages: &[Message], include_reasoning_content: bool) -> Vec<Value> {
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
                                    let args_string = Self::normalize_tool_arguments(arguments);
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
                            "content": text,
                        });
                        if include_reasoning_content {
                            let reasoning: String = msg
                                .content
                                .iter()
                                .filter_map(|p| match p {
                                    ContentPart::Thinking { text } => Some(text.clone()),
                                    _ => None,
                                })
                                .collect::<Vec<_>>()
                                .join("");
                            if !reasoning.is_empty() {
                                msg_json["reasoning_content"] = json!(reasoning);
                            }
                        }
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

    fn model_supports_tool_stream(model: &str) -> bool {
        model.contains("glm-5")
            || model.contains("glm-4.7")
            || model.contains("glm-4.6")
            || model.eq_ignore_ascii_case(PONY_ALPHA_2_MODEL)
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

    fn stream_tool_arguments_fragment(arguments: &Value) -> String {
        match arguments {
            Value::Null => String::new(),
            Value::String(s) => s.clone(),
            other => serde_json::to_string(other).unwrap_or_default(),
        }
    }

    fn append_stream_tool_call_chunks(
        chunks: &mut Vec<StreamChunk>,
        tool_calls: &[ZaiStreamToolCall],
        tool_states: &mut HashMap<usize, ZaiStreamToolState>,
        next_fallback_index: &mut usize,
        last_seen_index: &mut Option<usize>,
    ) {
        for tc in tool_calls {
            let index = tc
                .index
                .or_else(|| {
                    tc.id.as_ref().and_then(|id| {
                        tool_states
                            .iter()
                            .find_map(|(idx, state)| (state.stream_id == *id).then_some(*idx))
                    })
                })
                .or(*last_seen_index)
                .unwrap_or_else(|| {
                    let idx = *next_fallback_index;
                    *next_fallback_index += 1;
                    idx
                });
            *last_seen_index = Some(index);

            let state = tool_states
                .entry(index)
                .or_insert_with(|| ZaiStreamToolState {
                    stream_id: tc.id.clone().unwrap_or_else(|| format!("zai-tool-{index}")),
                    ..Default::default()
                });

            if let Some(id) = &tc.id
                && !state.started
                && state.stream_id.starts_with("zai-tool-")
            {
                state.stream_id = id.clone();
            }

            if let Some(func) = &tc.function {
                if let Some(name) = &func.name
                    && !name.is_empty()
                {
                    state.name = Some(name.clone());
                }

                if !state.started
                    && let Some(name) = &state.name
                {
                    chunks.push(StreamChunk::ToolCallStart {
                        id: state.stream_id.clone(),
                        name: name.clone(),
                    });
                    state.started = true;
                }

                if let Some(arguments) = &func.arguments {
                    let delta = Self::stream_tool_arguments_fragment(arguments);
                    if !delta.is_empty() {
                        if !state.started {
                            chunks.push(StreamChunk::ToolCallStart {
                                id: state.stream_id.clone(),
                                name: state.name.clone().unwrap_or_else(|| "tool".to_string()),
                            });
                            state.started = true;
                        }
                        chunks.push(StreamChunk::ToolCallDelta {
                            id: state.stream_id.clone(),
                            arguments_delta: delta,
                        });
                    }
                }
            }
        }
    }

    fn finish_stream_tool_call_chunks(
        chunks: &mut Vec<StreamChunk>,
        tool_states: &mut HashMap<usize, ZaiStreamToolState>,
    ) {
        let mut ordered_indexes: Vec<_> = tool_states.keys().copied().collect();
        ordered_indexes.sort_unstable();

        for index in ordered_indexes {
            if let Some(state) = tool_states.get_mut(&index)
                && state.started
                && !state.finished
            {
                chunks.push(StreamChunk::ToolCallEnd {
                    id: state.stream_id.clone(),
                });
                state.finished = true;
            }
        }
    }
}

#[derive(Debug, Deserialize)]
struct ZaiResponse {
    choices: Vec<ZaiChoice>,
    #[serde(default)]
    usage: Option<ZaiUsage>,
}

#[derive(Debug, Deserialize)]
struct ZaiChoice {
    message: ZaiMessage,
    #[serde(default)]
    finish_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ZaiMessage {
    #[serde(default)]
    content: Option<String>,
    #[serde(default)]
    tool_calls: Option<Vec<ZaiToolCall>>,
    #[serde(default)]
    reasoning_content: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ZaiToolCall {
    id: String,
    function: ZaiFunction,
}

#[derive(Debug, Deserialize)]
struct ZaiFunction {
    name: String,
    arguments: Value,
}

#[derive(Debug, Deserialize)]
struct ZaiUsage {
    #[serde(default)]
    prompt_tokens: usize,
    #[serde(default)]
    completion_tokens: usize,
    #[serde(default)]
    total_tokens: usize,
    #[serde(default)]
    prompt_tokens_details: Option<ZaiPromptTokensDetails>,
}

#[derive(Debug, Deserialize)]
struct ZaiPromptTokensDetails {
    #[serde(default)]
    cached_tokens: usize,
}

#[derive(Debug, Deserialize)]
struct ZaiError {
    error: ZaiErrorDetail,
}

#[derive(Debug, Deserialize)]
struct ZaiErrorDetail {
    message: String,
    #[serde(default, rename = "type")]
    error_type: Option<String>,
}

// SSE stream types
#[derive(Debug, Deserialize)]
struct ZaiStreamResponse {
    choices: Vec<ZaiStreamChoice>,
}

#[derive(Debug, Deserialize)]
struct ZaiStreamChoice {
    delta: ZaiStreamDelta,
    #[serde(default)]
    finish_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ZaiStreamDelta {
    #[serde(default)]
    content: Option<String>,
    #[serde(default)]
    reasoning_content: Option<String>,
    #[serde(default)]
    tool_calls: Option<Vec<ZaiStreamToolCall>>,
}

#[derive(Debug, Deserialize)]
struct ZaiStreamToolCall {
    #[serde(default)]
    index: Option<usize>,
    #[serde(default)]
    id: Option<String>,
    function: Option<ZaiStreamFunction>,
}

#[derive(Debug, Deserialize)]
struct ZaiStreamFunction {
    #[serde(default)]
    name: Option<String>,
    #[serde(default)]
    arguments: Option<Value>,
}

#[async_trait]
impl Provider for ZaiProvider {
    fn name(&self) -> &str {
        "zai"
    }

    async fn list_models(&self) -> Result<Vec<ModelInfo>> {
        // Attempt dynamic model discovery from the Z.AI /models endpoint.
        // When the API is reachable, this returns the authoritative model
        // catalog including newly-released models without a code change.
        let discovered = self.discover_models_from_api().await;
        if !discovered.is_empty() {
            // Merge in special models that exist outside the /models API
            // (coding endpoint models, etc.)
            let mut models = discovered;
            if !models.iter().any(|m| m.id == PONY_ALPHA_2_MODEL) {
                models.push(ModelInfo {
                    id: PONY_ALPHA_2_MODEL.to_string(),
                    name: "Pony Alpha 2".to_string(),
                    provider: "zai".to_string(),
                    context_window: 128_000,
                    max_output_tokens: Some(16_384),
                    supports_vision: false,
                    supports_tools: true,
                    supports_streaming: true,
                    input_cost_per_million: None,
                    output_cost_per_million: None,
                });
            }
            if !models.iter().any(|m| m.id == "glm-4.7-flash") {
                models.push(ModelInfo {
                    id: "glm-4.7-flash".to_string(),
                    name: "GLM-4.7 Flash".to_string(),
                    provider: "zai".to_string(),
                    context_window: 128_000,
                    max_output_tokens: Some(128_000),
                    supports_vision: false,
                    supports_tools: true,
                    supports_streaming: true,
                    input_cost_per_million: None,
                    output_cost_per_million: None,
                });
            }
            return Ok(models);
        }

        // Static catalog used when the /models endpoint is unavailable
        // (e.g. network partition, auth failure, or non-standard deployments).
        Ok(vec![
            ModelInfo {
                id: "glm-5.1".to_string(),
                name: "GLM-5.1".to_string(),
                provider: "zai".to_string(),
                context_window: 200_000,
                max_output_tokens: Some(128_000),
                supports_vision: false,
                supports_tools: true,
                supports_streaming: true,
                input_cost_per_million: None,
                output_cost_per_million: None,
            },
            ModelInfo {
                id: "glm-5".to_string(),
                name: "GLM-5".to_string(),
                provider: "zai".to_string(),
                context_window: 200_000,
                max_output_tokens: Some(128_000),
                supports_vision: false,
                supports_tools: true,
                supports_streaming: true,
                input_cost_per_million: None,
                output_cost_per_million: None,
            },
            ModelInfo {
                id: "glm-4.7".to_string(),
                name: "GLM-4.7".to_string(),
                provider: "zai".to_string(),
                context_window: 128_000,
                max_output_tokens: Some(128_000),
                supports_vision: false,
                supports_tools: true,
                supports_streaming: true,
                input_cost_per_million: None,
                output_cost_per_million: None,
            },
            ModelInfo {
                id: "glm-4.7-flash".to_string(),
                name: "GLM-4.7 Flash".to_string(),
                provider: "zai".to_string(),
                context_window: 128_000,
                max_output_tokens: Some(128_000),
                supports_vision: false,
                supports_tools: true,
                supports_streaming: true,
                input_cost_per_million: None,
                output_cost_per_million: None,
            },
            ModelInfo {
                id: "glm-4.6".to_string(),
                name: "GLM-4.6".to_string(),
                provider: "zai".to_string(),
                context_window: 128_000,
                max_output_tokens: Some(128_000),
                supports_vision: false,
                supports_tools: true,
                supports_streaming: true,
                input_cost_per_million: None,
                output_cost_per_million: None,
            },
            ModelInfo {
                id: "glm-4.5".to_string(),
                name: "GLM-4.5".to_string(),
                provider: "zai".to_string(),
                context_window: 128_000,
                max_output_tokens: Some(96_000),
                supports_vision: false,
                supports_tools: true,
                supports_streaming: true,
                input_cost_per_million: None,
                output_cost_per_million: None,
            },
            ModelInfo {
                id: "glm-5-turbo".to_string(),
                name: "GLM-5 Turbo".to_string(),
                provider: "zai".to_string(),
                context_window: 200_000,
                max_output_tokens: Some(128_000),
                supports_vision: false,
                supports_tools: true,
                supports_streaming: true,
                input_cost_per_million: Some(0.96),
                output_cost_per_million: Some(3.20),
            },
            ModelInfo {
                id: PONY_ALPHA_2_MODEL.to_string(),
                name: "Pony Alpha 2".to_string(),
                provider: "zai".to_string(),
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
        // Compatibility-first mode: omit historical reasoning_content from
        // request messages to avoid strict parameter validation errors on some
        // endpoint variants.
        let messages = Self::convert_messages(&request.messages, false);
        let tools = Self::convert_tools(&request.tools);

        // GLM-5 and GLM-4.7 default to temperature 1.0
        let temperature = request.temperature.unwrap_or(1.0);

        let mut body = json!({
            "model": request.model,
            "messages": messages,
            "temperature": temperature,
        });

        // Always enable thinking with clear_thinking: true.
        // The Coding endpoint (api.z.ai/api/coding/paas/v4) defaults
        // clear_thinking to false (Preserved Thinking), which requires
        // reasoning_content in historical assistant messages. Since we strip
        // reasoning_content, we must explicitly set clear_thinking: true.
        body["thinking"] = json!({
            "type": "enabled",
            "clear_thinking": true
        });

        if !tools.is_empty() {
            body["tools"] = json!(tools);
        }
        if let Some(max) = request.max_tokens {
            body["max_tokens"] = json!(max);
        }

        tracing::debug!(model = %request.model, "Z.AI request");
        tracing::trace!(body = %serde_json::to_string(&body).unwrap_or_default(), "Z.AI request body");
        let request_base_url = self.request_base_url(&request.model);

        let (text, status) = super::retry::send_with_retry(|| async {
            let resp = self
                .client
                .post(format!("{}/chat/completions", request_base_url))
                .header("Authorization", format!("Bearer {}", self.api_key))
                .header("Content-Type", "application/json")
                .json(&body)
                .send()
                .await
                .context("Failed to send request to Z.AI")?;
            let status = resp.status();
            let text = resp.text().await.context("Failed to read Z.AI response")?;
            Ok((text, status))
        })
        .await?;

        if !status.is_success() {
            tracing::debug!(status = %status, body = %text, "Z.AI error response");
            if let Ok(err) = serde_json::from_str::<ZaiError>(&text) {
                anyhow::bail!(
                    "Z.AI API error: {} ({:?})",
                    err.error.message,
                    err.error.error_type
                );
            }
            anyhow::bail!("Z.AI API error: {status} {text}");
        }

        let response: ZaiResponse = serde_json::from_str(&text).context(format!(
            "Failed to parse Z.AI response: {}",
            Self::preview_text(&text, 200)
        ))?;

        let choice = response
            .choices
            .first()
            .ok_or_else(|| anyhow::anyhow!("No choices in Z.AI response"))?;

        // Log thinking/reasoning content if present
        if let Some(ref reasoning) = choice.message.reasoning_content
            && !reasoning.is_empty()
        {
            tracing::info!(
                reasoning_len = reasoning.len(),
                "Z.AI reasoning content received"
            );
        }

        let mut content = Vec::new();
        let mut has_tool_calls = false;

        // Emit thinking content as a Thinking part
        if let Some(ref reasoning) = choice.message.reasoning_content
            && !reasoning.is_empty()
        {
            content.push(ContentPart::Thinking {
                text: reasoning.clone(),
            });
        }

        if let Some(text) = &choice.message.content
            && !text.is_empty()
        {
            content.push(ContentPart::Text { text: text.clone() });
        }

        if let Some(tool_calls) = &choice.message.tool_calls {
            has_tool_calls = !tool_calls.is_empty();
            for tc in tool_calls {
                // Z.AI returns arguments as an object; serialize to string for our ContentPart
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
                Some("sensitive") => FinishReason::ContentFilter,
                _ => FinishReason::Stop,
            }
        };

        Ok(CompletionResponse {
            message: Message {
                role: Role::Assistant,
                content,
            },
            usage: Usage {
                prompt_tokens: response
                    .usage
                    .as_ref()
                    .map(|u| u.prompt_tokens)
                    .unwrap_or(0),
                completion_tokens: response
                    .usage
                    .as_ref()
                    .map(|u| u.completion_tokens)
                    .unwrap_or(0),
                total_tokens: response.usage.as_ref().map(|u| u.total_tokens).unwrap_or(0),
                cache_read_tokens: response
                    .usage
                    .as_ref()
                    .and_then(|u| u.prompt_tokens_details.as_ref())
                    .map(|d| d.cached_tokens)
                    .filter(|&t| t > 0),
                cache_write_tokens: None,
            },
            finish_reason,
        })
    }

    async fn complete_stream(
        &self,
        request: CompletionRequest,
    ) -> Result<futures::stream::BoxStream<'static, StreamChunk>> {
        // Compatibility-first mode: omit historical reasoning_content from
        // request messages to avoid strict parameter validation errors on some
        // endpoint variants.
        let messages = Self::convert_messages(&request.messages, false);
        let tools = Self::convert_tools(&request.tools);

        let temperature = request.temperature.unwrap_or(1.0);

        let mut body = json!({
            "model": request.model,
            "messages": messages,
            "temperature": temperature,
            "stream": true,
        });

        body["thinking"] = json!({
            "type": "enabled",
            "clear_thinking": true
        });

        if !tools.is_empty() {
            body["tools"] = json!(tools);
            if Self::model_supports_tool_stream(&request.model) {
                // Enable streaming tool calls only on known-compatible models.
                body["tool_stream"] = json!(true);
            }
        }
        if let Some(max) = request.max_tokens {
            body["max_tokens"] = json!(max);
        }

        tracing::debug!(model = %request.model, "Z.AI streaming request");
        let request_base_url = self.request_base_url(&request.model);

        let response = super::retry::send_response_with_retry(|| async {
            self.client
                .post(format!("{}/chat/completions", request_base_url))
                .header("Authorization", format!("Bearer {}", self.api_key))
                .header("Content-Type", "application/json")
                .json(&body)
                .send()
                .await
                .context("Failed to send streaming request to Z.AI")
        })
        .await?;

        let stream = response.bytes_stream();
        let mut buffer = String::new();
        let mut tool_states = HashMap::<usize, ZaiStreamToolState>::new();
        let mut next_fallback_tool_index = 0usize;
        let mut last_seen_tool_index = None;

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
                                && let Ok(parsed) = serde_json::from_str::<ZaiStreamResponse>(data)
                                && let Some(choice) = parsed.choices.first()
                            {
                                // Reasoning content streamed as text (prefixed for TUI rendering)
                                if let Some(ref reasoning) = choice.delta.reasoning_content
                                    && !reasoning.is_empty()
                                {
                                    text_buf.push_str(reasoning);
                                }
                                if let Some(ref content) = choice.delta.content {
                                    text_buf.push_str(content);
                                }
                                // Streaming tool calls
                                if let Some(ref tool_calls) = choice.delta.tool_calls {
                                    if !text_buf.is_empty() {
                                        chunks
                                            .push(StreamChunk::Text(std::mem::take(&mut text_buf)));
                                    }
                                    Self::append_stream_tool_call_chunks(
                                        &mut chunks,
                                        tool_calls,
                                        &mut tool_states,
                                        &mut next_fallback_tool_index,
                                        &mut last_seen_tool_index,
                                    );
                                }
                                // finish_reason signals end of a tool call or completion
                                if let Some(ref reason) = choice.finish_reason {
                                    if !text_buf.is_empty() {
                                        chunks
                                            .push(StreamChunk::Text(std::mem::take(&mut text_buf)));
                                    }
                                    if reason == "tool_calls" {
                                        Self::finish_stream_tool_call_chunks(
                                            &mut chunks,
                                            &mut tool_states,
                                        );
                                    }
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::provider::Provider;

    #[tokio::test]
    async fn list_models_includes_pony_alpha_2() {
        let provider =
            ZaiProvider::with_base_url("test-key".to_string(), DEFAULT_BASE_URL.to_string())
                .expect("provider should construct");
        let models = provider.list_models().await.expect("models should list");

        assert!(models.iter().any(|model| model.id == PONY_ALPHA_2_MODEL));
    }

    #[tokio::test]
    async fn list_models_includes_glm_5_turbo() {
        let provider =
            ZaiProvider::with_base_url("test-key".to_string(), DEFAULT_BASE_URL.to_string())
                .expect("provider should construct");
        let models = provider.list_models().await.expect("models should list");

        let turbo = models
            .iter()
            .find(|m| m.id == "glm-5-turbo")
            .expect("glm-5-turbo should be in model list");
        assert_eq!(turbo.context_window, 200_000);
        assert_eq!(turbo.max_output_tokens, Some(128_000));
        assert!(turbo.supports_tools);
        assert!(turbo.supports_streaming);
        assert_eq!(turbo.input_cost_per_million, Some(0.96));
        assert_eq!(turbo.output_cost_per_million, Some(3.20));
    }

    #[tokio::test]
    async fn list_models_includes_glm_5_1() {
        let provider =
            ZaiProvider::with_base_url("test-key".to_string(), DEFAULT_BASE_URL.to_string())
                .expect("provider should construct");
        let models = provider.list_models().await.expect("models should list");

        let glm51 = models
            .iter()
            .find(|m| m.id == "glm-5.1")
            .expect("glm-5.1 should be in model list");
        assert_eq!(glm51.context_window, 200_000);
        assert_eq!(glm51.max_output_tokens, Some(128_000));
        assert!(glm51.supports_tools);
        assert!(glm51.supports_streaming);
    }

    #[test]
    fn model_supports_tool_stream_matches_glm_5_1() {
        assert!(ZaiProvider::model_supports_tool_stream("glm-5.1"));
        assert!(ZaiProvider::model_supports_tool_stream("glm-5"));
        assert!(ZaiProvider::model_supports_tool_stream("glm-5-turbo"));
        assert!(!ZaiProvider::model_supports_tool_stream("glm-4.5"));
    }

    #[test]
    fn pony_alpha_2_routes_to_coding_endpoint() {
        let provider =
            ZaiProvider::with_base_url("test-key".to_string(), DEFAULT_BASE_URL.to_string())
                .expect("provider should construct");

        assert_eq!(
            provider.request_base_url(PONY_ALPHA_2_MODEL),
            CODING_BASE_URL
        );
        assert_eq!(provider.request_base_url("glm-5"), DEFAULT_BASE_URL);
    }

    #[test]
    fn convert_messages_serializes_tool_arguments_as_json_string() {
        let messages = vec![Message {
            role: Role::Assistant,
            content: vec![ContentPart::ToolCall {
                id: "call_1".to_string(),
                name: "get_weather".to_string(),
                arguments: "{\"city\":\"Beijing\".. }".to_string(),
                thought_signature: None,
            }],
        }];

        let converted = ZaiProvider::convert_messages(&messages, true);
        let args = converted[0]["tool_calls"][0]["function"]["arguments"]
            .as_str()
            .expect("arguments must be a string");
        let parsed: Value =
            serde_json::from_str(args).expect("arguments string must contain valid JSON");

        assert_eq!(parsed, json!({"city":"Beijing"}));
    }

    #[test]
    fn convert_messages_wraps_invalid_tool_arguments_as_json_string() {
        let messages = vec![Message {
            role: Role::Assistant,
            content: vec![ContentPart::ToolCall {
                id: "call_1".to_string(),
                name: "get_weather".to_string(),
                arguments: "city=Beijing".to_string(),
                thought_signature: None,
            }],
        }];

        let converted = ZaiProvider::convert_messages(&messages, true);
        let args = converted[0]["tool_calls"][0]["function"]["arguments"]
            .as_str()
            .expect("arguments must be a string");
        let parsed: Value =
            serde_json::from_str(args).expect("arguments string must contain valid JSON");

        assert_eq!(parsed, json!({"input":"city=Beijing"}));
    }

    #[test]
    fn convert_messages_wraps_scalar_tool_arguments_as_json_string() {
        let messages = vec![Message {
            role: Role::Assistant,
            content: vec![ContentPart::ToolCall {
                id: "call_1".to_string(),
                name: "get_weather".to_string(),
                arguments: "\"Beijing\"".to_string(),
                thought_signature: None,
            }],
        }];

        let converted = ZaiProvider::convert_messages(&messages, true);
        let args = converted[0]["tool_calls"][0]["function"]["arguments"]
            .as_str()
            .expect("arguments must be a string");
        let parsed: Value =
            serde_json::from_str(args).expect("arguments string must contain valid JSON");

        assert_eq!(parsed, json!({"input":"Beijing"}));
    }

    #[test]
    fn stream_tool_chunks_keep_same_call_id_when_followup_delta_omits_id() {
        let mut chunks = Vec::new();
        let mut tool_states = HashMap::new();
        let mut next_fallback_tool_index = 0usize;
        let mut last_seen_tool_index = None;

        ZaiProvider::append_stream_tool_call_chunks(
            &mut chunks,
            &[ZaiStreamToolCall {
                index: Some(0),
                id: Some("call_1".to_string()),
                function: Some(ZaiStreamFunction {
                    name: Some("bash".to_string()),
                    arguments: Some(Value::String("{\"".to_string())),
                }),
            }],
            &mut tool_states,
            &mut next_fallback_tool_index,
            &mut last_seen_tool_index,
        );

        ZaiProvider::append_stream_tool_call_chunks(
            &mut chunks,
            &[ZaiStreamToolCall {
                index: Some(0),
                id: None,
                function: Some(ZaiStreamFunction {
                    name: None,
                    arguments: Some(Value::String("command\":\"pwd\"}".to_string())),
                }),
            }],
            &mut tool_states,
            &mut next_fallback_tool_index,
            &mut last_seen_tool_index,
        );

        ZaiProvider::finish_stream_tool_call_chunks(&mut chunks, &mut tool_states);

        assert_eq!(chunks.len(), 4);
        assert!(matches!(
            &chunks[0],
            StreamChunk::ToolCallStart { id, name }
                if id == "call_1" && name == "bash"
        ));
        assert!(matches!(
            &chunks[1],
            StreamChunk::ToolCallDelta { id, arguments_delta }
                if id == "call_1" && arguments_delta == "{\""
        ));
        assert!(matches!(
            &chunks[2],
            StreamChunk::ToolCallDelta { id, arguments_delta }
                if id == "call_1" && arguments_delta == "command\":\"pwd\"}"
        ));
        assert!(matches!(
            &chunks[3],
            StreamChunk::ToolCallEnd { id } if id == "call_1"
        ));
    }

    #[test]
    fn preview_text_truncates_on_char_boundary() {
        let text = "a😀b";
        assert_eq!(ZaiProvider::preview_text(text, 2), "a😀");
    }
}
