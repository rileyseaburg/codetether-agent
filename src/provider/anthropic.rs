//! Anthropic provider implementation using the Messages API
//!
//! Supports Claude Sonnet 4, Claude Opus 4, and other Claude models.
//! Uses the native Anthropic API format (not OpenAI-compatible).
//! Reference: https://docs.anthropic.com/en/api/messages

use super::{
    CompletionRequest, CompletionResponse, ContentPart, FinishReason, Message, ModelInfo, Provider,
    Role, StreamChunk, ToolDefinition, Usage,
};
use anyhow::{Context, Result};
use async_trait::async_trait;
use reqwest::Client;
use serde::Deserialize;
use serde_json::{Value, json};

const ANTHROPIC_API_BASE: &str = "https://api.anthropic.com";
const ANTHROPIC_VERSION: &str = "2023-06-01";

pub struct AnthropicProvider {
    client: Client,
    api_key: String,
    base_url: String,
    provider_name: String,
    enable_prompt_caching: bool,
}

impl std::fmt::Debug for AnthropicProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AnthropicProvider")
            .field("api_key", &"<REDACTED>")
            .field("api_key_len", &self.api_key.len())
            .field("base_url", &self.base_url)
            .field("provider_name", &self.provider_name)
            .field("enable_prompt_caching", &self.enable_prompt_caching)
            .finish()
    }
}

impl AnthropicProvider {
    pub fn new(api_key: String) -> Result<Self> {
        Self::with_base_url(api_key, ANTHROPIC_API_BASE.to_string(), "anthropic")
    }

    pub fn with_base_url(
        api_key: String,
        base_url: String,
        provider_name: impl Into<String>,
    ) -> Result<Self> {
        let provider_name = provider_name.into();
        let enable_prompt_caching = std::env::var("CODETETHER_ANTHROPIC_PROMPT_CACHING")
            .ok()
            .and_then(|v| parse_bool_env(&v))
            .unwrap_or_else(|| {
                provider_name.eq_ignore_ascii_case("minimax")
                    || provider_name.eq_ignore_ascii_case("minimax-credits")
            });

        tracing::debug!(
            provider = %provider_name,
            api_key_len = api_key.len(),
            base_url = %base_url,
            enable_prompt_caching,
            "Creating Anthropic provider"
        );

        Ok(Self {
            client: Client::new(),
            api_key,
            base_url,
            provider_name,
            enable_prompt_caching,
        })
    }

    fn validate_api_key(&self) -> Result<()> {
        if self.api_key.is_empty() {
            anyhow::bail!("Anthropic API key is empty");
        }
        Ok(())
    }

