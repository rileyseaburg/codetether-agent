//! Amazon Bedrock provider implementation using the Converse API
//!
//! Supports all Bedrock foundation models via API Key bearer token auth.
//! Uses the native Bedrock Converse API format.
//! Reference: https://docs.aws.amazon.com/bedrock/latest/APIReference/API_runtime_Converse.html

use super::{
    CompletionRequest, CompletionResponse, ContentPart, FinishReason, Message, ModelInfo, Provider,
    Role, StreamChunk, ToolDefinition, Usage,
};
use anyhow::{Context, Result};
use async_trait::async_trait;
use reqwest::Client;
use serde::Deserialize;
use serde_json::{Value, json};

const DEFAULT_REGION: &str = "us-east-1";

pub struct BedrockProvider {
    client: Client,
    api_key: String,
    region: String,
}

impl std::fmt::Debug for BedrockProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BedrockProvider")
            .field("api_key", &"<REDACTED>")
            .field("region", &self.region)
            .finish()
    }
}

impl BedrockProvider {
    pub fn new(api_key: String) -> Result<Self> {
        Self::with_region(api_key, DEFAULT_REGION.to_string())
    }

    pub fn with_region(api_key: String, region: String) -> Result<Self> {
        tracing::debug!(
            provider = "bedrock",
            region = %region,
            api_key_len = api_key.len(),
            "Creating Bedrock provider"
        );
        Ok(Self {
            client: Client::new(),
            api_key,
            region,
        })
    }

    fn validate_api_key(&self) -> Result<()> {
        if self.api_key.is_empty() {
            anyhow::bail!("Bedrock API key is empty");
        }
        Ok(())
    }

    fn base_url(&self) -> String {
        format!("https://bedrock-runtime.{}.amazonaws.com", self.region)
    }

    /// Resolve a short model alias to the full Bedrock model ID.
    /// Allows users to specify e.g. "claude-sonnet-4" instead of
    /// "anthropic.claude-sonnet-4-20250514-v1:0".
    fn resolve_model_id(model: &str) -> &str {
        match model {
            // Anthropic Claude models (cross-region inference profiles)
            "claude-sonnet-4" | "claude-4-sonnet" => "us.anthropic.claude-sonnet-4-20250514-v1:0",
            "claude-opus-4" | "claude-4-opus" => "us.anthropic.claude-opus-4-20250514-v1:0",
            "claude-3.5-haiku" | "claude-haiku-3.5" => {
                "us.anthropic.claude-3-5-haiku-20241022-v1:0"
            }
            "claude-3.5-sonnet" | "claude-sonnet-3.5" => {
                "us.anthropic.claude-3-5-sonnet-20241022-v2:0"
            }
            // Amazon Nova models (on-demand)
            "nova-pro" => "amazon.nova-pro-v1:0",
            "nova-lite" => "amazon.nova-lite-v1:0",
            "nova-micro" => "amazon.nova-micro-v1:0",
            "nova-premier" => "amazon.nova-premier-v1:0",
            // Meta Llama models (cross-region inference profiles)
            "llama-3.1-8b" => "us.meta.llama3-1-8b-instruct-v1:0",
            "llama-3.1-70b" => "us.meta.llama3-1-70b-instruct-v1:0",
            "llama-3.1-405b" => "us.meta.llama3-1-405b-instruct-v1:0",
            // Mistral models (cross-region inference profiles)
            "mistral-large" => "us.mistral.mistral-large-2407-v1:0",
            // Pass through full model IDs unchanged
            other => other,
        }
    }

