//! Local CUDA Provider using Candle bindings
//!
//! This provider uses pure-Rust ML execution directly on NVIDIA hardware
//! without needing C++ interop or external HTTP servers like Ollama.

use crate::provider::{
    CompletionRequest, CompletionResponse, ContentPart, FinishReason, Message, ModelInfo, Provider,
    Role, StreamChunk, Usage,
};
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use candle_core::{Device, Tensor};
use candle_nn::VarBuilder;
use candle_transformers::generation::LogitsProcessor;
use futures::stream::BoxStream;
use std::sync::Arc;
use tokio::sync::Mutex;

/// Local CUDA Provider using Candle bindings
///
/// This provider uses pure-Rust ML execution directly on NVIDIA hardware
/// without needing C++ interop.
pub struct LocalCudaProvider {
    model_name: String,
    device: Device,
    /// Model cache - in production this would hold the loaded model
    model_cache: Arc<Mutex<Option<ModelCache>>>,
}

struct ModelCache {
    // In a full implementation, this would hold:
    // - The loaded model weights
    // - The tokenizer
    // - Generation config
    model_path: String,
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
            model_cache: Arc::new(Mutex::new(None)),
        })
    }

    /// Create with explicit model path
    pub fn with_model(model_name: String, model_path: String) -> Result<Self> {
        let mut provider = Self::new(model_name)?;
        // Pre-load the model cache
        provider.model_cache = Arc::new(Mutex::new(Some(ModelCache { model_path })));
        Ok(provider)
    }

    /// Check if CUDA is available
    pub fn is_cuda_available() -> bool {
        Device::new_cuda(0).is_ok()
    }

    /// Get device info
    pub fn device_info() -> String {
        match Device::new_cuda(0) {
            Ok(d) => format!("CUDA: {}", d),
            Err(_) => "CPU only".to_string(),
        }
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
            supports_tools: false, // TODO: implement tool calling
            supports_streaming: false, // TODO: implement streaming inference
            input_cost_per_million: Some(0.0), // Free - local inference
            output_cost_per_million: Some(0.0),
        }])
    }

    async fn complete(&self, request: CompletionRequest) -> Result<CompletionResponse> {
        // For now, return an error indicating this needs model implementation
        // In a full implementation, we would:
        // 1. Format the prompt from messages
        // 2. Tokenize using the tokenizer
        // 3. Run inference on the model
        // 4. Decode tokens back to text

        tracing::debug!(
            model = %self.model_name,
            message_count = request.messages.len(),
            "Local CUDA inference requested (not yet implemented)"
        );

        // Return a generic error without exposing user prompt content
        Err(anyhow!(
            "Local CUDA inference not yet implemented. \
             Please configure a cloud provider (Anthropic, OpenAI, etc.) for completion requests."
        ))
    }

    async fn complete_stream(
        &self,
        request: CompletionRequest,
    ) -> Result<BoxStream<'static, StreamChunk>> {
        // TODO: Implement streaming inference with candle
        Err(anyhow!(
            "Streaming inference not yet implemented for local_cuda provider"
        ))
    }
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
            messages: vec![
                Message {
                    role: Role::User,
                    content: vec![ContentPart::Text {
                        text: "This is a sensitive prompt that should not appear in error messages!".to_string(),
                    }],
                },
            ],
            model: "test-model".to_string(),
            max_tokens: Some(100),
            temperature: None,
            stream: false,
            tools: vec![],
            tool_choice: None,
            response_format: None,
        };

        let result = provider.complete(request).await;
        assert!(result.is_err());
        
        let error_message = result.unwrap_err().to_string();
        
        // Verify error message does not contain user prompt content
        assert!(!error_message.contains("sensitive prompt"));
        assert!(!error_message.contains("should not appear"));
        
        // Verify error message contains generic "not implemented" text
        assert!(error_message.contains("not yet implemented") || error_message.contains("not implemented"));
    }

    #[tokio::test]
    async fn test_complete_stream_error_message_no_prompt_exposure() {
        let provider = LocalCudaProvider::new("test-model".to_string()).unwrap();
        
        let request = CompletionRequest {
            messages: vec![
                Message {
                    role: Role::User,
                    content: vec![ContentPart::Text {
                        text: "Another sensitive prompt for streaming test".to_string(),
                    }],
                },
            ],
            model: "test-model".to_string(),
            max_tokens: Some(100),
            temperature: None,
            stream: true,
            tools: vec![],
            tool_choice: None,
            response_format: None,
        };

        let result = provider.complete_stream(request).await;
        assert!(result.is_err());
        
        let error_message = result.unwrap_err().to_string();
        
        // Verify error message does not contain user prompt content
        assert!(!error_message.contains("sensitive prompt"));
        assert!(!error_message.contains("streaming test"));
        
        // Verify error message contains generic "not implemented" text
        assert!(error_message.contains("not yet implemented") || error_message.contains("not implemented"));
    }
}