    /// Convert our generic messages to Anthropic Messages API format.
    ///
    /// Anthropic uses a different format:
    /// - system prompt is a top-level field, not a message
    /// - tool results go in user messages with type "tool_result"
    /// - tool calls appear in assistant messages with type "tool_use"
    fn convert_messages(
        messages: &[Message],
        enable_prompt_caching: bool,
    ) -> (Option<Vec<Value>>, Vec<Value>) {
        let mut system_blocks: Vec<Value> = Vec::new();
        let mut api_messages: Vec<Value> = Vec::new();

        for msg in messages {
            match msg.role {
                Role::System => {
                    for part in &msg.content {
                        match part {
                            ContentPart::Text { text } => {
                                system_blocks.push(json!({
                                    "type": "text",
                                    "text": text,
                                }));
                            }
                            ContentPart::Thinking { text } => {
                                system_blocks.push(json!({
                                    "type": "thinking",
                                    "thinking": text,
                                }));
                            }
                            _ => {}
                        }
                    }
                }
                Role::User => {
                    let mut content_parts: Vec<Value> = Vec::new();
                    for part in &msg.content {
                        match part {
                            ContentPart::Text { text } => {
                                content_parts.push(json!({
                                    "type": "text",
                                    "text": text,
                                }));
                            }
                            ContentPart::Thinking { text } => {
                                content_parts.push(json!({
                                    "type": "thinking",
                                    "thinking": text,
                                }));
                            }
                            _ => {}
                        }
                    }
                    if content_parts.is_empty() {
                        content_parts.push(json!({"type": "text", "text": " "}));
                    }
                    api_messages.push(json!({
                        "role": "user",
                        "content": content_parts
                    }));
                }
                Role::Assistant => {
                    let mut content_parts: Vec<Value> = Vec::new();

                    for part in &msg.content {
                        match part {
                            ContentPart::Text { text } => {
                                content_parts.push(json!({
                                    "type": "text",
                                    "text": text
                                }));
                            }
                            ContentPart::Thinking { text } => {
                                content_parts.push(json!({
                                    "type": "thinking",
                                    "thinking": text
                                }));
                            }
                            ContentPart::ToolCall {
                                id,
                                name,
                                arguments,
                                ..
                            } => {
                                let input: Value = serde_json::from_str(arguments)
                                    .unwrap_or_else(|_| json!({"raw": arguments}));
                                content_parts.push(json!({
                                    "type": "tool_use",
                                    "id": id,
                                    "name": name,
                                    "input": input
                                }));
                            }
                            _ => {}
                        }
                    }

                    if content_parts.is_empty() {
                        content_parts.push(json!({"type": "text", "text": " "}));
                    }

                    api_messages.push(json!({
                        "role": "assistant",
                        "content": content_parts
                    }));
                }
                Role::Tool => {
                    let mut tool_results: Vec<Value> = Vec::new();
                    for part in &msg.content {
                        if let ContentPart::ToolResult {
                            tool_call_id,
                            content,
                        } = part
                        {
                            tool_results.push(json!({
                                "type": "tool_result",
                                "tool_use_id": tool_call_id,
                                "content": content
                            }));
                        }
                    }
                    if !tool_results.is_empty() {
                        api_messages.push(json!({
                            "role": "user",
                            "content": tool_results
                        }));
                    }
                }
            }
        }

        if enable_prompt_caching {
            if let Some(last_tool_or_text_msg) = api_messages.iter_mut().rev().find_map(|msg| {
                msg.get_mut("content")
                    .and_then(Value::as_array_mut)
                    .and_then(|parts| parts.last_mut())
            }) {
                Self::add_ephemeral_cache_control(last_tool_or_text_msg);
            }
            if let Some(last_system) = system_blocks.last_mut() {
                Self::add_ephemeral_cache_control(last_system);
            }
        }

        let system = if system_blocks.is_empty() {
            None
        } else {
            Some(system_blocks)
        };

        (system, api_messages)
    }

    fn convert_tools(tools: &[ToolDefinition], enable_prompt_caching: bool) -> Vec<Value> {
        let mut converted: Vec<Value> = tools
            .iter()
            .map(|t| {
                json!({
                    "name": t.name,
                    "description": t.description,
                    "input_schema": t.parameters
                })
            })
            .collect();

        if enable_prompt_caching && let Some(last_tool) = converted.last_mut() {
            Self::add_ephemeral_cache_control(last_tool);
        }

        converted
    }

    fn add_ephemeral_cache_control(block: &mut Value) {
        if let Some(obj) = block.as_object_mut() {
            obj.insert("cache_control".to_string(), json!({ "type": "ephemeral" }));
        }
    }
}

#[derive(Debug, Deserialize)]
struct AnthropicResponse {
    #[allow(dead_code)]
    id: String,
    #[allow(dead_code)]
    model: String,
    content: Vec<AnthropicContent>,
    #[serde(default)]
    stop_reason: Option<String>,
    #[serde(default)]
    usage: Option<AnthropicUsage>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
enum AnthropicContent {
    #[serde(rename = "text")]
    Text { text: String },
    #[serde(rename = "thinking")]
    Thinking {
        #[serde(default)]
        thinking: Option<String>,
        #[serde(default)]
        text: Option<String>,
    },
    #[serde(rename = "tool_use")]
    ToolUse {
        id: String,
        name: String,
        input: Value,
    },
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Deserialize)]
struct AnthropicUsage {
    #[serde(default)]
    input_tokens: usize,
    #[serde(default)]
    output_tokens: usize,
    #[serde(default)]
    cache_creation_input_tokens: Option<usize>,
    #[serde(default)]
    cache_read_input_tokens: Option<usize>,
}

