//! AI Provider abstraction layer
//!
//! Unified interface for multiple AI providers (OpenAI, Anthropic, Google, etc.)

pub mod anthropic;
pub mod google;
pub mod models;
pub mod openai;

use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

/// A message in a conversation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub role: Role,
    pub content: Vec<ContentPart>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    System,
    User,
    Assistant,
    Tool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ContentPart {
    Text { text: String },
    Image { url: String, mime_type: Option<String> },
    File { path: String, mime_type: Option<String> },
    ToolCall { id: String, name: String, arguments: String },
    ToolResult { tool_call_id: String, content: String },
}

/// Tool definition for the model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolDefinition {
    pub name: String,
    pub description: String,
    pub parameters: serde_json::Value, // JSON Schema
}

/// Request to generate a completion
#[derive(Debug, Clone)]
pub struct CompletionRequest {
    pub messages: Vec<Message>,
    pub tools: Vec<ToolDefinition>,
    pub model: String,
    pub temperature: Option<f32>,
    pub top_p: Option<f32>,
    pub max_tokens: Option<usize>,
    pub stop: Vec<String>,
}

/// A streaming chunk from the model
#[derive(Debug, Clone)]
pub enum StreamChunk {
    Text(String),
    ToolCallStart { id: String, name: String },
    ToolCallDelta { id: String, arguments_delta: String },
    ToolCallEnd { id: String },
    Done { usage: Option<Usage> },
    Error(String),
}

/// Token usage information
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Usage {
    pub prompt_tokens: usize,
    pub completion_tokens: usize,
    pub total_tokens: usize,
    pub cache_read_tokens: Option<usize>,
    pub cache_write_tokens: Option<usize>,
}

/// Response from a completion request
#[derive(Debug, Clone)]
pub struct CompletionResponse {
    pub message: Message,
    pub usage: Usage,
    pub finish_reason: FinishReason,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum FinishReason {
    Stop,
    Length,
    ToolCalls,
    ContentFilter,
    Error,
}

/// Provider trait that all AI providers must implement
#[async_trait]
pub trait Provider: Send + Sync {
    /// Get the provider name
    fn name(&self) -> &str;

    /// List available models
    async fn list_models(&self) -> Result<Vec<ModelInfo>>;

    /// Generate a completion
    async fn complete(&self, request: CompletionRequest) -> Result<CompletionResponse>;

    /// Generate a streaming completion
    async fn complete_stream(
        &self,
        request: CompletionRequest,
    ) -> Result<futures::stream::BoxStream<'static, StreamChunk>>;
}

/// Information about a model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelInfo {
    pub id: String,
    pub name: String,
    pub provider: String,
    pub context_window: usize,
    pub max_output_tokens: Option<usize>,
    pub supports_vision: bool,
    pub supports_tools: bool,
    pub supports_streaming: bool,
    pub input_cost_per_million: Option<f64>,
    pub output_cost_per_million: Option<f64>,
}

/// Registry of available providers
pub struct ProviderRegistry {
    providers: HashMap<String, Arc<dyn Provider>>,
}

impl ProviderRegistry {
    pub fn new() -> Self {
        Self {
            providers: HashMap::new(),
        }
    }

    /// Register a provider
    pub fn register(&mut self, provider: Arc<dyn Provider>) {
        self.providers.insert(provider.name().to_string(), provider);
    }

    /// Get a provider by name
    pub fn get(&self, name: &str) -> Option<Arc<dyn Provider>> {
        self.providers.get(name).cloned()
    }

    /// List all registered providers
    pub fn list(&self) -> Vec<&str> {
        self.providers.keys().map(|s| s.as_str()).collect()
    }

    /// Initialize with default providers from config
    pub async fn from_config(config: &crate::config::Config) -> Result<Self> {
        let mut registry = Self::new();

        // Always try to initialize OpenAI if key is available
        if let Some(provider_config) = config.providers.get("openai") {
            if let Some(api_key) = &provider_config.api_key {
                registry.register(Arc::new(openai::OpenAIProvider::new(api_key.clone())?));
            }
        } else if let Ok(api_key) = std::env::var("OPENAI_API_KEY") {
            registry.register(Arc::new(openai::OpenAIProvider::new(api_key)?));
        }

        // Initialize Anthropic
        if let Some(provider_config) = config.providers.get("anthropic") {
            if let Some(api_key) = &provider_config.api_key {
                registry.register(Arc::new(anthropic::AnthropicProvider::new(api_key.clone())?));
            }
        } else if let Ok(api_key) = std::env::var("ANTHROPIC_API_KEY") {
            registry.register(Arc::new(anthropic::AnthropicProvider::new(api_key)?));
        }

        // Initialize Google
        if let Some(provider_config) = config.providers.get("google") {
            if let Some(api_key) = &provider_config.api_key {
                registry.register(Arc::new(google::GoogleProvider::new(api_key.clone())?));
            }
        } else if let Ok(api_key) = std::env::var("GOOGLE_API_KEY") {
            registry.register(Arc::new(google::GoogleProvider::new(api_key)?));
        }

        Ok(registry)
    }

