//! OpenRouter provider implementation using raw HTTP
//!
//! This provider uses reqwest directly instead of async_openai to handle
//! OpenRouter's extended response formats (like Kimi's reasoning fields).

use super::{
    CompletionRequest, CompletionResponse, ContentPart, FinishReason, Message, ModelInfo, Provider,
    Role, StreamChunk, ToolDefinition, Usage,
};
use anyhow::{Context, Result};
use async_trait::async_trait;
use reqwest::Client;
use serde::Deserialize;
use serde_json::{Value, json};

pub struct OpenRouterProvider {
    client: Client,
    api_key: String,
    base_url: String,
}

impl OpenRouterProvider {
    pub fn new(api_key: String) -> Result<Self> {
        Ok(Self {
            client: Client::new(),
            api_key,
            base_url: "https://openrouter.ai/api/v1".to_string(),
        })
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
                        // Tool result message
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
                        // Assistant message - may have tool calls
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
                                } => Some(json!({
                                    "id": id,
                                    "type": "function",
                                    "function": {
                                        "name": name,
                                        "arguments": arguments
                                    }
                                })),
                                _ => None,
                            })
                            .collect();

                        if tool_calls.is_empty() {
                            json!({"role": "assistant", "content": text})
                        } else {
                            // For assistant with tool calls, content should be empty string or the text
                            json!({
                                "role": "assistant",
                                "content": if text.is_empty() { "".to_string() } else { text },
                                "tool_calls": tool_calls
                            })
                        }
                    }
                    _ => {
                        // System or User message
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
}

#[derive(Debug, Deserialize)]
struct OpenRouterResponse {
    #[serde(default)]
    id: String,
    // provider and model fields from OpenRouter
    #[serde(default)]
    provider: Option<String>,
    #[serde(default)]
    model: Option<String>,
    choices: Vec<OpenRouterChoice>,
    #[serde(default)]
    usage: Option<OpenRouterUsage>,
}

#[derive(Debug, Deserialize)]
struct OpenRouterChoice {
    message: OpenRouterMessage,
    #[serde(default)]
    finish_reason: Option<String>,
    // OpenRouter adds native_finish_reason
    #[serde(default)]
    native_finish_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
struct OpenRouterMessage {
    role: String,
    #[serde(default)]
    content: Option<String>,
    #[serde(default)]
    tool_calls: Option<Vec<OpenRouterToolCall>>,
    // Extended fields from thinking models like Kimi K2.5
    #[serde(default)]
    reasoning: Option<String>,
    #[serde(default)]
    reasoning_details: Option<Vec<Value>>,
    #[serde(default)]
    refusal: Option<String>,
}

#[derive(Debug, Deserialize)]
struct OpenRouterToolCall {
    id: String,
    #[serde(rename = "type")]
    #[allow(dead_code)]
    call_type: String,
    function: OpenRouterFunction,
    #[serde(default)]
    #[allow(dead_code)]
    index: Option<usize>,
}

#[derive(Debug, Deserialize)]
struct OpenRouterFunction {
    name: String,
    arguments: String,
}

#[derive(Debug, Deserialize)]
struct OpenRouterUsage {
    #[serde(default)]
    prompt_tokens: usize,
    #[serde(default)]
    completion_tokens: usize,
    #[serde(default)]
    total_tokens: usize,
}

#[derive(Debug, Deserialize)]
struct OpenRouterError {
    error: OpenRouterErrorDetail,
}

#[derive(Debug, Deserialize)]
struct OpenRouterErrorDetail {
    message: String,
    #[serde(default)]
    code: Option<i32>,
}

#[async_trait]
impl Provider for OpenRouterProvider {
    fn name(&self) -> &str {
        "openrouter"
    }

