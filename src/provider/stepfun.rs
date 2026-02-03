//! StepFun provider implementation (direct API, not via OpenRouter)
//!
//! StepFun models: step-1-8k, step-1-32k, step-1-128k, step-1-256k, step-3.5-flash

use super::{
    CompletionRequest, CompletionResponse, ContentPart, FinishReason, Message, ModelInfo, Provider,
    Role, StreamChunk, ToolDefinition, Usage,
};
use anyhow::Result;
use async_trait::async_trait;
use futures::StreamExt;
use serde::{Deserialize, Serialize};

const STEPFUN_API_BASE: &str = "https://api.stepfun.ai/v1";

pub struct StepFunProvider {
    api_key: String,
    client: reqwest::Client,
}

impl StepFunProvider {
    pub fn new(api_key: String) -> Result<Self> {
        Ok(Self {
            api_key,
            client: reqwest::Client::new(),
        })
    }
}

// ============== Request Types ==============

#[derive(Debug, Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<ChatMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tools: Option<Vec<ChatTool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tokens: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stream: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ChatMessage {
    role: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_calls: Option<Vec<ToolCall>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_call_id: Option<String>,
}

#[derive(Debug, Serialize)]
struct ChatTool {
    r#type: String,
    function: ChatFunction,
}

#[derive(Debug, Serialize)]
struct ChatFunction {
    name: String,
    description: String,
    parameters: serde_json::Value,
}

// ============== Response Types ==============

#[derive(Debug, Deserialize)]
struct ChatResponse {
    id: String,
    choices: Vec<ChatChoice>,
    usage: Option<ChatUsage>,
}