    /// Initialize providers from HashiCorp Vault
    /// 
    /// This loads API keys from Vault and creates providers dynamically.
    /// Supports OpenAI-compatible providers via base_url.
    pub async fn from_vault() -> Result<Self> {
        let mut registry = Self::new();
        
        let manager = match crate::secrets::secrets_manager() {
            Some(m) => m,
            None => {
                tracing::warn!("Vault not configured, no providers will be available");
                return Ok(registry);
            }
        };

        // List all configured providers from Vault
        let providers = manager.list_configured_providers().await?;
        tracing::info!("Found {} providers configured in Vault", providers.len());

        for provider_id in providers {
            let secrets = match manager.get_provider_secrets(&provider_id).await? {
                Some(s) => s,
                None => continue,
            };

            let api_key = match secrets.api_key {
                Some(key) => key,
                None => continue,
            };

            // Determine which provider implementation to use
            match provider_id.as_str() {
                // Native providers
                "anthropic" | "anthropic-eu" | "anthropic-asia" => {
                    match anthropic::AnthropicProvider::new(api_key) {
                        Ok(p) => registry.register(Arc::new(p)),
                        Err(e) => tracing::warn!("Failed to init {}: {}", provider_id, e),
                    }
                }
                "google" | "google-vertex" => {
                    match google::GoogleProvider::new(api_key) {
                        Ok(p) => registry.register(Arc::new(p)),
                        Err(e) => tracing::warn!("Failed to init {}: {}", provider_id, e),
                    }
                }
                // OpenAI-compatible providers (with custom base_url)
                "moonshotai" | "moonshotai-cn" | "deepseek" | "groq" | "togetherai" 
                | "fireworks-ai" | "openrouter" | "mistral" | "nvidia" | "alibaba"
                | "openai" | "azure" | "stepfun" => {
                    if let Some(base_url) = secrets.base_url {
                        match openai::OpenAIProvider::with_base_url(api_key, base_url, &provider_id) {
                            Ok(p) => registry.register(Arc::new(p)),
                            Err(e) => tracing::warn!("Failed to init {}: {}", provider_id, e),
                        }
                    } else if provider_id == "openai" {
                        // OpenAI doesn't need a custom base_url
                        match openai::OpenAIProvider::new(api_key) {
                            Ok(p) => registry.register(Arc::new(p)),
                            Err(e) => tracing::warn!("Failed to init openai: {}", e),
                        }
                    } else {
                        // Try using the base_url from the models API
                        if let Ok(catalog) = models::ModelCatalog::fetch().await {
                            if let Some(provider_info) = catalog.get_provider(&provider_id) {
                                if let Some(api_url) = &provider_info.api {
                                    match openai::OpenAIProvider::with_base_url(api_key, api_url.clone(), &provider_id) {
                                        Ok(p) => registry.register(Arc::new(p)),
                                        Err(e) => tracing::warn!("Failed to init {}: {}", provider_id, e),
                                    }
                                }
                            }
                        }
                    }
                }
                // Unknown providers - try as OpenAI-compatible with base_url from API
                other => {
                    if let Some(base_url) = secrets.base_url {
                        match openai::OpenAIProvider::with_base_url(api_key, base_url, other) {
                            Ok(p) => registry.register(Arc::new(p)),
                            Err(e) => tracing::warn!("Failed to init {}: {}", other, e),
                        }
                    } else {
                        tracing::debug!("Unknown provider {} without base_url, skipping", other);
                    }
                }
            }
        }

        tracing::info!("Registered {} providers from Vault", registry.providers.len());
        Ok(registry)
    }
}

impl Default for ProviderRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Parse a model string in the format "provider/model"
pub fn parse_model_string(s: &str) -> (Option<&str>, &str) {
    if let Some((provider, model)) = s.split_once('/') {
        (Some(provider), model)
    } else {
        (None, s)
    }
}