    async fn list_models(&self) -> Result<Vec<ModelInfo>> {
        // Fetch models from OpenRouter API
        let response = self
            .client
            .get(format!("{}/models", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await
            .context("Failed to fetch models")?;

        if !response.status().is_success() {
            return Ok(vec![]); // Return empty on error
        }

        #[derive(Deserialize)]
        struct ModelsResponse {
            data: Vec<ModelData>,
        }

        #[derive(Deserialize)]
        struct ModelData {
            id: String,
            #[serde(default)]
            name: Option<String>,
            #[serde(default)]
            context_length: Option<usize>,
        }

        let models: ModelsResponse = response
            .json()
            .await
            .unwrap_or(ModelsResponse { data: vec![] });

        Ok(models
            .data
            .into_iter()
            .map(|m| ModelInfo {
                id: m.id.clone(),
                name: m.name.unwrap_or_else(|| m.id.clone()),
                provider: "openrouter".to_string(),
                context_window: m.context_length.unwrap_or(128_000),
                max_output_tokens: Some(16_384),
                supports_vision: false,
                supports_tools: true,
                supports_streaming: true,
                input_cost_per_million: None,
                output_cost_per_million: None,
            })
            .collect())
    }

    async fn complete(&self, request: CompletionRequest) -> Result<CompletionResponse> {
        let messages = Self::convert_messages(&request.messages);
        let tools = Self::convert_tools(&request.tools);

        // Build request body
        let mut body = json!({
            "model": request.model,
            "messages": messages,
        });

        if !tools.is_empty() {
            body["tools"] = json!(tools);
        }
        if let Some(temp) = request.temperature {
            body["temperature"] = json!(temp);
        }
        if let Some(max) = request.max_tokens {
            body["max_tokens"] = json!(max);
        }

        tracing::debug!(
            "OpenRouter request: {}",
            serde_json::to_string_pretty(&body).unwrap_or_default()
        );

        let response = self
            .client
            .post(format!("{}/chat/completions", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .header("HTTP-Referer", "https://codetether.run")
            .header("X-Title", "CodeTether Agent")
            .json(&body)
            .send()
            .await
            .context("Failed to send request")?;

        let status = response.status();
        let text = response.text().await.context("Failed to read response")?;

        if !status.is_success() {
            // Try to parse as error response
            if let Ok(err) = serde_json::from_str::<OpenRouterError>(&text) {
                anyhow::bail!(
                    "OpenRouter API error: {} (code: {:?})",
                    err.error.message,
                    err.error.code
                );
            }
            anyhow::bail!("OpenRouter API error: {} {}", status, text);
        }

        tracing::debug!("OpenRouter response: {}", &text[..text.len().min(500)]);

        let response: OpenRouterResponse = serde_json::from_str(&text).context(format!(
            "Failed to parse response: {}",
            &text[..text.len().min(200)]
        ))?;

        // Log response metadata for debugging
        tracing::debug!(
            response_id = %response.id,
            provider = ?response.provider,
            model = ?response.model,
            "Received OpenRouter response"
        );

        let choice = response
            .choices
            .first()
            .ok_or_else(|| anyhow::anyhow!("No choices"))?;

        // Log native finish reason if present
        if let Some(ref native_reason) = choice.native_finish_reason {
            tracing::debug!(native_finish_reason = %native_reason, "OpenRouter native finish reason");
        }

        // Log reasoning content if present (e.g., Kimi K2 models)
        if let Some(ref reasoning) = choice.message.reasoning
            && !reasoning.is_empty()
        {
            tracing::info!(
                reasoning_len = reasoning.len(),
                "Model reasoning content received"
            );
            tracing::debug!(
                reasoning = %reasoning,
                "Full model reasoning"
            );
        }
        if let Some(ref details) = choice.message.reasoning_details
            && !details.is_empty()
        {
            tracing::debug!(
                reasoning_details = ?details,
                "Model reasoning details"
            );
        }

        let mut content = Vec::new();
        let mut has_tool_calls = false;

        // Add text content if present
        if let Some(text) = &choice.message.content
            && !text.is_empty()
        {
            content.push(ContentPart::Text { text: text.clone() });
        }

        // Log message role for debugging
        tracing::debug!(message_role = %choice.message.role, "OpenRouter message role");

        // Log refusal if present (model declined to respond)
        if let Some(ref refusal) = choice.message.refusal {
            tracing::warn!(refusal = %refusal, "Model refused to respond");
        }

        // Add tool calls if present
        if let Some(tool_calls) = &choice.message.tool_calls {
            has_tool_calls = !tool_calls.is_empty();
            for tc in tool_calls {
                // Log tool call details (uses call_type and index fields)
                tracing::debug!(
                    tool_call_id = %tc.id,
                    call_type = %tc.call_type,
                    index = ?tc.index,
                    function_name = %tc.function.name,
                    "Processing OpenRouter tool call"
                );
                content.push(ContentPart::ToolCall {
                    id: tc.id.clone(),
                    name: tc.function.name.clone(),
                    arguments: tc.function.arguments.clone(),
                    thought_signature: None,
                });
            }
        }

        // Determine finish reason
        let finish_reason = if has_tool_calls {
            FinishReason::ToolCalls
        } else {
            match choice.finish_reason.as_deref() {
                Some("stop") => FinishReason::Stop,
                Some("length") => FinishReason::Length,
                Some("tool_calls") => FinishReason::ToolCalls,
                Some("content_filter") => FinishReason::ContentFilter,
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
                ..Default::default()
            },
            finish_reason,
        })
    }

    async fn complete_stream(
        &self,
        request: CompletionRequest,
    ) -> Result<futures::stream::BoxStream<'static, StreamChunk>> {
        tracing::debug!(
            provider = "openrouter",
            model = %request.model,
            message_count = request.messages.len(),
            "Starting streaming completion request (falling back to non-streaming)"
        );

        // For now, fall back to non-streaming
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
