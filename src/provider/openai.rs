//! OpenAI provider implementation

use super::{
    CompletionRequest, CompletionResponse, ContentPart, FinishReason, Message, ModelInfo, Provider,
    Role, StreamChunk, ToolDefinition, Usage,
};
use anyhow::Result;
use async_openai::{
    Client,
    config::OpenAIConfig,
    types::chat::{
        ChatCompletionMessageToolCall, ChatCompletionMessageToolCalls,
        ChatCompletionRequestAssistantMessageArgs, ChatCompletionRequestMessage,
        ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestToolMessageArgs,
        ChatCompletionRequestUserMessageArgs, ChatCompletionTool, ChatCompletionTools,
        CreateChatCompletionRequestArgs, FinishReason as OpenAIFinishReason, FunctionCall,
        FunctionObjectArgs,
    },
};
use async_trait::async_trait;
use futures::StreamExt;

pub struct OpenAIProvider {
    client: Client<OpenAIConfig>,
    provider_name: String,
}

impl std::fmt::Debug for OpenAIProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OpenAIProvider")
            .field("provider_name", &self.provider_name)
            .field("client", &"<async_openai::Client>")
            .finish()
    }
}

impl OpenAIProvider {
    pub fn new(api_key: String) -> Result<Self> {
        tracing::debug!(
            provider = "openai",
            api_key_len = api_key.len(),
            "Creating OpenAI provider"
        );
        let config = OpenAIConfig::new().with_api_key(api_key);
        Ok(Self {
            client: Client::with_config(config),
            provider_name: "openai".to_string(),
        })
    }

    /// Create with custom base URL (for OpenAI-compatible providers like Moonshot)
    pub fn with_base_url(api_key: String, base_url: String, provider_name: &str) -> Result<Self> {
        tracing::debug!(
            provider = provider_name,
            base_url = %base_url,
            api_key_len = api_key.len(),
            "Creating OpenAI-compatible provider"
        );
        let config = OpenAIConfig::new()
            .with_api_key(api_key)
            .with_api_base(base_url);
        Ok(Self {
            client: Client::with_config(config),
            provider_name: provider_name.to_string(),
        })
    }

    /// Return known models for specific OpenAI-compatible providers
    fn provider_default_models(&self) -> Vec<ModelInfo> {
        let models: Vec<(&str, &str)> = match self.provider_name.as_str() {
            "cerebras" => vec![
                ("llama3.1-8b", "Llama 3.1 8B"),
                ("llama-3.3-70b", "Llama 3.3 70B"),
                ("qwen-3-32b", "Qwen 3 32B"),
                ("gpt-oss-120b", "GPT-OSS 120B"),
            ],
            "novita" => vec![
                ("meta-llama/llama-3.1-8b-instruct", "Llama 3.1 8B"),
                ("meta-llama/llama-3.1-70b-instruct", "Llama 3.1 70B"),
                ("deepseek/deepseek-v3-0324", "DeepSeek V3"),
                ("qwen/qwen-2.5-72b-instruct", "Qwen 2.5 72B"),
            ],
            "minimax" => vec![
                ("MiniMax-M1-80k", "MiniMax M1 80k"),
                ("MiniMax-Text-01", "MiniMax Text 01"),
            ],
            _ => vec![],
        };

        models
            .into_iter()
            .map(|(id, name)| ModelInfo {
                id: id.to_string(),
                name: name.to_string(),
                provider: self.provider_name.clone(),
                context_window: 128_000,
                max_output_tokens: Some(16_384),
                supports_vision: false,
                supports_tools: true,
                supports_streaming: true,
                input_cost_per_million: None,
                output_cost_per_million: None,
            })
            .collect()
    }

    fn convert_messages(messages: &[Message]) -> Result<Vec<ChatCompletionRequestMessage>> {
        let mut result = Vec::new();

        for msg in messages {
            let content = msg
                .content
                .iter()
                .filter_map(|p| match p {
                    ContentPart::Text { text } => Some(text.clone()),
                    _ => None,
                })
                .collect::<Vec<_>>()
                .join("\n");

            match msg.role {
                Role::System => {
                    result.push(
                        ChatCompletionRequestSystemMessageArgs::default()
                            .content(content)
                            .build()?
                            .into(),
                    );
                }
                Role::User => {
                    result.push(
                        ChatCompletionRequestUserMessageArgs::default()
                            .content(content)
                            .build()?
                            .into(),
                    );
                }
                Role::Assistant => {
                    let tool_calls: Vec<ChatCompletionMessageToolCalls> = msg
                        .content
                        .iter()
                        .filter_map(|p| match p {
                            ContentPart::ToolCall {
                                id,
                                name,
                                arguments,
                            } => Some(ChatCompletionMessageToolCalls::Function(
                                ChatCompletionMessageToolCall {
                                    id: id.clone(),
                                    function: FunctionCall {
                                        name: name.clone(),
                                        arguments: arguments.clone(),
                                    },
                                },
                            )),
                            _ => None,
                        })
                        .collect();

                    let mut builder = ChatCompletionRequestAssistantMessageArgs::default();
                    if !content.is_empty() {
                        builder.content(content);
                    }
                    if !tool_calls.is_empty() {
                        builder.tool_calls(tool_calls);
                    }
                    result.push(builder.build()?.into());
                }
                Role::Tool => {
                    for part in &msg.content {
                        if let ContentPart::ToolResult {
                            tool_call_id,
                            content,
                        } = part
                        {
                            result.push(
                                ChatCompletionRequestToolMessageArgs::default()
                                    .tool_call_id(tool_call_id.clone())
                                    .content(content.clone())
                                    .build()?
                                    .into(),
                            );
                        }
                    }
                }
            }
        }

        Ok(result)
    }