#[derive(Debug, Deserialize)]
struct AnthropicError {
    error: AnthropicErrorDetail,
}

#[derive(Debug, Deserialize)]
struct AnthropicErrorDetail {
    message: String,
    #[serde(default, rename = "type")]
    error_type: Option<String>,
}

#[async_trait]
impl Provider for AnthropicProvider {
    fn name(&self) -> &str {
        &self.provider_name
    }

    async fn list_models(&self) -> Result<Vec<ModelInfo>> {
        self.validate_api_key()?;

        if self.provider_name.eq_ignore_ascii_case("minimax-credits") {
            // Credits-based key: highspeed models only (not on coding plan)
            return Ok(vec![
                ModelInfo {
                    id: "MiniMax-M2.5-highspeed".to_string(),
                    name: "MiniMax M2.5 Highspeed".to_string(),
                    provider: self.provider_name.clone(),
                    context_window: 200_000,
                    max_output_tokens: Some(65_536),
                    supports_vision: false,
                    supports_tools: true,
                    supports_streaming: true,
                    input_cost_per_million: Some(0.6),
                    output_cost_per_million: Some(2.4),
                },
                ModelInfo {
                    id: "MiniMax-M2.1-highspeed".to_string(),
                    name: "MiniMax M2.1 Highspeed".to_string(),
                    provider: self.provider_name.clone(),
                    context_window: 200_000,
                    max_output_tokens: Some(65_536),
                    supports_vision: false,
                    supports_tools: true,
                    supports_streaming: true,
                    input_cost_per_million: Some(0.6),
                    output_cost_per_million: Some(2.4),
                },
            ]);
        }

        if self.provider_name.eq_ignore_ascii_case("minimax") {
            // Coding plan key: regular (non-highspeed) models
            return Ok(vec![
                ModelInfo {
                    id: "MiniMax-M2.5".to_string(),
                    name: "MiniMax M2.5".to_string(),
                    provider: self.provider_name.clone(),
                    context_window: 200_000,
                    max_output_tokens: Some(65_536),
                    supports_vision: false,
                    supports_tools: true,
                    supports_streaming: true,
                    input_cost_per_million: Some(0.3),
                    output_cost_per_million: Some(1.2),
                },
                ModelInfo {
                    id: "MiniMax-M2.1".to_string(),
                    name: "MiniMax M2.1".to_string(),
                    provider: self.provider_name.clone(),
                    context_window: 200_000,
                    max_output_tokens: Some(65_536),
                    supports_vision: false,
                    supports_tools: true,
                    supports_streaming: true,
                    input_cost_per_million: Some(0.3),
                    output_cost_per_million: Some(1.2),
                },
                ModelInfo {
                    id: "MiniMax-M2".to_string(),
                    name: "MiniMax M2".to_string(),
                    provider: self.provider_name.clone(),
                    context_window: 200_000,
                    max_output_tokens: Some(65_536),
                    supports_vision: false,
                    supports_tools: true,
                    supports_streaming: true,
                    input_cost_per_million: Some(0.3),
                    output_cost_per_million: Some(1.2),
                },
            ]);
        }

        Ok(vec![
            ModelInfo {
                id: "claude-sonnet-4-6".to_string(),
                name: "Claude Sonnet 4.6".to_string(),
                provider: self.provider_name.clone(),
                context_window: 200_000,
                max_output_tokens: Some(128_000),
                supports_vision: true,
                supports_tools: true,
                supports_streaming: true,
                input_cost_per_million: Some(3.0),
                output_cost_per_million: Some(15.0),
            },
            ModelInfo {
                id: "claude-sonnet-4-20250514".to_string(),
                name: "Claude Sonnet 4".to_string(),
                provider: self.provider_name.clone(),
                context_window: 200_000,
                max_output_tokens: Some(64_000),
                supports_vision: true,
                supports_tools: true,
                supports_streaming: true,
                input_cost_per_million: Some(3.0),
                output_cost_per_million: Some(15.0),
            },
            ModelInfo {
                id: "claude-opus-4-20250514".to_string(),
                name: "Claude Opus 4".to_string(),
                provider: self.provider_name.clone(),
                context_window: 200_000,
                max_output_tokens: Some(32_000),
                supports_vision: true,
                supports_tools: true,
                supports_streaming: true,
                input_cost_per_million: Some(15.0),
                output_cost_per_million: Some(75.0),
            },
            ModelInfo {
                id: "claude-haiku-3-5-20241022".to_string(),
                name: "Claude 3.5 Haiku".to_string(),
                provider: self.provider_name.clone(),
                context_window: 200_000,
                max_output_tokens: Some(8_192),
                supports_vision: true,
                supports_tools: true,
                supports_streaming: true,
                input_cost_per_million: Some(0.80),
                output_cost_per_million: Some(4.0),
            },
        ])
    }

