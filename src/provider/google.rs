//! Google Gemini provider implementation
//!
//! Uses the Google AI Gemini OpenAI-compatible endpoint for simplicity.
//! Reference: https://ai.google.dev/gemini-api/docs/openai

use super::{
    CompletionRequest, CompletionResponse, ContentPart, FinishReason, Message, ModelInfo, Provider,
    Role, StreamChunk, ToolDefinition, Usage,
};
use anyhow::{Context, Result};
use async_trait::async_trait;
use reqwest::Client;
use serde::Deserialize;
use serde_json::{Value, json};

const GOOGLE_OPENAI_BASE: &str = "https://generativelanguage.googleapis.com/v1beta/openai";

pub struct GoogleProvider {
    client: Client,
    api_key: String,
}

impl std::fmt::Debug for GoogleProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GoogleProvider")
            .field("api_key", &"<REDACTED>")
            .field("api_key_len", &self.api_key.len())
            .finish()
    }
}

impl GoogleProvider {
    pub fn new(api_key: String) -> Result<Self> {
        tracing::debug!(
            provider = "google",
            api_key_len = api_key.len(),
            "Creating Google Gemini provider"
        );
        Ok(Self {
            client: Client::new(),
            api_key,
        })
    }

    fn validate_api_key(&self) -> Result<()> {
        if self.api_key.is_empty() {
            anyhow::bail!("Google API key is empty");
        }
        Ok(())
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

                // For tool messages, we need to produce one message per tool result
                if msg.role == Role::Tool {
                    let mut content_parts: Vec<Value> = Vec::new();
                    let mut tool_call_id = None;
                    for part in &msg.content {
                        match part {
                            ContentPart::ToolResult {
                                tool_call_id: id,
                                content,
                            } => {
                                tool_call_id = Some(id.clone());
                                content_parts.push(json!(content));
                            }
                            ContentPart::Text { text } => {
                                content_parts.push(json!(text));
                            }
                            _ => {}
                        }
                    }
                    let content_str = content_parts
                        .iter()
                        .filter_map(|v| v.as_str())
                        .collect::<Vec<_>>()
                        .join("\n");
                    let mut m = json!({
                        "role": "tool",
                        "content": content_str,
                    });
                    if let Some(id) = tool_call_id {
                        m["tool_call_id"] = json!(id);
                    }
                    return m;
                }

                // For assistant messages with tool calls
                if msg.role == Role::Assistant {
                    let mut text_parts = Vec::new();
                    let mut tool_calls = Vec::new();
                    for part in &msg.content {
                        match part {
                            ContentPart::Text { text } => {
                                if !text.is_empty() {
                                    text_parts.push(text.clone());
                                }
                            }
                            ContentPart::ToolCall {
                                id,
                                name,
                                arguments,
                                thought_signature,
                            } => {
                                let mut tc = json!({
                                    "id": id,
                                    "type": "function",
                                    "function": {
                                        "name": name,
                                        "arguments": arguments
                                    }
                                });
                                // Include thought signature for Gemini 3.x models
                                if let Some(sig) = thought_signature {
                                    tc["extra_content"] = json!({
                                        "google": {
                                            "thought_signature": sig
                                        }
                                    });
                                }
                                tool_calls.push(tc);
                            }
                            _ => {}
                        }
                    }
                    let content = text_parts.join("\n");
                    let mut m = json!({"role": "assistant"});
                    if !content.is_empty() || tool_calls.is_empty() {
                        m["content"] = json!(content);
                    }
                    if !tool_calls.is_empty() {
                        m["tool_calls"] = json!(tool_calls);
                    }
                    return m;
                }

                let text: String = msg
                    .content
                    .iter()
                    .filter_map(|p| match p {
                        ContentPart::Text { text } => Some(text.clone()),
                        _ => None,
                    })
                    .collect::<Vec<_>>()
                    .join("\n");

                json!({
                    "role": role,
                    "content": text
                })
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

/// OpenAI-compatible types for parsing Google's response

#[derive(Debug, Deserialize)]
struct ChatCompletion {
    #[allow(dead_code)]
    id: Option<String>,
    choices: Vec<Choice>,
    #[serde(default)]
    usage: Option<ApiUsage>,
}

#[derive(Debug, Deserialize)]
struct Choice {
    message: ChoiceMessage,
    #[serde(default)]
    finish_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ChoiceMessage {
    #[allow(dead_code)]
    role: Option<String>,
    #[serde(default)]
    content: Option<String>,
    #[serde(default)]
    tool_calls: Option<Vec<ToolCall>>,
}

#[derive(Debug, Deserialize)]
struct ToolCall {
    id: String,
    function: FunctionCall,
    /// Thought signature for Gemini 3.x models
    #[serde(default)]
    extra_content: Option<ExtraContent>,
}

#[derive(Debug, Deserialize)]
struct ExtraContent {
    google: Option<GoogleExtra>,
}

#[derive(Debug, Deserialize)]
struct GoogleExtra {
    thought_signature: Option<String>,
}

#[derive(Debug, Deserialize)]
struct FunctionCall {
    name: String,
    arguments: String,
}

#[derive(Debug, Deserialize)]
struct ApiUsage {
    #[serde(default)]
    prompt_tokens: usize,
    #[serde(default)]
    completion_tokens: usize,
    #[serde(default)]
    total_tokens: usize,
}

#[derive(Debug, Deserialize)]
struct ApiError {
    error: ApiErrorDetail,
}

#[derive(Debug, Deserialize)]
struct ApiErrorDetail {
    message: String,
}

#[async_trait]
impl Provider for GoogleProvider {
    fn name(&self) -> &str {
        "google"
    }

    async fn list_models(&self) -> Result<Vec<ModelInfo>> {
        self.validate_api_key()?;

        Ok(vec![
            // Gemini 3.x models (require thought signatures for tool calls)
            ModelInfo {
                id: "gemini-3.1-pro-preview".to_string(),
                name: "Gemini 3.1 Pro Preview".to_string(),
                provider: "google".to_string(),
                context_window: 1_048_576,
                max_output_tokens: Some(65_536),
                supports_vision: true,
                supports_tools: true,
                supports_streaming: true,
                input_cost_per_million: Some(1.25),
                output_cost_per_million: Some(10.0),
            },
            ModelInfo {
                id: "gemini-3-pro".to_string(),
                name: "Gemini 3 Pro".to_string(),
                provider: "google".to_string(),
                context_window: 1_048_576,
                max_output_tokens: Some(65_536),
                supports_vision: true,
                supports_tools: true,
                supports_streaming: true,
                input_cost_per_million: Some(1.25),
                output_cost_per_million: Some(10.0),
            },
            ModelInfo {
                id: "gemini-3-flash".to_string(),
                name: "Gemini 3 Flash".to_string(),
                provider: "google".to_string(),
                context_window: 1_048_576,
                max_output_tokens: Some(65_536),
                supports_vision: true,
                supports_tools: true,
                supports_streaming: true,
                input_cost_per_million: Some(0.15),
                output_cost_per_million: Some(0.60),
            },
            // Gemini 2.5 models
            ModelInfo {
                id: "gemini-2.5-pro".to_string(),
                name: "Gemini 2.5 Pro".to_string(),
                provider: "google".to_string(),
                context_window: 1_048_576,
                max_output_tokens: Some(65_536),
                supports_vision: true,
                supports_tools: true,
                supports_streaming: true,
                input_cost_per_million: Some(1.25),
                output_cost_per_million: Some(10.0),
            },
            ModelInfo {
                id: "gemini-2.5-flash".to_string(),
                name: "Gemini 2.5 Flash".to_string(),
                provider: "google".to_string(),
                context_window: 1_048_576,
                max_output_tokens: Some(65_536),
                supports_vision: true,
                supports_tools: true,
                supports_streaming: true,
                input_cost_per_million: Some(0.15),
                output_cost_per_million: Some(0.60),
            },
            ModelInfo {
                id: "gemini-2.0-flash".to_string(),
                name: "Gemini 2.0 Flash".to_string(),
                provider: "google".to_string(),
                context_window: 1_048_576,
                max_output_tokens: Some(8_192),
                supports_vision: true,
                supports_tools: true,
                supports_streaming: true,
                input_cost_per_million: Some(0.10),
                output_cost_per_million: Some(0.40),
            },
        ])
    }

    async fn complete(&self, request: CompletionRequest) -> Result<CompletionResponse> {
        tracing::debug!(
            provider = "google",
            model = %request.model,
            message_count = request.messages.len(),
            tool_count = request.tools.len(),
            "Starting Google Gemini completion request"
        );

        self.validate_api_key()?;

        let messages = Self::convert_messages(&request.messages);
        let tools = Self::convert_tools(&request.tools);

        let mut body = json!({
            "model": request.model,
            "messages": messages,
        });

        if let Some(max_tokens) = request.max_tokens {
            body["max_tokens"] = json!(max_tokens);
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

        tracing::debug!("Google Gemini request to model {}", request.model);

        // Google AI Studio OpenAI-compatible endpoint uses Bearer token auth
        let url = format!("{}/chat/completions", GOOGLE_OPENAI_BASE);
        let response = self
            .client
            .post(&url)
            .header("content-type", "application/json")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&body)
            .send()
            .await
            .context("Failed to send request to Google Gemini")?;

        let status = response.status();
        let text = response
            .text()
            .await
            .context("Failed to read Google Gemini response")?;

        if !status.is_success() {
            if let Ok(err) = serde_json::from_str::<ApiError>(&text) {
                anyhow::bail!("Google Gemini API error: {}", err.error.message);
            }
            anyhow::bail!("Google Gemini API error: {} {}", status, text);
        }

        let completion: ChatCompletion = serde_json::from_str(&text).context(format!(
            "Failed to parse Google Gemini response: {}",
            &text[..text.len().min(200)]
        ))?;

        let choice = completion
            .choices
            .into_iter()
            .next()
            .context("No choices in Google Gemini response")?;

        let mut content_parts = Vec::new();
        let mut has_tool_calls = false;

        if let Some(text) = choice.message.content {
            if !text.is_empty() {
                content_parts.push(ContentPart::Text { text });
            }
        }

        if let Some(tool_calls) = choice.message.tool_calls {
            has_tool_calls = !tool_calls.is_empty();
            for tc in tool_calls {
                // Extract thought signature from extra_content.google.thought_signature
                let thought_signature = tc
                    .extra_content
                    .as_ref()
                    .and_then(|ec| ec.google.as_ref())
                    .and_then(|g| g.thought_signature.clone());
                
                content_parts.push(ContentPart::ToolCall {
                    id: tc.id,
                    name: tc.function.name,
                    arguments: tc.function.arguments,
                    thought_signature,
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
                Some("content_filter") => FinishReason::ContentFilter,
                _ => FinishReason::Stop,
            }
        };

        let usage = completion.usage.as_ref();

        Ok(CompletionResponse {
            message: Message {
                role: Role::Assistant,
                content: content_parts,
            },
            usage: Usage {
                prompt_tokens: usage.map(|u| u.prompt_tokens).unwrap_or(0),
                completion_tokens: usage.map(|u| u.completion_tokens).unwrap_or(0),
                total_tokens: usage.map(|u| u.total_tokens).unwrap_or(0),
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