#[derive(Debug, Deserialize)]
struct ChatChoice {
    index: usize,
    message: ChatResponseMessage,
    finish_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ChatResponseMessage {
    role: String,
    #[serde(default)]
    content: Option<String>,
    #[serde(default)]
    tool_calls: Option<Vec<ToolCall>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ToolCall {
    id: String,
    r#type: String,
    function: ToolCallFunction,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ToolCallFunction {
    name: String,
    arguments: String,
}

#[derive(Debug, Deserialize)]
struct ChatUsage {
    prompt_tokens: usize,
    completion_tokens: usize,
    total_tokens: usize,
}

#[derive(Debug, Deserialize)]
struct ErrorResponse {
    error: ErrorDetail,
}

#[derive(Debug, Deserialize)]
struct ErrorDetail {
    message: String,
    #[serde(default)]
    code: Option<String>,
}

// ============== Streaming Types ==============

#[derive(Debug, Deserialize)]
struct StreamChunkResponse {
    choices: Vec<StreamChoice>,
}

#[derive(Debug, Deserialize)]
struct StreamChoice {
    delta: StreamDelta,
    finish_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
struct StreamDelta {
    #[serde(default)]
    content: Option<String>,
    #[serde(default)]
    tool_calls: Option<Vec<StreamToolCall>>,
}

#[derive(Debug, Deserialize)]
struct StreamToolCall {
    index: usize,
    #[serde(default)]
    id: Option<String>,
    #[serde(default)]
    function: Option<StreamToolFunction>,
}

#[derive(Debug, Deserialize)]
struct StreamToolFunction {
    #[serde(default)]
    name: Option<String>,
    #[serde(default)]
    arguments: Option<String>,
}

impl StepFunProvider {
    fn convert_messages(&self, messages: &[Message]) -> Vec<ChatMessage> {
        let mut result = Vec::new();

        for msg in messages {
            match msg.role {
                Role::System => {
                    let content = msg
                        .content
                        .iter()
                        .filter_map(|p| match p {
                            ContentPart::Text { text } => Some(text.clone()),
                            _ => None,
                        })
                        .collect::<Vec<_>>()
                        .join("\n");
                    result.push(ChatMessage {
                        role: "system".to_string(),
                        content: Some(content),
                        tool_calls: None,
                        tool_call_id: None,
                    });
                }
                Role::User => {
                    let content = msg
                        .content
                        .iter()
                        .filter_map(|p| match p {
                            ContentPart::Text { text } => Some(text.clone()),
                            _ => None,
                        })
                        .collect::<Vec<_>>()
                        .join("\n");
                    result.push(ChatMessage {
                        role: "user".to_string(),
                        content: Some(content),
                        tool_calls: None,
                        tool_call_id: None,
                    });
                }
                Role::Assistant => {
                    let content = msg
                        .content
                        .iter()
                        .filter_map(|p| match p {
                            ContentPart::Text { text } => Some(text.clone()),
                            _ => None,
                        })
                        .collect::<Vec<_>>()
                        .join("\n");

                    let tool_calls: Vec<ToolCall> = msg
                        .content
                        .iter()
                        .filter_map(|p| match p {
                            ContentPart::ToolCall { id, name, arguments } => Some(ToolCall {
                                id: id.clone(),
                                r#type: "function".to_string(),
                                function: ToolCallFunction {
                                    name: name.clone(),
                                    arguments: arguments.clone(),
                                },
                            }),
                            _ => None,
                        })
                        .collect();

                    result.push(ChatMessage {
                        role: "assistant".to_string(),
                        // StepFun requires content field to be present (even if empty) when tool_calls exist
                        content: if content.is_empty() && !tool_calls.is_empty() { 
                            Some(String::new()) 
                        } else if content.is_empty() { 
                            None 
                        } else { 
                            Some(content) 
                        },
                        tool_calls: if tool_calls.is_empty() {
                            None
                        } else {
                            Some(tool_calls)
                        },
                        tool_call_id: None,
                    });
                }
                Role::Tool => {
                    for part in &msg.content {
                        if let ContentPart::ToolResult {
                            tool_call_id,
                            content,
                        } = part
                        {
                            result.push(ChatMessage {
                                role: "tool".to_string(),
                                content: Some(content.clone()),
                                tool_calls: None,
                                tool_call_id: Some(tool_call_id.clone()),
                            });
                        }
                    }
                }
            }
        }

        result
    }

    fn convert_tools(&self, tools: &[ToolDefinition]) -> Vec<ChatTool> {
        tools
            .iter()
            .map(|t| ChatTool {
                r#type: "function".to_string(),
                function: ChatFunction {
                    name: t.name.clone(),
                    description: t.description.clone(),
                    parameters: t.parameters.clone(),
                },
            })
            .collect()
    }
}

#[async_trait]
impl Provider for StepFunProvider {
    fn name(&self) -> &str {
        "stepfun"
    }

    async fn list_models(&self) -> Result<Vec<ModelInfo>> {
        Ok(vec![
            ModelInfo {
                id: "step-3.5-flash".to_string(),
                name: "Step 3.5 Flash".to_string(),
                provider: "stepfun".to_string(),
                context_window: 128_000,
                max_output_tokens: Some(8192),
                supports_vision: false,
                supports_tools: true,
                supports_streaming: true,
                input_cost_per_million: Some(0.0), // Free tier
                output_cost_per_million: Some(0.0),
            },
            ModelInfo {
                id: "step-1-8k".to_string(),
                name: "Step 1 8K".to_string(),
                provider: "stepfun".to_string(),
                context_window: 8_000,
                max_output_tokens: Some(4096),
                supports_vision: false,
                supports_tools: true,
                supports_streaming: true,
                input_cost_per_million: Some(0.5),
                output_cost_per_million: Some(1.5),
            },
            ModelInfo {
                id: "step-1-32k".to_string(),
                name: "Step 1 32K".to_string(),
                provider: "stepfun".to_string(),
                context_window: 32_000,
                max_output_tokens: Some(8192),
                supports_vision: false,
                supports_tools: true,
                supports_streaming: true,
                input_cost_per_million: Some(1.0),
                output_cost_per_million: Some(3.0),
            },
            ModelInfo {
                id: "step-1-128k".to_string(),
                name: "Step 1 128K".to_string(),
                provider: "stepfun".to_string(),
                context_window: 128_000,
                max_output_tokens: Some(8192),
                supports_vision: false,
                supports_tools: true,
                supports_streaming: true,
                input_cost_per_million: Some(2.0),
                output_cost_per_million: Some(6.0),
            },
            ModelInfo {
                id: "step-1v-8k".to_string(),
                name: "Step 1 Vision 8K".to_string(),
                provider: "stepfun".to_string(),
                context_window: 8_000,
                max_output_tokens: Some(4096),
                supports_vision: true,
                supports_tools: true,
                supports_streaming: true,
                input_cost_per_million: Some(1.0),
                output_cost_per_million: Some(3.0),
            },
        ])
    }

    async fn complete(&self, request: CompletionRequest) -> Result<CompletionResponse> {
        let messages = self.convert_messages(&request.messages);
        let tools = self.convert_tools(&request.tools);

        let chat_request = ChatRequest {
            model: request.model.clone(),
            messages,
            tools: if tools.is_empty() { None } else { Some(tools) },
            temperature: request.temperature,
            max_tokens: request.max_tokens,
            stream: Some(false),
        };

        // Debug: log the request being sent
        if let Ok(json_str) = serde_json::to_string_pretty(&chat_request) {
            tracing::debug!("StepFun request: {}", json_str);
        }

        let response = self
            .client
            .post(format!("{}/chat/completions", STEPFUN_API_BASE))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&chat_request)
            .send()
            .await?;

        let status = response.status();
        let body = response.text().await?;

        if !status.is_success() {
            if let Ok(err) = serde_json::from_str::<ErrorResponse>(&body) {
                anyhow::bail!("StepFun API error: {}", err.error.message);
            }
            anyhow::bail!("StepFun API error ({}): {}", status, body);
        }

        let chat_response: ChatResponse = serde_json::from_str(&body)
            .map_err(|e| anyhow::anyhow!("Failed to parse response: {} - Body: {}", e, body))?;

        let choice = chat_response
            .choices
            .first()
            .ok_or_else(|| anyhow::anyhow!("No choices in response"))?;

        // Log usage for tracing
        tracing::info!(
            prompt_tokens = chat_response.usage.as_ref().map(|u| u.prompt_tokens).unwrap_or(0),
            completion_tokens = chat_response.usage.as_ref().map(|u| u.completion_tokens).unwrap_or(0),
            finish_reason = ?choice.finish_reason,
            "StepFun completion received"
        );

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
                prompt_tokens: chat_response.usage.as_ref().map(|u| u.prompt_tokens).unwrap_or(0),
                completion_tokens: chat_response.usage.as_ref().map(|u| u.completion_tokens).unwrap_or(0),
                total_tokens: chat_response.usage.as_ref().map(|u| u.total_tokens).unwrap_or(0),
                ..Default::default()
            },
            finish_reason,
        })
    }