    async fn complete(&self, request: CompletionRequest) -> Result<CompletionResponse> {
        tracing::debug!(
            provider = %self.provider_name,
            model = %request.model,
            message_count = request.messages.len(),
            tool_count = request.tools.len(),
            "Starting completion request"
        );

        self.validate_api_key()?;

        let (system_prompt, messages) =
            Self::convert_messages(&request.messages, self.enable_prompt_caching);
        let tools = Self::convert_tools(&request.tools, self.enable_prompt_caching);

        let mut body = json!({
            "model": request.model,
            "messages": messages,
            "max_tokens": request.max_tokens.unwrap_or(8192),
        });

        if let Some(system) = system_prompt {
            body["system"] = json!(system);
        }
        if !tools.is_empty() {
            body["tools"] = json!(tools);
        }
        if let Some(temp) = request.temperature {
            body["temperature"] = json!(temp);
        }
        if let Some(top_p) = request.top_p {
            body["top_p"] = json!(top_p);
        }

        tracing::debug!("Anthropic request to model {}", request.model);

        let response = self
            .client
            .post(format!(
                "{}/v1/messages",
                self.base_url.trim_end_matches('/')
            ))
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", ANTHROPIC_VERSION)
            .header("content-type", "application/json")
            .json(&body)
            .send()
            .await
            .context("Failed to send request to Anthropic")?;

        let status = response.status();
        let text = response
            .text()
            .await
            .context("Failed to read Anthropic response")?;

        if !status.is_success() {
            if let Ok(err) = serde_json::from_str::<AnthropicError>(&text) {
                anyhow::bail!(
                    "Anthropic API error: {} ({:?})",
                    err.error.message,
                    err.error.error_type
                );
            }
            anyhow::bail!("Anthropic API error: {} {}", status, text);
        }

        let response: AnthropicResponse = serde_json::from_str(&text).context(format!(
            "Failed to parse Anthropic response: {}",
            &text[..text.len().min(200)]
        ))?;

        tracing::debug!(
            response_id = %response.id,
            model = %response.model,
            stop_reason = ?response.stop_reason,
            "Received Anthropic response"
        );

        let mut content = Vec::new();
        let mut has_tool_calls = false;

        for part in &response.content {
            match part {
                AnthropicContent::Text { text } => {
                    if !text.is_empty() {
                        content.push(ContentPart::Text { text: text.clone() });
                    }
                }
                AnthropicContent::Thinking { thinking, text } => {
                    let reasoning = thinking
                        .as_deref()
                        .or(text.as_deref())
                        .unwrap_or_default()
                        .trim()
                        .to_string();
                    if !reasoning.is_empty() {
                        content.push(ContentPart::Thinking { text: reasoning });
                    }
                }
                AnthropicContent::ToolUse { id, name, input } => {
                    has_tool_calls = true;
                    content.push(ContentPart::ToolCall {
                        id: id.clone(),
                        name: name.clone(),
                        arguments: serde_json::to_string(input).unwrap_or_default(),
                        thought_signature: None,
                    });
                }
                AnthropicContent::Unknown => {}
            }
        }

        let finish_reason = if has_tool_calls {
            FinishReason::ToolCalls
        } else {
            match response.stop_reason.as_deref() {
                Some("end_turn") | Some("stop") => FinishReason::Stop,
                Some("max_tokens") => FinishReason::Length,
                Some("tool_use") => FinishReason::ToolCalls,
                Some("content_filter") => FinishReason::ContentFilter,
                _ => FinishReason::Stop,
            }
        };

        let usage = response.usage.as_ref();

        Ok(CompletionResponse {
            message: Message {
                role: Role::Assistant,
                content,
            },
            usage: Usage {
                prompt_tokens: usage.map(|u| u.input_tokens).unwrap_or(0),
                completion_tokens: usage.map(|u| u.output_tokens).unwrap_or(0),
                total_tokens: usage.map(|u| u.input_tokens + u.output_tokens).unwrap_or(0),
                cache_read_tokens: usage.and_then(|u| u.cache_read_input_tokens),
                cache_write_tokens: usage.and_then(|u| u.cache_creation_input_tokens),
            },
            finish_reason,
        })
    }