    /// Convert our generic messages to Bedrock Converse API format.
    ///
    /// Bedrock Converse uses:
    /// - system prompt as a top-level "system" array
    /// - messages with "role" and "content" array
    /// - tool_use blocks in assistant content
    /// - toolResult blocks in user content
    fn convert_messages(messages: &[Message]) -> (Vec<Value>, Vec<Value>) {
        let mut system_parts: Vec<Value> = Vec::new();
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
                    system_parts.push(json!({"text": text}));
                }
                Role::User => {
                    let mut content_parts: Vec<Value> = Vec::new();
                    for part in &msg.content {
                        match part {
                            ContentPart::Text { text } => {
                                if !text.is_empty() {
                                    content_parts.push(json!({"text": text}));
                                }
                            }
                            _ => {}
                        }
                    }
                    if !content_parts.is_empty() {
                        api_messages.push(json!({
                            "role": "user",
                            "content": content_parts
                        }));
                    }
                }
                Role::Assistant => {
                    let mut content_parts: Vec<Value> = Vec::new();
                    for part in &msg.content {
                        match part {
                            ContentPart::Text { text } => {
                                if !text.is_empty() {
                                    content_parts.push(json!({"text": text}));
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
                                    "toolUse": {
                                        "toolUseId": id,
                                        "name": name,
                                        "input": input
                                    }
                                }));
                            }
                            _ => {}
                        }
                    }
                    if content_parts.is_empty() {
                        content_parts.push(json!({"text": ""}));
                    }
                    api_messages.push(json!({
                        "role": "assistant",
                        "content": content_parts
                    }));
                }
                Role::Tool => {
                    // Tool results go into a user message with toolResult blocks
                    let mut content_parts: Vec<Value> = Vec::new();
                    for part in &msg.content {
                        if let ContentPart::ToolResult {
                            tool_call_id,
                            content,
                        } = part
                        {
                            content_parts.push(json!({
                                "toolResult": {
                                    "toolUseId": tool_call_id,
                                    "content": [{"text": content}],
                                    "status": "success"
                                }
                            }));
                        }
                    }
                    if !content_parts.is_empty() {
                        api_messages.push(json!({
                            "role": "user",
                            "content": content_parts
                        }));
                    }
                }
            }
        }

        (system_parts, api_messages)
    }

    fn convert_tools(tools: &[ToolDefinition]) -> Vec<Value> {
        tools
            .iter()
            .map(|t| {
                json!({
                    "toolSpec": {
                        "name": t.name,
                        "description": t.description,
                        "inputSchema": {
                            "json": t.parameters
                        }
                    }
                })
            })
            .collect()
    }
}

/// Bedrock Converse API response types

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ConverseResponse {
    output: ConverseOutput,
    #[serde(default)]
    stop_reason: Option<String>,
    #[serde(default)]
    usage: Option<ConverseUsage>,
}

#[derive(Debug, Deserialize)]
struct ConverseOutput {
    message: ConverseMessage,
}

