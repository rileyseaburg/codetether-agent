//! AI Provider abstraction layer
//!
//! Unified interface for multiple AI providers (OpenAI, Anthropic, Google, StepFun, etc.)

pub mod anthropic;
pub mod bedrock;
pub mod copilot;
pub mod google;
pub mod metrics;
pub mod models;
pub mod moonshot;
pub mod openai;
pub mod openai_codex;
pub mod openrouter;
pub mod stepfun;
pub mod vertex_anthropic;
pub mod vertex_glm;
pub mod zai;

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
    Text {
        text: String,
    },
    Image {
        url: String,
        mime_type: Option<String>,
    },
    File {
        path: String,
        mime_type: Option<String>,
    },
    ToolCall {
        id: String,
        name: String,
        arguments: String,
        /// Thought signature for Gemini 3.x models - must be passed back when sending tool results
        #[serde(skip_serializing_if = "Option::is_none")]
        thought_signature: Option<String>,
    },
    ToolResult {
        tool_call_id: String,
        content: String,
    },
    Thinking {
        text: String,
    },
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

impl std::fmt::Debug for ProviderRegistry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ProviderRegistry")
            .field("provider_count", &self.providers.len())
            .field("providers", &self.providers.keys().collect::<Vec<_>>())
            .finish()
    }
}

impl ProviderRegistry {
    pub fn new() -> Self {
        Self {
            providers: HashMap::new(),
        }
    }

    /// Register a provider (automatically wrapped with metrics instrumentation)
    pub fn register(&mut self, provider: Arc<dyn Provider>) {
        let name = provider.name().to_string();
        let wrapped = metrics::MetricsProvider::wrap(provider);
        self.providers.insert(name, wrapped);
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
                let provider = if let Some(base_url) = provider_config.base_url.clone() {
                    anthropic::AnthropicProvider::with_base_url(
                        api_key.clone(),
                        base_url,
                        "anthropic",
                    )?
                } else {
                    anthropic::AnthropicProvider::new(api_key.clone())?
                };
                registry.register(Arc::new(provider));
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

        // Initialize Novita (OpenAI-compatible)
        if let Some(provider_config) = config.providers.get("novita") {
            if let Some(api_key) = &provider_config.api_key {
                let base_url = provider_config
                    .base_url
                    .clone()
                    .unwrap_or_else(|| "https://api.novita.ai/openai/v1".to_string());
                registry.register(Arc::new(openai::OpenAIProvider::with_base_url(
                    api_key.clone(),
                    base_url,
                    "novita",
                )?));
            }
        }

        // Initialize Bedrock via AWS credentials (env vars or ~/.aws/credentials)
        if let Some(creds) = bedrock::AwsCredentials::from_environment() {
            let region = bedrock::AwsCredentials::detect_region()
                .unwrap_or_else(|| bedrock::DEFAULT_REGION.to_string());
            match bedrock::BedrockProvider::with_credentials(creds, region) {
                Ok(p) => registry.register(Arc::new(p)),
                Err(e) => tracing::warn!("Failed to init bedrock from AWS credentials: {}", e),
            }
        }

        Ok(registry)
    }