    async fn complete_stream(
        &self,
        request: CompletionRequest,
    ) -> Result<futures::stream::BoxStream<'static, StreamChunk>> {
        // Fall back to non-streaming for now
        let response = self.complete(request).await?;
        let text = response
            .message
            .content
            .iter()
            .filter_map(|p| match p {
                ContentPart::Text { text } => Some(text.clone()),
                _ => None,
            })
            .collect::<Vec<_>>()
            .join("");

        Ok(Box::pin(futures::stream::once(async move {
            StreamChunk::Text(text)
        })))
    }
}

fn parse_bool_env(value: &str) -> Option<bool> {
    let normalized = value.trim().to_ascii_lowercase();
    match normalized.as_str() {
        "1" | "true" | "yes" | "on" | "enabled" => Some(true),
        "0" | "false" | "no" | "off" | "disabled" => Some(false),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adds_cache_control_to_last_tool_system_and_message_block() {
        let messages = vec![
            Message {
                role: Role::System,
                content: vec![ContentPart::Text {
                    text: "static instruction".to_string(),
                }],
            },
            Message {
                role: Role::User,
                content: vec![ContentPart::Text {
                    text: "dynamic input".to_string(),
                }],
            },
        ];

        let (system, converted_messages) = AnthropicProvider::convert_messages(&messages, true);
        let mut converted_tools = AnthropicProvider::convert_tools(
            &[ToolDefinition {
                name: "get_weather".to_string(),
                description: "Get weather".to_string(),
                parameters: json!({"type": "object"}),
            }],
            true,
        );

        let system = system.expect("system blocks should be present");
        let system_cache = system
            .last()
            .and_then(|v| v.get("cache_control"))
            .and_then(|v| v.get("type"))
            .and_then(Value::as_str);
        assert_eq!(system_cache, Some("ephemeral"));

        let message_cache = converted_messages
            .last()
            .and_then(|msg| msg.get("content"))
            .and_then(Value::as_array)
            .and_then(|parts| parts.last())
            .and_then(|part| part.get("cache_control"))
            .and_then(|v| v.get("type"))
            .and_then(Value::as_str);
        assert_eq!(message_cache, Some("ephemeral"));

        let tool_cache = converted_tools
            .pop()
            .and_then(|tool| tool.get("cache_control").cloned())
            .and_then(|v| v.get("type").cloned())
            .and_then(|v| v.as_str().map(str::to_string));
        assert_eq!(tool_cache.as_deref(), Some("ephemeral"));
    }

    #[test]
    fn minimax_provider_name_enables_prompt_caching_by_default() {
        let provider = AnthropicProvider::with_base_url(
            "test-key".to_string(),
            "https://api.minimax.io/anthropic".to_string(),
            "minimax",
        )
        .expect("provider should initialize");

        assert_eq!(provider.name(), "minimax");
        assert!(provider.enable_prompt_caching);
    }
}
