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
}

impl std::fmt::Debug for AnthropicProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AnthropicProvider")
            .field("api_key", &"<REDACTED>")
            .field("api_key_len", &self.api_key.len())
            .finish()
    }
}

impl AnthropicProvider {
    pub fn new(api_key: String) -> Result<Self> {
        tracing::debug!(
            provider = "anthropic",
            api_key_len = api_key.len(),
            "Creating Anthropic provider"
        );
        Ok(Self {
            client: Client::new(),
            api_key,
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
    fn convert_messages(messages: &[Message]) -> (Option<String>, Vec<Value>) {
        let mut system_prompt = None;
        let mut api_messages: Vec<Value> = Vec::new();

        for msg in messages {
            match msg.role {
                Role::System => {
                    let text: String = msg
                        .content
                        .iter()
                        .filter_map(|p| match p {
                            ContentPart::Text { text } => Some(text.clone()),
                            _ => None,
                        })
                        .collect::<Vec<_>>()
                        .join("\n");
                    system_prompt = Some(match system_prompt {
                        Some(existing) => format!("{}\n{}", existing, text),
                        None => text,
                    });
                }
                Role::User => {
                    let text: String = msg
                        .content
                        .iter()
                        .filter_map(|p| match p {
                            ContentPart::Text { text } => Some(text.clone()),
                            _ => None,
                        })
                        .collect::<Vec<_>>()
                        .join("\n");
                    api_messages.push(json!({
                        "role": "user",
                        "content": text
                    }));
                }
                Role::Assistant => {
                    let mut content_parts: Vec<Value> = Vec::new();

                    for part in &msg.content {
                        match part {
                            ContentPart::Text { text } => {
                                if !text.is_empty() {
                                    content_parts.push(json!({
                                        "type": "text",
                                        "text": text
                                    }));
                                }
                            }
                            ContentPart::ToolCall {
                                id,
                                name,
                                arguments,
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

        (system_prompt, api_messages)
    }

    fn convert_tools(tools: &[ToolDefinition]) -> Vec<Value> {
        tools
            .iter()
            .map(|t| {
                json!({
                    "name": t.name,
                    "description": t.description,
                    "input_schema": t.parameters
                })
            })
            .collect()
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
    #[serde(rename = "tool_use")]
    ToolUse {
        id: String,
        name: String,
        input: Value,
    },
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
        "anthropic"
    }

    async fn list_models(&self) -> Result<Vec<ModelInfo>> {
        self.validate_api_key()?;

        Ok(vec![
            ModelInfo {
                id: "claude-sonnet-4-20250514".to_string(),
                name: "Claude Sonnet 4".to_string(),
                provider: "anthropic".to_string(),
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
                provider: "anthropic".to_string(),
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
                provider: "anthropic".to_string(),
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
            provider = "anthropic",
            model = %request.model,
            message_count = request.messages.len(),
            tool_count = request.tools.len(),
            "Starting completion request"
        );

        self.validate_api_key()?;

        let (system_prompt, messages) = Self::convert_messages(&request.messages);
        let tools = Self::convert_tools(&request.tools);

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
            .post(format!("{}/v1/messages", ANTHROPIC_API_BASE))
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
                AnthropicContent::ToolUse { id, name, input } => {
                    has_tool_calls = true;
                    content.push(ContentPart::ToolCall {
                        id: id.clone(),
                        name: name.clone(),
                        arguments: serde_json::to_string(input).unwrap_or_default(),
                    });
                }
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
