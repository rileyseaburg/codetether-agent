//! Local CUDA Provider using Candle bindings
//!
//! This provider uses pure-Rust ML execution directly on NVIDIA hardware
//! without needing C++ interop or external HTTP servers like Ollama.

use crate::cognition::{CandleDevicePreference, ThinkerBackend, ThinkerClient, ThinkerConfig};
use crate::provider::{
    CompletionRequest, CompletionResponse, ContentPart, FinishReason, Message, ModelInfo, Provider,
    Role, StreamChunk, Usage,
};
use anyhow::{Context, Result, anyhow};
use async_trait::async_trait;
use candle_core::Device;
use futures::stream::BoxStream;
use std::path::Path;
use tokio::sync::OnceCell;

/// Local CUDA Provider using Candle bindings
///
/// This provider uses pure-Rust ML execution directly on NVIDIA hardware
/// without needing C++ interop.
pub struct LocalCudaProvider {
    model_name: String,
    device: Device,
    /// Model cache - in production this would hold the loaded model
    model_cache: Option<ModelCache>,
    runtime: OnceCell<ThinkerClient>,
}

struct ModelCache {
    model_path: String,
    tokenizer_path: Option<String>,
    architecture: Option<String>,
}

impl LocalCudaProvider {
    /// Create a new LocalCudaProvider
    pub fn new(model_name: String) -> Result<Self> {
        // Try to create a CUDA device, fall back to CPU if unavailable
        let device = match Device::new_cuda(0) {
            Ok(d) => {
                tracing::info!("Using CUDA device for local inference");
                d
            }
            Err(_) => {
                tracing::warn!("CUDA not available, using CPU (will be slow)");
                Device::Cpu
            }
        };

        Ok(Self {
            model_name,
            device,
            model_cache: None,
            runtime: OnceCell::const_new(),
        })
    }

    /// Create with explicit model path
    pub fn with_model(model_name: String, model_path: String) -> Result<Self> {
        Self::with_paths(model_name, model_path, None, None)
    }

    /// Create with explicit model and tokenizer paths.
    pub fn with_paths(
        model_name: String,
        model_path: String,
        tokenizer_path: Option<String>,
        architecture: Option<String>,
    ) -> Result<Self> {
        let mut provider = Self::new(model_name)?;
        provider.model_cache = Some(ModelCache {
            model_path,
            tokenizer_path,
            architecture,
        });
        Ok(provider)
    }

    /// Check if CUDA is available
    pub fn is_cuda_available() -> bool {
        Device::new_cuda(0).is_ok()
    }

    /// Get device info
    pub fn device_info() -> String {
        match Device::new_cuda(0) {
            Ok(d) => format!("CUDA: {:?}", d),
            Err(_) => "CPU only".to_string(),
        }
    }

    async fn runtime(&self, request: &CompletionRequest) -> Result<&ThinkerClient> {
        // Runtime config is captured on first use; changing env vars later
        // requires creating a new provider instance.
        self.runtime
            .get_or_try_init(|| async { ThinkerClient::new(self.build_config(request)?) })
            .await
    }