    /// Initialize providers from HashiCorp Vault
    ///
    /// This loads API keys from Vault and creates providers dynamically.
    /// Supports OpenAI-compatible providers via base_url.
    pub async fn from_vault() -> Result<Self> {
        let mut registry = Self::new();

        if let Some(manager) = crate::secrets::secrets_manager() {
            // List all configured providers from Vault
            let providers = manager.list_configured_providers().await?;
            tracing::info!("Found {} providers configured in Vault", providers.len());

            for provider_id in providers {
                let secrets = match manager.get_provider_secrets(&provider_id).await? {
                    Some(s) => s,
                    None => continue,
                };

                // Handle Bedrock before api_key extraction since it can use
                // AWS IAM credentials instead of an API key.
                if matches!(provider_id.as_str(), "bedrock" | "aws-bedrock") {
                    let region = secrets
                        .extra
                        .get("region")
                        .and_then(|v| v.as_str())
                        .unwrap_or("us-east-1")
                        .to_string();

                    // Prefer SigV4 if AWS credentials are in Vault
                    let aws_key_id = secrets
                        .extra
                        .get("aws_access_key_id")
                        .and_then(|v| v.as_str());
                    let aws_secret = secrets
                        .extra
                        .get("aws_secret_access_key")
                        .and_then(|v| v.as_str());

                    let result = if let (Some(key_id), Some(secret)) = (aws_key_id, aws_secret) {
                        let creds = bedrock::AwsCredentials {
                            access_key_id: key_id.to_string(),
                            secret_access_key: secret.to_string(),
                            session_token: secrets
                                .extra
                                .get("aws_session_token")
                                .and_then(|v| v.as_str())
                                .map(|s| s.to_string()),
                        };
                        bedrock::BedrockProvider::with_credentials(creds, region)
                    } else if let Some(ref key) = secrets.api_key {
                        bedrock::BedrockProvider::with_region(key.clone(), region)
                    } else {
                        // Try auto-detecting from environment as last resort
                        if let Some(creds) = bedrock::AwsCredentials::from_environment() {
                            bedrock::BedrockProvider::with_credentials(creds, region)
                        } else {
                            Err(anyhow::anyhow!(
                                "No AWS credentials or API key found for Bedrock"
                            ))
                        }
                    };

                    match result {
                        Ok(p) => registry.register(Arc::new(p)),
                        Err(e) => tracing::warn!("Failed to init {}: {}", provider_id, e),
                    }
                    continue;
                }

                // Handle Vertex AI GLM before api_key extraction since it uses
                // service account JWT auth (no api_key needed).
                if matches!(provider_id.as_str(), "vertex-glm" | "vertex-ai" | "gcp-glm") {
                    let sa_json = secrets
                        .extra
                        .get("service_account_json")
                        .and_then(|v| v.as_str());

                    if let Some(sa_json) = sa_json {
                        let project_id = secrets
                            .extra
                            .get("project_id")
                            .and_then(|v| v.as_str())
                            .or_else(|| secrets.extra.get("projectId").and_then(|v| v.as_str()))
                            .map(|s| s.to_string());

                        match vertex_glm::VertexGlmProvider::new(sa_json, project_id) {
                            Ok(p) => registry.register(Arc::new(p)),
                            Err(e) => tracing::warn!("Failed to init vertex-glm: {e}"),
                        }
                    } else {
                        tracing::warn!(
                            "vertex-glm provider requires service_account_json in Vault secrets"
                        );
                    }
                    continue;
                }

                // Handle Vertex AI Anthropic (Claude models via GCP)
                // Uses service account JWT auth like vertex-glm
                if matches!(
                    provider_id.as_str(),
                    "vertex-anthropic" | "vertex-claude" | "gcp-anthropic"
                ) {
                    let sa_json = secrets
                        .extra
                        .get("service_account_json")
                        .and_then(|v| v.as_str());

                    if let Some(sa_json) = sa_json {
                        let project_id = secrets
                            .extra
                            .get("project_id")
                            .and_then(|v| v.as_str())
                            .or_else(|| secrets.extra.get("projectId").and_then(|v| v.as_str()))
                            .map(|s| s.to_string());

                        match vertex_anthropic::VertexAnthropicProvider::new(sa_json, project_id) {
                            Ok(p) => registry.register(Arc::new(p)),
                            Err(e) => tracing::warn!("Failed to init vertex-anthropic: {e}"),
                        }
                    } else {
                        tracing::warn!(
                            "vertex-anthropic provider requires service_account_json in Vault secrets"
                        );
                    }
                    continue;
                }

                // Handle OpenAI Codex (ChatGPT subscription) before api_key extraction
                // since it uses OAuth credentials (access_token, refresh_token, expires_at)
                if matches!(provider_id.as_str(), "openai-codex" | "codex" | "chatgpt") {
                    let access_token = secrets.extra.get("access_token").and_then(|v| v.as_str());
                    let refresh_token = secrets.extra.get("refresh_token").and_then(|v| v.as_str());
                    let expires_at = secrets.extra.get("expires_at").and_then(|v| v.as_u64());

                    match (access_token, refresh_token, expires_at) {
                        (Some(access), Some(refresh), Some(expires)) => {
                            let creds = openai_codex::OAuthCredentials {
                                access_token: access.to_string(),
                                refresh_token: refresh.to_string(),
                                expires_at: expires,
                            };
                            let provider =
                                openai_codex::OpenAiCodexProvider::from_credentials(creds);
                            registry.register(Arc::new(provider));
                        }
                        _ => {
                            tracing::warn!(
                                "openai-codex provider requires access_token, refresh_token, and expires_at in Vault secrets"
                            );
                        }
                    }
                    continue;
                }

                let api_key = match secrets.api_key {
                    Some(key) => key,
                    None => continue,
                };

                // Determine which provider implementation to use
                match provider_id.as_str() {
                    // Native providers
                    "anthropic" | "anthropic-eu" | "anthropic-asia" => {
                        let base_url = secrets
                            .base_url
                            .clone()
                            .unwrap_or_else(|| "https://api.anthropic.com".to_string());
                        match anthropic::AnthropicProvider::with_base_url(
                            api_key,
                            base_url,
                            &provider_id,
                        ) {
                            Ok(p) => registry.register(Arc::new(p)),
                            Err(e) => tracing::warn!("Failed to init {}: {}", provider_id, e),
                        }
                    }
                    "google" | "google-vertex" => match google::GoogleProvider::new(api_key) {
                        Ok(p) => registry.register(Arc::new(p)),
                        Err(e) => tracing::warn!("Failed to init {}: {}", provider_id, e),
                    },
                    // StepFun - native provider (direct API, not via OpenRouter)
                    "stepfun" => match stepfun::StepFunProvider::new(api_key) {
                        Ok(p) => registry.register(Arc::new(p)),
                        Err(e) => tracing::warn!("Failed to init stepfun: {}", e),
                    },
                    // OpenRouter - native provider with support for extended response formats
                    "openrouter" => match openrouter::OpenRouterProvider::new(api_key) {
                        Ok(p) => registry.register(Arc::new(p)),
                        Err(e) => tracing::warn!("Failed to init openrouter: {}", e),
                    },
                    // Moonshot AI - native provider for Kimi models
                    "moonshotai" | "moonshotai-cn" => {
                        match moonshot::MoonshotProvider::new(api_key) {
                            Ok(p) => registry.register(Arc::new(p)),
                            Err(e) => tracing::warn!("Failed to init moonshotai: {}", e),
                        }
                    }
                    // GitHub Copilot providers require custom headers/token semantics
                    "github-copilot" => {
                        let result = if let Some(base_url) = secrets.base_url.clone() {
                            copilot::CopilotProvider::with_base_url(
                                api_key,
                                base_url,
                                "github-copilot",
                            )
                        } else {
                            copilot::CopilotProvider::new(api_key)
                        };

                        match result {
                            Ok(p) => registry.register(Arc::new(p)),
                            Err(e) => tracing::warn!("Failed to init github-copilot: {}", e),
                        }
                    }
                    "github-copilot-enterprise" => {
                        let enterprise_url = secrets
                            .extra
                            .get("enterpriseUrl")
                            .and_then(|v| v.as_str())
                            .or_else(|| {
                                secrets.extra.get("enterprise_url").and_then(|v| v.as_str())
                            });

                        let result = if let Some(base_url) = secrets.base_url.clone() {
                            copilot::CopilotProvider::with_base_url(
                                api_key,
                                base_url,
                                "github-copilot-enterprise",
                            )
                        } else if let Some(url) = enterprise_url {
                            copilot::CopilotProvider::enterprise(api_key, url.to_string())
                        } else {
                            copilot::CopilotProvider::with_base_url(
                                api_key,
                                "https://api.githubcopilot.com".to_string(),
                                "github-copilot-enterprise",
                            )
                        };

                        match result {
                            Ok(p) => registry.register(Arc::new(p)),
                            Err(e) => {
                                tracing::warn!("Failed to init github-copilot-enterprise: {}", e)
                            }
                        }
                    }
                    // Z.AI (formerly ZhipuAI) — first-class provider for GLM models
                    "zhipuai" | "zai" => {
                        let base_url = secrets
                            .base_url
                            .clone()
                            .unwrap_or_else(|| "https://api.z.ai/api/paas/v4".to_string());
                        match zai::ZaiProvider::with_base_url(api_key, base_url) {
                            Ok(p) => registry.register(Arc::new(p)),
                            Err(e) => tracing::warn!("Failed to init zai: {}", e),
                        }
                    }

                    // Cerebras - OpenAI-compatible fast inference
                    "cerebras" => {
                        let base_url = secrets
                            .base_url
                            .clone()
                            .unwrap_or_else(|| "https://api.cerebras.ai/v1".to_string());
                        match openai::OpenAIProvider::with_base_url(api_key, base_url, "cerebras") {
                            Ok(p) => registry.register(Arc::new(p)),
                            Err(e) => tracing::warn!("Failed to init cerebras: {}", e),
                        }
                    }
                    // MiniMax - Anthropic-compatible API harness (recommended by MiniMax docs)
                    // "minimax" uses the coding-plan key (regular models)
                    // "minimax-credits" uses the credits-based key (highspeed models)
                    "minimax" | "minimax-credits" => {
                        let base_url = secrets
                            .base_url
                            .clone()
                            .unwrap_or_else(|| "https://api.minimax.io/anthropic".to_string());
                        let base_url = normalize_minimax_anthropic_base_url(&base_url);
                        match anthropic::AnthropicProvider::with_base_url(
                            api_key,
                            base_url,
                            &provider_id,
                        ) {
                            Ok(p) => registry.register(Arc::new(p)),
                            Err(e) => tracing::warn!("Failed to init {}: {}", provider_id, e),
                        }
                    }
                    // OpenAI-compatible providers (with custom base_url)
                    "deepseek" | "groq" | "togetherai" | "fireworks-ai" | "mistral" | "nvidia"
                    | "alibaba" | "openai" | "azure" | "novita" => {
                        if let Some(base_url) = secrets.base_url.clone() {
                            match openai::OpenAIProvider::with_base_url(
                                api_key,
                                base_url,
                                &provider_id,
                            ) {
                                Ok(p) => registry.register(Arc::new(p)),
                                Err(e) => tracing::warn!("Failed to init {}: {}", provider_id, e),
                            }
                        } else if provider_id == "openai" {
                            // OpenAI doesn't need a custom base_url
                            match openai::OpenAIProvider::new(api_key) {
                                Ok(p) => registry.register(Arc::new(p)),
                                Err(e) => tracing::warn!("Failed to init openai: {}", e),
                            }
                        } else if provider_id == "novita" {
                            let base_url = "https://api.novita.ai/openai/v1".to_string();
                            match openai::OpenAIProvider::with_base_url(
                                api_key,
                                base_url,
                                &provider_id,
                            ) {
                                Ok(p) => registry.register(Arc::new(p)),
                                Err(e) => tracing::warn!("Failed to init {}: {}", provider_id, e),
                            }
                        } else {
                            tracing::warn!(
                                "Provider {} has no built-in base_url; set base_url in Vault secrets",
                                provider_id
                            );
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
                            tracing::debug!(
                                "Unknown provider {} without base_url, skipping",
                                other
                            );
                        }
                    }
                }
            }
        } else {
            tracing::warn!("Vault not configured, no providers will be available from Vault");
        }

        // If Bedrock wasn't registered via Vault, try auto-detecting AWS credentials
        if !registry.providers.contains_key("bedrock") {
            if let Some(creds) = bedrock::AwsCredentials::from_environment() {
                let region = bedrock::AwsCredentials::detect_region()
                    .unwrap_or_else(|| "us-east-1".to_string());
                match bedrock::BedrockProvider::with_credentials(creds, region) {
                    Ok(p) => {
                        tracing::info!("Registered Bedrock provider from local AWS credentials");
                        registry.register(Arc::new(p));
                    }
                    Err(e) => tracing::warn!("Failed to init bedrock from AWS credentials: {}", e),
                }
            }
        }

        // Fallback to environment variables for common providers if not registered via Vault
        if !registry.providers.contains_key("openai") {
            if let Ok(api_key) = std::env::var("OPENAI_API_KEY") {
                match openai::OpenAIProvider::new(api_key) {
                    Ok(p) => {
                        tracing::info!("Registered OpenAI provider from OPENAI_API_KEY env var");
                        registry.register(Arc::new(p));
                    }
                    Err(e) => tracing::warn!("Failed to init openai from env: {}", e),
                }
            }
        }

        if !registry.providers.contains_key("anthropic") {
            if let Ok(api_key) = std::env::var("ANTHROPIC_API_KEY") {
                match anthropic::AnthropicProvider::new(api_key) {
                    Ok(p) => {
                        tracing::info!("Registered Anthropic provider from ANTHROPIC_API_KEY env var");
                        registry.register(Arc::new(p));
                    }
                    Err(e) => tracing::warn!("Failed to init anthropic from env: {}", e),
                }
            }
        }

        if !registry.providers.contains_key("google") {
            if let Ok(api_key) = std::env::var("GOOGLE_API_KEY") {
                match google::GoogleProvider::new(api_key) {
                    Ok(p) => {
                        tracing::info!("Registered Google provider from GOOGLE_API_KEY env var");
                        registry.register(Arc::new(p));
                    }
                    Err(e) => tracing::warn!("Failed to init google from env: {}", e),
                }
            }
        }

        if !registry.providers.contains_key("openrouter") {
            if let Ok(api_key) = std::env::var("OPENROUTER_API_KEY") {
                match openrouter::OpenRouterProvider::new(api_key) {
                    Ok(p) => {
                        tracing::info!("Registered OpenRouter provider from OPENROUTER_API_KEY env var");
                        registry.register(Arc::new(p));
                    }
                    Err(e) => tracing::warn!("Failed to init openrouter from env: {}", e),
                }
            }
        }

        tracing::info!(
            "Registered {} providers (Vault + env fallback)",
            registry.providers.len()
        );
        Ok(registry)
    }
}

fn normalize_minimax_anthropic_base_url(base_url: &str) -> String {
    let trimmed = base_url.trim().trim_end_matches('/');
    if trimmed.eq_ignore_ascii_case("https://api.minimax.io/v1") {
        "https://api.minimax.io/anthropic".to_string()
    } else {
        trimmed.to_string()
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