#[derive(Debug, Deserialize)]
struct ConverseMessage {
    #[allow(dead_code)]
    role: String,
    content: Vec<ConverseContent>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum ConverseContent {
    Text {
        text: String,
    },
    ToolUse {
        #[serde(rename = "toolUse")]
        tool_use: ConverseToolUse,
    },
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ConverseToolUse {
    tool_use_id: String,
    name: String,
    input: Value,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ConverseUsage {
    #[serde(default)]
    input_tokens: usize,
    #[serde(default)]
    output_tokens: usize,
    #[serde(default)]
    total_tokens: usize,
}

#[derive(Debug, Deserialize)]
struct BedrockError {
    message: String,
}

#[async_trait]
impl Provider for BedrockProvider {
    fn name(&self) -> &str {
        "bedrock"
    }

    async fn list_models(&self) -> Result<Vec<ModelInfo>> {
        self.validate_api_key()?;

        Ok(vec![
            // Anthropic Claude via Bedrock (best price/performance)
            ModelInfo {
                id: "us.anthropic.claude-sonnet-4-20250514-v1:0".to_string(),
                name: "Claude Sonnet 4 (Bedrock)".to_string(),
                provider: "bedrock".to_string(),
                context_window: 200_000,
                max_output_tokens: Some(64_000),
                supports_vision: true,
                supports_tools: true,
                supports_streaming: true,
                input_cost_per_million: Some(3.0),
                output_cost_per_million: Some(15.0),
            },
            ModelInfo {
                id: "us.anthropic.claude-opus-4-20250514-v1:0".to_string(),
                name: "Claude Opus 4 (Bedrock)".to_string(),
                provider: "bedrock".to_string(),
                context_window: 200_000,
                max_output_tokens: Some(32_000),
                supports_vision: true,
                supports_tools: true,
                supports_streaming: true,
                input_cost_per_million: Some(15.0),
                output_cost_per_million: Some(75.0),
            },
            ModelInfo {
                id: "us.anthropic.claude-3-5-haiku-20241022-v1:0".to_string(),
                name: "Claude 3.5 Haiku (Bedrock)".to_string(),
                provider: "bedrock".to_string(),
                context_window: 200_000,
                max_output_tokens: Some(8_192),
                supports_vision: true,
                supports_tools: true,
                supports_streaming: true,
                input_cost_per_million: Some(0.80),
                output_cost_per_million: Some(4.0),
            },
            ModelInfo {
                id: "us.anthropic.claude-3-5-sonnet-20241022-v2:0".to_string(),
                name: "Claude 3.5 Sonnet v2 (Bedrock)".to_string(),
                provider: "bedrock".to_string(),
                context_window: 200_000,
                max_output_tokens: Some(8_192),
                supports_vision: true,
                supports_tools: true,
                supports_streaming: true,
                input_cost_per_million: Some(3.0),
                output_cost_per_million: Some(15.0),
            },
            // Amazon Nova models
            ModelInfo {
                id: "amazon.nova-pro-v1:0".to_string(),
                name: "Amazon Nova Pro".to_string(),
                provider: "bedrock".to_string(),
                context_window: 300_000,
                max_output_tokens: Some(5_000),
                supports_vision: true,
                supports_tools: true,
                supports_streaming: true,
                input_cost_per_million: Some(0.80),
                output_cost_per_million: Some(3.20),
            },
            ModelInfo {
                id: "amazon.nova-lite-v1:0".to_string(),
                name: "Amazon Nova Lite".to_string(),
                provider: "bedrock".to_string(),
                context_window: 300_000,
                max_output_tokens: Some(5_000),
                supports_vision: true,
                supports_tools: true,
                supports_streaming: true,
                input_cost_per_million: Some(0.06),
                output_cost_per_million: Some(0.24),
            },
            ModelInfo {
                id: "amazon.nova-micro-v1:0".to_string(),
                name: "Amazon Nova Micro".to_string(),
                provider: "bedrock".to_string(),
                context_window: 128_000,
                max_output_tokens: Some(5_000),
                supports_vision: false,
                supports_tools: true,
                supports_streaming: true,
                input_cost_per_million: Some(0.035),
                output_cost_per_million: Some(0.14),
            },
            // Meta Llama models
            ModelInfo {
                id: "us.meta.llama3-1-70b-instruct-v1:0".to_string(),
                name: "Llama 3.1 70B (Bedrock)".to_string(),
                provider: "bedrock".to_string(),
                context_window: 128_000,
                max_output_tokens: Some(2_048),
                supports_vision: false,
                supports_tools: true,
                supports_streaming: true,
                input_cost_per_million: Some(0.72),
                output_cost_per_million: Some(0.72),
            },
            ModelInfo {
                id: "us.meta.llama3-1-8b-instruct-v1:0".to_string(),
                name: "Llama 3.1 8B (Bedrock)".to_string(),
                provider: "bedrock".to_string(),
                context_window: 128_000,
                max_output_tokens: Some(2_048),
                supports_vision: false,
                supports_tools: true,
                supports_streaming: true,
                input_cost_per_million: Some(0.22),
                output_cost_per_million: Some(0.22),
            },
            // Mistral
            ModelInfo {
                id: "us.mistral.mistral-large-2407-v1:0".to_string(),
                name: "Mistral Large (Bedrock)".to_string(),
                provider: "bedrock".to_string(),
                context_window: 128_000,
                max_output_tokens: Some(8_192),
                supports_vision: false,
                supports_tools: true,
                supports_streaming: true,
                input_cost_per_million: Some(2.0),
                output_cost_per_million: Some(6.0),
            },
        ])
    }

    async fn complete(&self, request: CompletionRequest) -> Result<CompletionResponse> {
        let model_id = Self::resolve_model_id(&request.model);

        tracing::debug!(
            provider = "bedrock",
            model = %model_id,
            original_model = %request.model,
            message_count = request.messages.len(),
            tool_count = request.tools.len(),
            "Starting Bedrock Converse request"
        );

        self.validate_api_key()?;

        let (system_parts, messages) = Self::convert_messages(&request.messages);
        let tools = Self::convert_tools(&request.tools);

        let mut body = json!({
            "messages": messages,
        });

        if !system_parts.is_empty() {
            body["system"] = json!(system_parts);
        }

        // inferenceConfig
        let mut inference_config = json!({});
        if let Some(max_tokens) = request.max_tokens {
            inference_config["maxTokens"] = json!(max_tokens);
        } else {
            inference_config["maxTokens"] = json!(8192);
        }
        if let Some(temp) = request.temperature {
            inference_config["temperature"] = json!(temp);
        }
        if let Some(top_p) = request.top_p {
            inference_config["topP"] = json!(top_p);
        }
        body["inferenceConfig"] = inference_config;

        if !tools.is_empty() {
            body["toolConfig"] = json!({"tools": tools});
        }

        // URL-encode the colon in model IDs (e.g. v1:0 -> v1%3A0)
        let encoded_model_id = model_id.replace(':', "%3A");
        let url = format!("{}/model/{}/converse", self.base_url(), encoded_model_id);
        tracing::debug!("Bedrock request URL: {}", url);

        let response = self
            .client
            .post(&url)
            .bearer_auth(&self.api_key)
            .header("content-type", "application/json")
            .header("accept", "application/json")
            .json(&body)
            .send()
            .await
            .context("Failed to send request to Bedrock")?;

        let status = response.status();
        let text = response
            .text()
            .await
            .context("Failed to read Bedrock response")?;

        if !status.is_success() {
            if let Ok(err) = serde_json::from_str::<BedrockError>(&text) {
                anyhow::bail!("Bedrock API error ({}): {}", status, err.message);
            }
            anyhow::bail!(
                "Bedrock API error: {} {}",
                status,
                &text[..text.len().min(500)]
            );
        }

        let response: ConverseResponse = serde_json::from_str(&text).context(format!(
            "Failed to parse Bedrock response: {}",
            &text[..text.len().min(300)]
        ))?;

        tracing::debug!(
            stop_reason = ?response.stop_reason,
            "Received Bedrock response"
        );

        let mut content = Vec::new();
        let mut has_tool_calls = false;

        for part in &response.output.message.content {
            match part {
                ConverseContent::Text { text } => {
                    if !text.is_empty() {
                        content.push(ContentPart::Text { text: text.clone() });
                    }
                }
                ConverseContent::ToolUse { tool_use } => {
                    has_tool_calls = true;
                    content.push(ContentPart::ToolCall {
                        id: tool_use.tool_use_id.clone(),
                        name: tool_use.name.clone(),
                        arguments: serde_json::to_string(&tool_use.input).unwrap_or_default(),
                    });
                }
            }
        }

        let finish_reason = if has_tool_calls {
            FinishReason::ToolCalls
        } else {
            match response.stop_reason.as_deref() {
                Some("end_turn") | Some("stop") | Some("stop_sequence") => FinishReason::Stop,
                Some("max_tokens") => FinishReason::Length,
                Some("tool_use") => FinishReason::ToolCalls,
                Some("content_filtered") => FinishReason::ContentFilter,
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