    fn build_config(&self, request: &CompletionRequest) -> Result<ThinkerConfig> {
        let model_path = self
            .resolve_model_path()
            .ok_or_else(|| {
                anyhow!(
                    "Local CUDA requires model path via LOCAL_CUDA_MODEL_PATH or \
                     CODETETHER_LOCAL_CUDA_MODEL_PATH"
                )
            })?;
        let tokenizer_path = self.resolve_tokenizer_path().or_else(|| {
            // Common local layout: tokenizer.json next to model.gguf
            Path::new(&model_path)
                .parent()
                .map(|p| p.join("tokenizer.json"))
                .filter(|p| p.exists())
                .map(|p| p.to_string_lossy().to_string())
        });
        let tokenizer_path = tokenizer_path.ok_or_else(|| {
            anyhow!(
                "Local CUDA requires tokenizer path via LOCAL_CUDA_TOKENIZER_PATH or \
                 CODETETHER_LOCAL_CUDA_TOKENIZER_PATH"
            )
        })?;

        if !Path::new(&model_path).exists() {
            return Err(anyhow!(
                "Local CUDA model path does not exist: {}",
                model_path
            ));
        }
        if !Path::new(&tokenizer_path).exists() {
            return Err(anyhow!(
                "Local CUDA tokenizer path does not exist: {}",
                tokenizer_path
            ));
        }

        let mut config = ThinkerConfig {
            enabled: true,
            backend: ThinkerBackend::Candle,
            model: self.model_name.clone(),
            candle_model_path: Some(model_path),
            candle_tokenizer_path: Some(tokenizer_path),
            candle_arch: self.resolve_architecture(),
            candle_device: self.resolve_device_preference(),
            candle_cuda_ordinal: parse_env_usize(
                &["LOCAL_CUDA_ORDINAL", "CODETETHER_LOCAL_CUDA_ORDINAL"],
                0,
            ),
            candle_repeat_penalty: parse_env_f32(
                &[
                    "LOCAL_CUDA_REPEAT_PENALTY",
                    "CODETETHER_LOCAL_CUDA_REPEAT_PENALTY",
                ],
                1.1,
            ),
            candle_repeat_last_n: parse_env_usize(
                &[
                    "LOCAL_CUDA_REPEAT_LAST_N",
                    "CODETETHER_LOCAL_CUDA_REPEAT_LAST_N",
                ],
                64,
            ),
            candle_seed: parse_env_u64(&["LOCAL_CUDA_SEED", "CODETETHER_LOCAL_CUDA_SEED"], 42),
            temperature: request.temperature.unwrap_or_else(|| {
                parse_env_f32(
                    &["LOCAL_CUDA_TEMPERATURE", "CODETETHER_LOCAL_CUDA_TEMPERATURE"],
                    0.2,
                )
            }),
            top_p: request.top_p,
            max_tokens: request.max_tokens.unwrap_or(512).max(1),
            ..ThinkerConfig::default()
        };
        config.timeout_ms = parse_env_u64(
            &["LOCAL_CUDA_TIMEOUT_MS", "CODETETHER_LOCAL_CUDA_TIMEOUT_MS"],
            120_000,
        );

        tracing::info!(
            model = %config.model,
            device = ?config.candle_device,
            cuda_ordinal = config.candle_cuda_ordinal,
            max_tokens = config.max_tokens,
            "Initialized Local CUDA runtime configuration"
        );

        Ok(config)
    }

    fn resolve_model_path(&self) -> Option<String> {
        self.model_cache
            .as_ref()
            .map(|c| c.model_path.clone())
            .or_else(|| {
                first_env(&[
                    "LOCAL_CUDA_MODEL_PATH",
                    "CODETETHER_LOCAL_CUDA_MODEL_PATH",
                ])
            })
    }

    fn resolve_tokenizer_path(&self) -> Option<String> {
        self.model_cache
            .as_ref()
            .and_then(|c| c.tokenizer_path.clone())
            .or_else(|| {
                first_env(&[
                    "LOCAL_CUDA_TOKENIZER_PATH",
                    "CODETETHER_LOCAL_CUDA_TOKENIZER_PATH",
                ])
            })
    }

    fn resolve_architecture(&self) -> Option<String> {
        self.model_cache
            .as_ref()
            .and_then(|c| c.architecture.clone())
            .or_else(|| first_env(&["LOCAL_CUDA_ARCH", "CODETETHER_LOCAL_CUDA_ARCH"]))
    }

    fn resolve_device_preference(&self) -> CandleDevicePreference {
        if let Some(v) = first_env(&["LOCAL_CUDA_DEVICE", "CODETETHER_LOCAL_CUDA_DEVICE"]) {
            return CandleDevicePreference::from_env(&v);
        }
        CandleDevicePreference::Auto
    }

    fn map_finish_reason(reason: Option<&str>) -> FinishReason {
        match reason {
            Some("stop") => FinishReason::Stop,
            Some("length") => FinishReason::Length,
            Some("tool_calls") => FinishReason::ToolCalls,
            Some("content_filter") => FinishReason::ContentFilter,
            Some("error") => FinishReason::Error,
            _ => FinishReason::Stop,
        }
    }

    fn to_prompts(messages: &[Message]) -> (String, String) {
        let mut system_lines = Vec::new();
        let mut convo_lines = Vec::new();

        for msg in messages {
            match msg.role {
                Role::System => {
                    let text = Self::content_to_string(&msg.content);
                    if !text.is_empty() {
                        system_lines.push(text);
                    }
                }
                Role::User => {
                    convo_lines.push(format!("User:\n{}", Self::content_to_string(&msg.content)));
                }
                Role::Assistant => {
                    convo_lines.push(format!(
                        "Assistant:\n{}",
                        Self::content_to_string(&msg.content)
                    ));
                }
                Role::Tool => {
                    convo_lines.push(format!("Tool:\n{}", Self::content_to_string(&msg.content)));
                }
            }
        }

        let system_prompt = if system_lines.is_empty() {
            "You are CodeTether local CUDA coding assistant.".to_string()
        } else {
            system_lines.join("\n\n")
        };
        let user_prompt = if convo_lines.is_empty() {
            "User:\n(Empty prompt)".to_string()
        } else {
            convo_lines.join("\n\n")
        };

        (system_prompt, user_prompt)
    }
}