    async fn complete_stream(
        &self,
        request: CompletionRequest,
    ) -> Result<futures::stream::BoxStream<'static, StreamChunk>> {
        let messages = self.convert_messages(&request.messages);
        let tools = self.convert_tools(&request.tools);

        let chat_request = ChatRequest {
            model: request.model.clone(),
            messages,
            tools: if tools.is_empty() { None } else { Some(tools) },
            temperature: request.temperature,
            max_tokens: request.max_tokens,
            stream: Some(true),
        };

        let response = self
            .client
            .post(format!("{}/chat/completions", STEPFUN_API_BASE))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&chat_request)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await?;
            anyhow::bail!("StepFun API error ({}): {}", status, body);
        }

        let stream = response
            .bytes_stream()
            .map(|result| match result {
                Ok(bytes) => {
                    let text = String::from_utf8_lossy(&bytes);
                    let mut chunks = Vec::new();

                    for line in text.lines() {
                        if let Some(data) = line.strip_prefix("data: ") {
                            if data.trim() == "[DONE]" {
                                chunks.push(StreamChunk::Done { usage: None });
                                continue;
                            }

                            if let Ok(chunk) = serde_json::from_str::<StreamChunkResponse>(data) {
                                if let Some(choice) = chunk.choices.first() {
                                    if let Some(content) = &choice.delta.content {
                                        chunks.push(StreamChunk::Text(content.clone()));
                                    }

                                    if let Some(tool_calls) = &choice.delta.tool_calls {
                                        for tc in tool_calls {
                                            if let Some(id) = &tc.id {
                                                if let Some(func) = &tc.function {
                                                    if let Some(name) = &func.name {
                                                        chunks.push(StreamChunk::ToolCallStart {
                                                            id: id.clone(),
                                                            name: name.clone(),
                                                        });
                                                    }
                                                }
                                            }
                                            if let Some(func) = &tc.function {
                                                if let Some(args) = &func.arguments {
                                                    if !args.is_empty() {
                                                        chunks.push(StreamChunk::ToolCallDelta {
                                                            id: tc.id.clone().unwrap_or_default(),
                                                            arguments_delta: args.clone(),
                                                        });
                                                    }
                                                }
                                            }
                                        }
                                    }

                                    if choice.finish_reason.is_some() {
                                        chunks.push(StreamChunk::Done { usage: None });
                                    }
                                }
                            }
                        }
                    }

                    if chunks.is_empty() {
                        StreamChunk::Text(String::new())
                    } else if chunks.len() == 1 {
                        chunks.pop().unwrap()
                    } else {
                        // Return first chunk, others are lost (simplified)
                        chunks.remove(0)
                    }
                }
                Err(e) => StreamChunk::Error(e.to_string()),
            })
            .boxed();

        Ok(stream)
    }
}
