//! OpenAI provider implementation

use super::{
    CompletionRequest, CompletionResponse, ContentPart, EmbeddingRequest, EmbeddingResponse,
    FinishReason, Message, ModelInfo, Provider, Role, StreamChunk, ToolDefinition, Usage,
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
use reqwest::Client as HttpClient;
use serde_json::Value;

pub struct OpenAIProvider {
    client: Client<OpenAIConfig>,
    provider_name: String,
    api_key: Option<String>,
    api_base: String,
    http: HttpClient,
}

impl std::fmt::Debug for OpenAIProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OpenAIProvider")
            .field("provider_name", &self.provider_name)
            .field("api_base", &self.api_base)
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
        let config = OpenAIConfig::new().with_api_key(api_key.clone());
        let api_base = "https://api.openai.com/v1".to_string();
        Ok(Self {
            client: Client::with_config(config),
            provider_name: "openai".to_string(),
            api_key: Some(api_key),
            api_base,
            http: HttpClient::builder()
                .timeout(std::time::Duration::from_secs(45))
                .build()?,
        })
    }

    /// Create with custom base URL (for OpenAI-compatible providers like Moonshot)
    pub fn with_base_url(api_key: String, base_url: String, provider_name: &str) -> Result<Self> {
        Self::with_base_url_optional_key(Some(api_key), base_url, provider_name)
    }

    /// Create with custom base URL and optional API key.
    ///
    /// Useful for private in-cluster OpenAI-compatible endpoints that rely on
    /// network policy instead of bearer authentication.
    pub fn with_base_url_optional_key(
        api_key: Option<String>,
        base_url: String,
        provider_name: &str,
    ) -> Result<Self> {
        let api_key = api_key.filter(|key| !key.trim().is_empty());
        tracing::debug!(
            provider = provider_name,
            base_url = %base_url,
            api_key_len = api_key.as_ref().map(|key| key.len()).unwrap_or(0),
            "Creating OpenAI-compatible provider"
        );
        let config = OpenAIConfig::new()
            .with_api_key(api_key.clone().unwrap_or_default())
            .with_api_base(base_url.clone());
        let api_base = base_url.trim_end_matches('/').to_string();
        Ok(Self {
            client: Client::with_config(config),
            provider_name: provider_name.to_string(),
            api_key,
            api_base,
            http: HttpClient::builder()
                .timeout(std::time::Duration::from_secs(45))
                .build()?,
        })
    }

    /// Return known models for specific OpenAI-compatible providers
    fn provider_default_models(&self) -> Vec<ModelInfo> {
        let models: Vec<(&str, &str)> = match self.provider_name.as_str() {
            "cerebras" => vec![
                ("llama3.1-8b", "Llama 3.1 8B"),
                ("llama-3.3-70b", "Llama 3.3 70B"),
                ("qwen-3.5-32b", "Qwen 3.5 32B"),
                ("gpt-oss-120b", "GPT-OSS 120B"),
            ],

            "minimax" => vec![
                ("MiniMax-M2.5", "MiniMax M2.5"),
                ("MiniMax-M2.5-highspeed", "MiniMax M2.5 Highspeed"),
                ("MiniMax-M2.1", "MiniMax M2.1"),
                ("MiniMax-M2.1-highspeed", "MiniMax M2.1 Highspeed"),
                ("MiniMax-M2", "MiniMax M2"),
            ],
            "zhipuai" => vec![],
            "novita" => vec![
                ("Qwen/Qwen3.5-35B-A3B", "Qwen 3.5 35B A3B"),
                ("deepseek/deepseek-v3-0324", "DeepSeek V3"),
                ("meta-llama/llama-3.1-70b-instruct", "Llama 3.1 70B"),
                ("meta-llama/llama-3.1-8b-instruct", "Llama 3.1 8B"),
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

    async fn discover_models_from_api(&self) -> Vec<ModelInfo> {
        let url = format!("{}/models", self.api_base);
        let mut request = self.http.get(&url);
        if let Some(api_key) = &self.api_key {
            request = request.bearer_auth(api_key);
        }

        let response = match request.send().await {
            Ok(response) => response,
            Err(error) => {
                tracing::debug!(
                    provider = %self.provider_name,
                    url = %url,
                    error = %error,
                    "Failed to fetch OpenAI-compatible /models endpoint"
                );
                return Vec::new();
            }
        };

        let status = response.status();
        if !status.is_success() {
            tracing::debug!(
                provider = %self.provider_name,
                url = %url,
                status = %status,
                "OpenAI-compatible /models endpoint returned non-success"
            );
            return Vec::new();
        }

        let payload: Value = match response.json().await {
            Ok(payload) => payload,
            Err(error) => {
                tracing::debug!(
                    provider = %self.provider_name,
                    url = %url,
                    error = %error,
                    "Failed to parse OpenAI-compatible /models response"
                );
                return Vec::new();
            }
        };

        let models = Self::parse_models_payload(&payload, &self.provider_name);
        if models.is_empty() {
            tracing::debug!(
                provider = %self.provider_name,
                url = %url,
                "OpenAI-compatible /models payload did not contain any model ids"
            );
        }
        models
    }

    fn parse_models_payload(payload: &Value, provider_name: &str) -> Vec<ModelInfo> {
        payload
            .get("data")
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
            .filter_map(|entry| Self::model_info_from_api_entry(entry, provider_name))
            .collect()
    }

    fn model_info_from_api_entry(entry: &Value, provider_name: &str) -> Option<ModelInfo> {
        let id = match entry {
            Value::String(id) => id.trim(),
            Value::Object(_) => entry.get("id").and_then(Value::as_str)?.trim(),
            _ => return None,
        };
        if id.is_empty() {
            return None;
        }

        let name = entry
            .get("name")
            .and_then(Value::as_str)
            .map(str::trim)
            .filter(|name| !name.is_empty())
            .unwrap_or(id);

        let supports_vision = entry
            .get("supports_vision")
            .and_then(Value::as_bool)
            .or_else(|| {
                entry
                    .get("input_modalities")
                    .and_then(Value::as_array)
                    .map(|modalities| {
                        modalities.iter().any(|modality| {
                            modality
                                .as_str()
                                .is_some_and(|modality| modality.eq_ignore_ascii_case("image"))
                        })
                    })
            })
            .unwrap_or(false);

        Some(ModelInfo {
            id: id.to_string(),
            name: name.to_string(),
            provider: provider_name.to_string(),
            context_window: value_to_usize(
                entry
                    .pointer("/limits/max_context_window_tokens")
                    .or_else(|| entry.get("context_window")),
            )
            .unwrap_or(128_000),
            max_output_tokens: value_to_usize(
                entry
                    .pointer("/limits/max_output_tokens")
                    .or_else(|| entry.get("max_output_tokens")),
            ),
            supports_vision,
            supports_tools: entry
                .get("supports_tools")
                .and_then(Value::as_bool)
                .unwrap_or(true),
            supports_streaming: entry
                .get("supports_streaming")
                .and_then(Value::as_bool)
                .unwrap_or(true),
            input_cost_per_million: entry
                .pointer("/pricing/input_cost_per_million")
                .and_then(Value::as_f64),
            output_cost_per_million: entry
                .pointer("/pricing/output_cost_per_million")
                .and_then(Value::as_f64),
        })
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
                                ..
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

    fn is_minimax_chat_setting_error(error: &str) -> bool {
        let normalized = error.to_ascii_lowercase();
        normalized.contains("invalid chat setting")
            || normalized.contains("(2013)")
            || normalized.contains("code: 2013")
            || normalized.contains("\"2013\"")
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
            let discovered = self.discover_models_from_api().await;
            if !discovered.is_empty() {
                return Ok(discovered);
            }
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
        req_builder.model(&request.model).messages(messages.clone());

        // Pass tools to the API if provided
        if !tools.is_empty() {
            req_builder.tools(tools);
        }
        if let Some(temp) = request.temperature {
            req_builder.temperature(temp);
        }
        if let Some(top_p) = request.top_p {
            req_builder.top_p(top_p);
        }
        if let Some(max) = request.max_tokens {
            if self.provider_name == "openai" {
                req_builder.max_completion_tokens(max as u32);
            } else {
                req_builder.max_tokens(max as u32);
            }
        }

        let primary_request = req_builder.build()?;
        let response = match self.client.chat().create(primary_request).await {
            Ok(response) => response,
            Err(err)
                if self.provider_name == "minimax"
                    && Self::is_minimax_chat_setting_error(&err.to_string()) =>
            {
                tracing::warn!(
                    provider = "minimax",
                    error = %err,
                    "MiniMax rejected chat settings; retrying with conservative defaults"
                );

                let mut fallback_builder = CreateChatCompletionRequestArgs::default();
                fallback_builder.model(&request.model).messages(messages);
                self.client.chat().create(fallback_builder.build()?).await?
            }
            Err(err) => return Err(err.into()),
        };

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
                        thought_signature: None,
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
        let tools = Self::convert_tools(&request.tools)?;

        let mut req_builder = CreateChatCompletionRequestArgs::default();
        req_builder
            .model(&request.model)
            .messages(messages)
            .stream(true);

        if !tools.is_empty() {
            req_builder.tools(tools);
        }
        if let Some(temp) = request.temperature {
            req_builder.temperature(temp);
        }
        if let Some(max) = request.max_tokens {
            if self.provider_name == "openai" {
                req_builder.max_completion_tokens(max as u32);
            } else {
                req_builder.max_tokens(max as u32);
            }
        }

        let stream = self
            .client
            .chat()
            .create_stream(req_builder.build()?)
            .await?;

        Ok(stream
            .flat_map(|result| {
                let chunks: Vec<StreamChunk> = match result {
                    Ok(response) => {
                        let mut out = Vec::new();
                        if let Some(choice) = response.choices.first() {
                            // Text content delta
                            if let Some(content) = &choice.delta.content {
                                if !content.is_empty() {
                                    out.push(StreamChunk::Text(content.clone()));
                                }
                            }
                            // Tool call deltas
                            if let Some(tool_calls) = &choice.delta.tool_calls {
                                for tc in tool_calls {
                                    if let Some(func) = &tc.function {
                                        // First chunk for a tool call has id and name
                                        if let Some(id) = &tc.id {
                                            out.push(StreamChunk::ToolCallStart {
                                                id: id.clone(),
                                                name: func.name.clone().unwrap_or_default(),
                                            });
                                        }
                                        // Argument deltas
                                        if let Some(args) = &func.arguments {
                                            if !args.is_empty() {
                                                // Derive the id from tc.id or use index as fallback
                                                let id = tc.id.clone().unwrap_or_else(|| {
                                                    format!("tool_{}", tc.index)
                                                });
                                                out.push(StreamChunk::ToolCallDelta {
                                                    id,
                                                    arguments_delta: args.clone(),
                                                });
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        out
                    }
                    Err(e) => vec![StreamChunk::Error(e.to_string())],
                };
                futures::stream::iter(chunks)
            })
            .boxed())
    }

    async fn embed(&self, request: EmbeddingRequest) -> Result<EmbeddingResponse> {
        if request.inputs.is_empty() {
            return Ok(EmbeddingResponse {
                embeddings: Vec::new(),
                usage: Usage::default(),
            });
        }

        let url = format!("{}/embeddings", self.api_base.trim_end_matches('/'));
        let body = OpenAIEmbeddingRequest {
            model: request.model,
            input: request.inputs,
        };

        let mut request_builder = self.http.post(url);
        if let Some(api_key) = self.api_key.as_deref().filter(|key| !key.is_empty()) {
            request_builder = request_builder.bearer_auth(api_key);
        }
        let response = request_builder.json(&body).send().await?;

        let status = response.status();
        let text = response.text().await?;
        if !status.is_success() {
            anyhow::bail!(
                "embedding request failed ({status}): {}",
                safe_char_prefix(&text, 500)
            );
        }

        let mut payload: OpenAIEmbeddingResponse = serde_json::from_str(&text)?;
        payload.data.sort_by_key(|item| item.index);
        let embeddings: Vec<Vec<f32>> = payload
            .data
            .into_iter()
            .map(|item| item.embedding)
            .collect();

        if embeddings.len() != body.input.len() {
            anyhow::bail!(
                "embedding response length mismatch: expected {}, got {}",
                body.input.len(),
                embeddings.len()
            );
        }

        let prompt_tokens = payload.usage.prompt_tokens.unwrap_or(0) as usize;
        let total_tokens = payload
            .usage
            .total_tokens
            .unwrap_or(payload.usage.prompt_tokens.unwrap_or(0))
            as usize;

        Ok(EmbeddingResponse {
            embeddings,
            usage: Usage {
                prompt_tokens,
                completion_tokens: 0,
                total_tokens,
                ..Default::default()
            },
        })
    }
}

fn value_to_usize(value: Option<&Value>) -> Option<usize> {
    value
        .and_then(Value::as_u64)
        .and_then(|value| usize::try_from(value).ok())
}

fn safe_char_prefix(input: &str, max_chars: usize) -> String {
    input.chars().take(max_chars).collect()
}

#[derive(Debug, serde::Serialize)]
struct OpenAIEmbeddingRequest {
    model: String,
    input: Vec<String>,
}

#[derive(Debug, serde::Deserialize)]
struct OpenAIEmbeddingResponse {
    data: Vec<OpenAIEmbeddingData>,
    #[serde(default)]
    usage: OpenAIEmbeddingUsage,
}

#[derive(Debug, serde::Deserialize)]
struct OpenAIEmbeddingData {
    index: usize,
    embedding: Vec<f32>,
}

#[derive(Debug, Default, serde::Deserialize)]
struct OpenAIEmbeddingUsage {
    #[serde(default)]
    prompt_tokens: Option<u32>,
    #[serde(default)]
    total_tokens: Option<u32>,
}

#[cfg(test)]
mod tests {
    use super::{OpenAIProvider, Provider};
    use serde_json::json;

    #[test]
    fn detects_minimax_chat_setting_error_variants() {
        assert!(OpenAIProvider::is_minimax_chat_setting_error(
            "bad_request_error: invalid params, invalid chat setting (2013)"
        ));
        assert!(OpenAIProvider::is_minimax_chat_setting_error(
            "code: 2013 invalid params"
        ));
        assert!(!OpenAIProvider::is_minimax_chat_setting_error(
            "rate limit exceeded"
        ));
    }

    #[test]
    fn supports_openai_compatible_provider_without_api_key() {
        let provider = OpenAIProvider::with_base_url_optional_key(
            None,
            "http://localhost:8080/v1".to_string(),
            "huggingface",
        )
        .expect("provider should initialize without API key");

        assert_eq!(provider.name(), "huggingface");
    }

    #[test]
    fn parses_openai_compatible_models_payload() {
        let payload = json!({
            "object": "list",
            "data": [
                {
                    "id": "GLM-5-Turbo",
                    "name": "GLM-5-Turbo",
                    "limits": {
                        "max_context_window_tokens": 200000,
                        "max_output_tokens": 16384
                    },
                    "input_modalities": ["text"]
                }
            ]
        });

        let models = OpenAIProvider::parse_models_payload(&payload, "custom-openapi");

        assert_eq!(models.len(), 1);
        assert_eq!(models[0].id, "GLM-5-Turbo");
        assert_eq!(models[0].name, "GLM-5-Turbo");
        assert_eq!(models[0].provider, "custom-openapi");
        assert_eq!(models[0].context_window, 200_000);
        assert_eq!(models[0].max_output_tokens, Some(16_384));
    }

    #[test]
    fn parses_string_only_models_payload() {
        let payload = json!({
            "data": ["glm-5", "glm-5-turbo"]
        });

        let models = OpenAIProvider::parse_models_payload(&payload, "custom-openapi");

        assert_eq!(models.len(), 2);
        assert_eq!(models[0].id, "glm-5");
        assert_eq!(models[1].id, "glm-5-turbo");
    }
}