#[async_trait]
impl Provider for LocalCudaProvider {
    fn name(&self) -> &str {
        "local_cuda"
    }

    async fn list_models(&self) -> Result<Vec<ModelInfo>> {
        // Note: streaming and tool support are planned features, currently unimplemented
        Ok(vec![ModelInfo {
            id: self.model_name.clone(),
            name: self.model_name.clone(),
            provider: "local_cuda".to_string(),
            context_window: 8192,
            max_output_tokens: Some(4096),
            supports_vision: false,
            supports_tools: false,             // TODO: implement tool calling
            supports_streaming: false,         // TODO: implement streaming inference
            input_cost_per_million: Some(0.0), // Free - local inference
            output_cost_per_million: Some(0.0),
        }])
    }

    async fn complete(&self, request: CompletionRequest) -> Result<CompletionResponse> {
        let (system_prompt, user_prompt) = Self::to_prompts(&request.messages);
        let runtime = self.runtime(&request).await?;
        let output = runtime
            .think(&system_prompt, &user_prompt)
            .await
            .with_context(|| format!("local_cuda inference failed for model {}", self.model_name))?;

        tracing::debug!(
            model = %self.model_name,
            prompt_tokens = output.prompt_tokens.unwrap_or(0),
            completion_tokens = output.completion_tokens.unwrap_or(0),
            finish_reason = ?output.finish_reason,
            "Local CUDA inference completed"
        );

        let text = output.text.trim().to_string();
        Ok(CompletionResponse {
            message: Message {
                role: Role::Assistant,
                content: vec![ContentPart::Text { text }],
            },
            usage: Usage {
                prompt_tokens: output.prompt_tokens.unwrap_or(0) as usize,
                completion_tokens: output.completion_tokens.unwrap_or(0) as usize,
                total_tokens: output.total_tokens.unwrap_or(0) as usize,
                cache_read_tokens: None,
                cache_write_tokens: None,
            },
            finish_reason: Self::map_finish_reason(output.finish_reason.as_deref()),
        })
    }

    async fn complete_stream(
        &self,
        _request: CompletionRequest,
    ) -> Result<BoxStream<'static, StreamChunk>> {
        Err(anyhow!(
            "Streaming inference not yet implemented for local_cuda provider"
        ))
    }
}

fn first_env(keys: &[&str]) -> Option<String> {
    keys.iter().find_map(|k| std::env::var(k).ok())
}

fn parse_env_f32(keys: &[&str], default: f32) -> f32 {
    first_env(keys)
        .and_then(|v| v.parse::<f32>().ok())
        .unwrap_or(default)
}

fn parse_env_usize(keys: &[&str], default: usize) -> usize {
    first_env(keys)
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(default)
}

fn parse_env_u64(keys: &[&str], default: u64) -> u64 {
    first_env(keys)
        .and_then(|v| v.parse::<u64>().ok())
        .unwrap_or(default)
}

impl LocalCudaProvider {
    /// Format messages into a prompt string
    fn format_messages(messages: &[Message]) -> String {
        let mut prompt = String::new();

        for msg in messages {
            match msg.role {
                Role::System => {
                    prompt.push_str("System: ");
                    prompt.push_str(&Self::content_to_string(&msg.content));
                    prompt.push_str("\n\n");
                }
                Role::User => {
                    prompt.push_str("User: ");
                    prompt.push_str(&Self::content_to_string(&msg.content));
                    prompt.push_str("\n\n");
                }
                Role::Assistant => {
                    prompt.push_str("Assistant: ");
                    prompt.push_str(&Self::content_to_string(&msg.content));
                    prompt.push_str("\n\n");
                }
                Role::Tool => {
                    // Tool results go in the conversation as context
                    prompt.push_str("Tool: ");
                    prompt.push_str(&Self::content_to_string(&msg.content));
                    prompt.push_str("\n\n");
                }
            }
        }

        prompt.push_str("Assistant: ");
        prompt
    }