    fn convert_tools(tools: &[ToolDefinition]) -> Result<Vec<ChatCompletionTools>> {
        let mut result = Vec::new();
        for tool in tools {
            result.push(ChatCompletionTools::Function(ChatCompletionTool {
                function: FunctionObjectArgs::default()
                    .name(&tool.name)
                    .description(&tool.description)
                    .parameters(tool.parameters.clone())
                    .build()?,
            }));
        }
        Ok(result)
    }
}

#[async_trait]
impl Provider for OpenAIProvider {
    fn name(&self) -> &str {
        &self.provider_name
    }

    async fn list_models(&self) -> Result<Vec<ModelInfo>> {
        // For non-OpenAI providers, return provider-specific model defaults.
        // Note: async-openai 0.32 does not expose a stable models list API across
        // all OpenAI-compatible endpoints.
        if self.provider_name != "openai" {
            return Ok(self.provider_default_models());
        }

        // OpenAI default models
        Ok(vec![
            ModelInfo {
                id: "gpt-4o".to_string(),
                name: "GPT-4o".to_string(),
                provider: "openai".to_string(),
                context_window: 128_000,
                max_output_tokens: Some(16_384),
                supports_vision: true,
                supports_tools: true,
                supports_streaming: true,
                input_cost_per_million: Some(2.5),
                output_cost_per_million: Some(10.0),
            },
            ModelInfo {
                id: "gpt-4o-mini".to_string(),
                name: "GPT-4o Mini".to_string(),
                provider: "openai".to_string(),
                context_window: 128_000,
                max_output_tokens: Some(16_384),
                supports_vision: true,
                supports_tools: true,
                supports_streaming: true,
                input_cost_per_million: Some(0.15),
                output_cost_per_million: Some(0.6),
            },
            ModelInfo {
                id: "o1".to_string(),
                name: "o1".to_string(),
                provider: "openai".to_string(),
                context_window: 200_000,
                max_output_tokens: Some(100_000),
                supports_vision: true,
                supports_tools: true,
                supports_streaming: true,
                input_cost_per_million: Some(15.0),
                output_cost_per_million: Some(60.0),
            },
        ])
    }

    async fn complete(&self, request: CompletionRequest) -> Result<CompletionResponse> {
        let messages = Self::convert_messages(&request.messages)?;
        let tools = Self::convert_tools(&request.tools)?;

        let mut req_builder = CreateChatCompletionRequestArgs::default();
        req_builder.model(&request.model).messages(messages);

        // Pass tools to the API if provided
        if !tools.is_empty() {
            req_builder.tools(tools);
        }
        if let Some(temp) = request.temperature {
            req_builder.temperature(temp);
        }
        if let Some(max) = request.max_tokens {
            req_builder.max_completion_tokens(max as u32);
        }

        let response = self.client.chat().create(req_builder.build()?).await?;

        let choice = response
            .choices
            .first()
            .ok_or_else(|| anyhow::anyhow!("No choices"))?;

        let mut content = Vec::new();
        let mut has_tool_calls = false;

        if let Some(text) = &choice.message.content {
            content.push(ContentPart::Text { text: text.clone() });
        }
        if let Some(tool_calls) = &choice.message.tool_calls {
            has_tool_calls = !tool_calls.is_empty();
            for tc in tool_calls {
                if let ChatCompletionMessageToolCalls::Function(func_call) = tc {
                    content.push(ContentPart::ToolCall {
                        id: func_call.id.clone(),
                        name: func_call.function.name.clone(),
                        arguments: func_call.function.arguments.clone(),
                    });
                }
            }
        }

        // Determine finish reason based on response
        let finish_reason = if has_tool_calls {
            FinishReason::ToolCalls
        } else {
            match choice.finish_reason {
                Some(OpenAIFinishReason::Stop) => FinishReason::Stop,
                Some(OpenAIFinishReason::Length) => FinishReason::Length,
                Some(OpenAIFinishReason::ToolCalls) => FinishReason::ToolCalls,
                Some(OpenAIFinishReason::ContentFilter) => FinishReason::ContentFilter,
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
                    .map(|u| u.prompt_tokens as usize)
                    .unwrap_or(0),
                completion_tokens: response
                    .usage
                    .as_ref()
                    .map(|u| u.completion_tokens as usize)
                    .unwrap_or(0),
                total_tokens: response
                    .usage
                    .as_ref()
                    .map(|u| u.total_tokens as usize)
                    .unwrap_or(0),
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
            provider = %self.provider_name,
            model = %request.model,
            message_count = request.messages.len(),
            "Starting streaming completion request"
        );

        let messages = Self::convert_messages(&request.messages)?;

        let mut req_builder = CreateChatCompletionRequestArgs::default();
        req_builder
            .model(&request.model)
            .messages(messages)
            .stream(true);

        if let Some(temp) = request.temperature {
            req_builder.temperature(temp);
        }

        let stream = self
            .client
            .chat()
            .create_stream(req_builder.build()?)
            .await?;

        Ok(stream
            .map(|result| match result {
                Ok(response) => {
                    if let Some(choice) = response.choices.first() {
                        if let Some(content) = &choice.delta.content {
                            return StreamChunk::Text(content.clone());
                        }
                    }
                    StreamChunk::Text(String::new())
                }
                Err(e) => StreamChunk::Error(e.to_string()),
            })
            .boxed())
    }
}
