//! Moonshot AI provider implementation (direct API)
//!
//! For Kimi K2.5 and other Moonshot models via api.moonshot.ai

use super::{
    CompletionRequest, CompletionResponse, ContentPart, FinishReason, Message, ModelInfo, Provider,
    Role, StreamChunk, ToolDefinition, Usage,
};
use anyhow::{Context, Result};
use async_trait::async_trait;
use reqwest::Client;
use serde::Deserialize;
use serde_json::{json, Value};

pub struct MoonshotProvider {
    client: Client,
    api_key: String,
    base_url: String,
}

impl MoonshotProvider {
    pub fn new(api_key: String) -> Result<Self> {
        Ok(Self {
            client: Client::new(),
            api_key,
            base_url: "https://api.moonshot.ai/v1".to_string(),
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
                        if let Some(ContentPart::ToolResult { tool_call_id, content }) = msg.content.first() {
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
                        let text: String = msg.content.iter()
                            .filter_map(|p| match p {
                                ContentPart::Text { text } => Some(text.clone()),
                                _ => None,
                            })
                            .collect::<Vec<_>>()
                            .join("");

                        let tool_calls: Vec<Value> = msg.content.iter()
                            .filter_map(|p| match p {
                                ContentPart::ToolCall { id, name, arguments } => Some(json!({
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
                            // Moonshot requires reasoning_content for K2.5 thinking models
                            // Include empty string when we don't have the original
                            json!({
                                "role": "assistant",
                                "content": if text.is_empty() { "".to_string() } else { text },
                                "reasoning_content": "",
                                "tool_calls": tool_calls
                            })
                        }
                    }
                    _ => {
                        let text: String = msg.content.iter()
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
struct MoonshotResponse {
    id: String,
    model: String,
    choices: Vec<MoonshotChoice>,
    #[serde(default)]
    usage: Option<MoonshotUsage>,
}

#[derive(Debug, Deserialize)]
struct MoonshotChoice {
    message: MoonshotMessage,
    #[serde(default)]
    finish_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
struct MoonshotMessage {
    #[allow(dead_code)]
    role: String,
    #[serde(default)]
    content: Option<String>,
    #[serde(default)]
    tool_calls: Option<Vec<MoonshotToolCall>>,
    // Kimi K2.5 reasoning
    #[serde(default)]
    reasoning_content: Option<String>,
}

#[derive(Debug, Deserialize)]
struct MoonshotToolCall {
    id: String,
    #[serde(rename = "type")]
    call_type: String,
    function: MoonshotFunction,
}

#[derive(Debug, Deserialize)]
struct MoonshotFunction {
    name: String,
    arguments: String,
}

#[derive(Debug, Deserialize)]
struct MoonshotUsage {
    #[serde(default)]
    prompt_tokens: usize,
    #[serde(default)]
    completion_tokens: usize,
    #[serde(default)]
    total_tokens: usize,
}

#[derive(Debug, Deserialize)]
struct MoonshotError {
    #[allow(dead_code)]
    error: MoonshotErrorDetail,
}

#[derive(Debug, Deserialize)]
struct MoonshotErrorDetail {
    message: String,
    #[serde(default, rename = "type")]
    error_type: Option<String>,
}

#[async_trait]
impl Provider for MoonshotProvider {
    fn name(&self) -> &str {
        "moonshotai"
    }

    async fn list_models(&self) -> Result<Vec<ModelInfo>> {
        Ok(vec![
            ModelInfo {
                id: "kimi-k2.5".to_string(),
                name: "Kimi K2.5".to_string(),
                provider: "moonshotai".to_string(),
                context_window: 256_000,
                max_output_tokens: Some(64_000),
                supports_vision: true,
                supports_tools: true,
                supports_streaming: true,
                input_cost_per_million: Some(0.56),  // ¥4/M tokens
                output_cost_per_million: Some(2.8),  // ¥20/M tokens
            },
            ModelInfo {
                id: "kimi-k2-thinking".to_string(),
                name: "Kimi K2 Thinking".to_string(),
                provider: "moonshotai".to_string(),
                context_window: 128_000,
                max_output_tokens: Some(64_000),
                supports_vision: false,
                supports_tools: true,
                supports_streaming: true,
                input_cost_per_million: Some(0.56),
                output_cost_per_million: Some(2.8),
            },
            ModelInfo {
                id: "kimi-latest".to_string(),
                name: "Kimi Latest".to_string(),
                provider: "moonshotai".to_string(),
                context_window: 128_000,
                max_output_tokens: Some(64_000),
                supports_vision: false,
                supports_tools: true,
                supports_streaming: true,
                input_cost_per_million: Some(0.42),  // Cheaper
                output_cost_per_million: Some(1.68),
            },
        ])
    }

    async fn complete(&self, request: CompletionRequest) -> Result<CompletionResponse> {
        let messages = Self::convert_messages(&request.messages);
        let tools = Self::convert_tools(&request.tools);

        // Kimi K2.5 requires specific temperatures:
        // - temperature = 1.0 when thinking is enabled  
        // - temperature = 0.6 when thinking is disabled
        let temperature = if request.model.contains("k2") {
            0.6  // We disable thinking for tool calling workflows
        } else {
            request.temperature.unwrap_or(0.7)
        };

        let mut body = json!({
            "model": request.model,
            "messages": messages,
            "temperature": temperature,
        });

        // Disable thinking mode to avoid needing to track reasoning_content
        // across message roundtrips (required for K2.5)
        if request.model.contains("k2") {
            body["thinking"] = json!({"type": "disabled"});
        }

        if !tools.is_empty() {
            body["tools"] = json!(tools);
        }
        if let Some(max) = request.max_tokens {
            body["max_tokens"] = json!(max);
        }

        tracing::debug!("Moonshot request to model {}", request.model);

        let response = self
            .client
            .post(format!("{}/chat/completions", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .context("Failed to send request to Moonshot")?;

        let status = response.status();
        let text = response.text().await.context("Failed to read response")?;

        if !status.is_success() {
            if let Ok(err) = serde_json::from_str::<MoonshotError>(&text) {
                anyhow::bail!("Moonshot API error: {} ({:?})", err.error.message, err.error.error_type);
            }
            anyhow::bail!("Moonshot API error: {} {}", status, text);
        }

        let response: MoonshotResponse = serde_json::from_str(&text)
            .context(format!("Failed to parse Moonshot response: {}", &text[..text.len().min(200)]))?;

        // Log response metadata for debugging
        tracing::debug!(
            response_id = %response.id,
            model = %response.model,
            "Received Moonshot response"
        );

        let choice = response.choices.first().ok_or_else(|| anyhow::anyhow!("No choices"))?;

        // Log reasoning/thinking content if present (Kimi K2 models)
        if let Some(ref reasoning) = choice.message.reasoning_content {
            if !reasoning.is_empty() {
                tracing::info!(
                    reasoning_len = reasoning.len(),
                    "Model reasoning/thinking content received"
                );
                tracing::debug!(
                    reasoning = %reasoning,
                    "Full model reasoning"
                );
            }
        }

        let mut content = Vec::new();
        let mut has_tool_calls = false;

        if let Some(text) = &choice.message.content {
            if !text.is_empty() {
                content.push(ContentPart::Text { text: text.clone() });
            }
        }

        if let Some(tool_calls) = &choice.message.tool_calls {
            has_tool_calls = !tool_calls.is_empty();
            for tc in tool_calls {
                // Log tool call details for debugging (uses role and call_type fields)
                tracing::debug!(
                    tool_call_id = %tc.id,
                    call_type = %tc.call_type,
                    function_name = %tc.function.name,
                    "Processing tool call"
                );
                content.push(ContentPart::ToolCall {
                    id: tc.id.clone(),
                    name: tc.function.name.clone(),
                    arguments: tc.function.arguments.clone(),
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
                prompt_tokens: response.usage.as_ref().map(|u| u.prompt_tokens).unwrap_or(0),
                completion_tokens: response.usage.as_ref().map(|u| u.completion_tokens).unwrap_or(0),
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
            provider = "moonshotai",
            model = %request.model,
            message_count = request.messages.len(),
            "Starting streaming completion request (falling back to non-streaming)"
        );
        
        // Fall back to non-streaming for now
        let response = self.complete(request).await?;
        let text = response.message.content.iter()
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