    fn content_to_string(parts: &[ContentPart]) -> String {
        parts
            .iter()
            .map(|part| match part {
                ContentPart::Text { text } => text.clone(),
                ContentPart::ToolResult { content, .. } => content.clone(),
                ContentPart::Thinking { text } => text.clone(),
                _ => String::new(),
            })
            .collect::<Vec<_>>()
            .join("\n")
    }
}

/// Configuration for LocalCudaProvider
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LocalCudaConfig {
    /// Model name (e.g., "qwen2.5-coder-7b", "deepseek-coder-6.7b")
    pub model_name: String,
    /// Path to the model weights (GGUF or safetensors format)
    pub model_path: Option<String>,
    /// Context window size
    pub context_window: Option<usize>,
    /// Maximum tokens to generate
    pub max_new_tokens: Option<usize>,
    /// Temperature for sampling
    pub temperature: Option<f32>,
    /// Top-p for nucleus sampling
    pub top_p: Option<f32>,
    /// Repetition penalty
    pub repeat_penalty: Option<f32>,
    /// CUDA device ordinal (0 for first GPU)
    pub cuda_device: Option<usize>,
}

impl Default for LocalCudaConfig {
    fn default() -> Self {
        Self {
            model_name: "qwen2.5-coder-7b".to_string(),
            model_path: None,
            context_window: Some(8192),
            max_new_tokens: Some(4096),
            temperature: Some(0.7),
            top_p: Some(0.9),
            repeat_penalty: Some(1.1),
            cuda_device: Some(0),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_local_cuda_provider_creation() {
        let provider = LocalCudaProvider::new("test-model".to_string());
        assert!(provider.is_ok());
        assert_eq!(provider.unwrap().name(), "local_cuda");
    }

    #[test]
    fn test_cuda_availability_check() {
        // This will return whatever the system has
        let _ = LocalCudaProvider::is_cuda_available();
    }

    #[test]
    fn test_format_messages() {
        let messages = vec![
            Message {
                role: Role::System,
                content: vec![ContentPart::Text {
                    text: "You are a helpful assistant.".to_string(),
                }],
            },
            Message {
                role: Role::User,
                content: vec![ContentPart::Text {
                    text: "Hello!".to_string(),
                }],
            },
        ];

        let formatted = LocalCudaProvider::format_messages(&messages);
        assert!(formatted.contains("You are a helpful assistant."));
        assert!(formatted.contains("Hello!"));
    }

    #[tokio::test]
    async fn test_complete_error_message_no_prompt_exposure() {
        let provider = LocalCudaProvider::new("test-model".to_string()).unwrap();

        let request = CompletionRequest {
            messages: vec![Message {
                role: Role::User,
                content: vec![ContentPart::Text {
                    text: "This is a sensitive prompt that should not appear in error messages!"
                        .to_string(),
                }],
            }],
            model: "test-model".to_string(),
            tools: vec![],
            temperature: None,
            top_p: None,
            max_tokens: Some(100),
            stop: vec![],
        };

        let result = provider.complete(request).await;
        let error_message = match result {
            Ok(_) => panic!("Expected local_cuda complete() to fail until inference is implemented"),
            Err(e) => e.to_string(),
        };

        // Verify error message does not contain user prompt content
        assert!(!error_message.contains("sensitive prompt"));
        assert!(!error_message.contains("should not appear"));

        // With no local model configured, provider should return config error.
        assert!(error_message.contains("Local CUDA requires model path"));
    }

    #[tokio::test]
    async fn test_complete_stream_error_message_no_prompt_exposure() {
        let provider = LocalCudaProvider::new("test-model".to_string()).unwrap();

        let request = CompletionRequest {
            messages: vec![Message {
                role: Role::User,
                content: vec![ContentPart::Text {
                    text: "Another sensitive prompt for streaming test".to_string(),
                }],
            }],
            model: "test-model".to_string(),
            tools: vec![],
            temperature: None,
            top_p: None,
            max_tokens: Some(100),
            stop: vec![],
        };

        let result = provider.complete_stream(request).await;
        let error_message = match result {
            Ok(_) => {
                panic!("Expected local_cuda complete_stream() to fail until streaming is implemented")
            }
            Err(e) => e.to_string(),
        };

        // Verify error message does not contain user prompt content
        assert!(!error_message.contains("sensitive prompt"));
        assert!(!error_message.contains("streaming test"));

        // Streaming remains unimplemented.
        assert!(
            error_message.contains("not yet implemented")
                || error_message.contains("not implemented")
        );
    }
}
