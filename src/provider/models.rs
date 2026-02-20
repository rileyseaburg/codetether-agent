//! Model catalog from CodeTether API
//!
//! Fetches model information from the "models" endpoint for each provider, including capabilities, costs, and limits. This is used to enrich our internal model information and provide better recommendations and cost estimates.
//! The catalog is fetched on demand and cached in memory. It also integrates with the secrets manager to check which providers have API keys configured, allowing us to filter available models accordingly.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Model cost information (per million tokens)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelCost {
    pub input: f64,
    pub output: f64,
    #[serde(default)]
    pub cache_read: Option<f64>,
    #[serde(default)]
    pub cache_write: Option<f64>,
    #[serde(default)]
    pub reasoning: Option<f64>,
}

/// Model limits
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelLimit {
    #[serde(default)]
    pub context: u64,
    #[serde(default)]
    pub output: u64,
}

/// Model modalities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelModalities {
    #[serde(default)]
    pub input: Vec<String>,
    #[serde(default)]
    pub output: Vec<String>,
}

/// Model information from the API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiModelInfo {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub family: Option<String>,
    #[serde(default)]
    pub attachment: bool,
    #[serde(default)]
    pub reasoning: bool,
    #[serde(default)]
    pub tool_call: bool,
    #[serde(default)]
    pub structured_output: Option<bool>,
    #[serde(default)]
    pub temperature: Option<bool>,
    #[serde(default)]
    pub knowledge: Option<String>,
    #[serde(default)]
    pub release_date: Option<String>,
    #[serde(default)]
    pub last_updated: Option<String>,
    #[serde(default)]
    pub modalities: Option<ModelModalities>,
    #[serde(default)]
    pub open_weights: bool,
    #[serde(default)]
    pub cost: Option<ModelCost>,
    #[serde(default)]
    pub limit: Option<ModelLimit>,
}

/// Provider information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderInfo {
    pub id: String,
    #[serde(default)]
    pub env: Vec<String>,
    #[serde(default)]
    pub npm: Option<String>,
    #[serde(default)]
    pub api: Option<String>,
    pub name: String,
    #[serde(default)]
    pub doc: Option<String>,
    #[serde(default)]
    pub models: HashMap<String, ApiModelInfo>,
}

/// The full API response
pub type ModelsApiResponse = HashMap<String, ProviderInfo>;

/// Model catalog for looking up models
#[derive(Debug, Clone, Default)]
pub struct ModelCatalog {
    providers: HashMap<String, ProviderInfo>,
}

#[allow(dead_code)]
impl ModelCatalog {
    /// Create an empty catalog
    pub fn new() -> Self {
        Self::default()
    }

    /// Fetch models from the CodeTether API
    pub async fn fetch() -> anyhow::Result<Self> {
        const MODELS_URL: &str = "https://models.dev/api.json";
        tracing::info!("Fetching models from {}", MODELS_URL);
        let response = reqwest::get(MODELS_URL).await?;
        let providers: ModelsApiResponse = response.json().await?;
        tracing::info!("Loaded {} providers", providers.len());
        Ok(Self { providers })
    }

    /// Fetch models with a custom URL (for testing or alternate sources)
    #[allow(dead_code)]
    pub async fn fetch_from(url: &str) -> anyhow::Result<Self> {
        let response = reqwest::get(url).await?;
        let providers: ModelsApiResponse = response.json().await?;
        Ok(Self { providers })
    }

    /// Check if a provider has an API key configured in HashiCorp Vault
    ///
    /// NOTE: This is a sync wrapper that checks the Vault cache.
    /// For initial population, use `check_provider_api_key_async`.
    pub fn provider_has_api_key(&self, provider_id: &str) -> bool {
        // Check if we have it cached in the secrets manager
        if let Some(manager) = crate::secrets::secrets_manager() {
            // Sync check - only works if already cached
            let cache = manager.cache.try_read();
            if let Ok(cache) = cache {
                return cache.contains_key(provider_id);
            }
        }
        false
    }

    /// Async check if a provider has an API key in Vault
    pub async fn check_provider_api_key_async(&self, provider_id: &str) -> bool {
        crate::secrets::has_api_key(provider_id).await
    }

    /// Pre-load API key availability from Vault for all providers
    pub async fn preload_available_providers(&self) -> Vec<String> {
        let mut available = Vec::new();

        if let Some(manager) = crate::secrets::secrets_manager() {
            // List all configured providers from Vault
            if let Ok(providers) = manager.list_configured_providers().await {
                for provider_id in providers {
                    // Verify each one actually has an API key
                    if manager.has_api_key(&provider_id).await {
                        available.push(provider_id);
                    }
                }
            }
        }

        available
    }

    /// Get list of providers that have API keys configured (sync, uses cache)
    pub fn available_providers(&self) -> Vec<&str> {
        self.providers
            .keys()
            .filter(|id| self.provider_has_api_key(id))
            .map(|s| s.as_str())
            .collect()
    }

    /// Get list of providers that have API keys configured (async, checks Vault)
    #[allow(dead_code)]
    pub async fn available_providers_async(&self) -> Vec<String> {
        let mut available = Vec::new();
        for provider_id in self.providers.keys() {
            if self.check_provider_api_key_async(provider_id).await {
                available.push(provider_id.clone());
            }
        }
        available
    }

    /// Get a provider by ID
    pub fn get_provider(&self, provider_id: &str) -> Option<&ProviderInfo> {
        self.providers.get(provider_id)
    }

    /// Get a provider by ID only if it has an API key
    pub fn get_available_provider(&self, provider_id: &str) -> Option<&ProviderInfo> {
        if self.provider_has_api_key(provider_id) {
            self.providers.get(provider_id)
        } else {
            None
        }
    }

    /// Get a model by provider and model ID
    pub fn get_model(&self, provider_id: &str, model_id: &str) -> Option<&ApiModelInfo> {
        self.providers
            .get(provider_id)
            .and_then(|p| p.models.get(model_id))
    }

    /// Get a model only if the provider has an API key
    pub fn get_available_model(&self, provider_id: &str, model_id: &str) -> Option<&ApiModelInfo> {
        if self.provider_has_api_key(provider_id) {
            self.get_model(provider_id, model_id)
        } else {
            None
        }
    }

    /// Find a model by ID across all providers (only available ones)
    pub fn find_model(&self, model_id: &str) -> Option<(&str, &ApiModelInfo)> {
        for (provider_id, provider) in &self.providers {
            if !self.provider_has_api_key(provider_id) {
                continue;
            }
            if let Some(model) = provider.models.get(model_id) {
                return Some((provider_id, model));
            }
        }
        None
    }

    /// Find a model across ALL providers (ignoring API key requirement)
    pub fn find_model_any(&self, model_id: &str) -> Option<(&str, &ApiModelInfo)> {
        for (provider_id, provider) in &self.providers {
            if let Some(model) = provider.models.get(model_id) {
                return Some((provider_id, model));
            }
        }
        None
    }

    /// List all provider IDs (all, not filtered)
    #[allow(dead_code)]
    pub fn provider_ids(&self) -> Vec<&str> {
        self.providers.keys().map(|s| s.as_str()).collect()
    }

    /// Get iterator over all providers and their info (unfiltered, no API key check)
    pub fn all_providers(&self) -> &HashMap<String, ProviderInfo> {
        &self.providers
    }

    /// List models for a provider (only if API key available)
    pub fn models_for_provider(&self, provider_id: &str) -> Vec<&ApiModelInfo> {
        if !self.provider_has_api_key(provider_id) {
            return Vec::new();
        }
        self.providers
            .get(provider_id)
            .map(|p| p.models.values().collect())
            .unwrap_or_default()
    }

    /// Find models with tool calling support (only from available providers)
    pub fn tool_capable_models(&self) -> Vec<(&str, &ApiModelInfo)> {
        let mut result = Vec::new();
        for (provider_id, provider) in &self.providers {
            if !self.provider_has_api_key(provider_id) {
                continue;
            }
            for model in provider.models.values() {
                if model.tool_call {
                    result.push((provider_id.as_str(), model));
                }
            }
        }
        result
    }

    /// Find models with reasoning support (only from available providers)
    pub fn reasoning_models(&self) -> Vec<(&str, &ApiModelInfo)> {
        let mut result = Vec::new();
        for (provider_id, provider) in &self.providers {
            if !self.provider_has_api_key(provider_id) {
                continue;
            }
            for model in provider.models.values() {
                if model.reasoning {
                    result.push((provider_id.as_str(), model));
                }
            }
        }
        result
    }

    /// Get recommended models for coding tasks
    pub fn recommended_coding_models(&self) -> Vec<(&str, &ApiModelInfo)> {
        let preferred_ids = [
            "claude-sonnet-4-6",
            "claude-sonnet-4-20250514",
            "claude-opus-4-20250514",
            "gpt-5-codex",
            "gpt-5.1-codex",
            "gpt-4o",
            "gemini-3.1-pro-preview",
            "gemini-2.5-pro",
            "deepseek-v3.2",
            "step-3.5-flash",
            "glm-5",
            "z-ai/glm-5",
        ];

        let mut result = Vec::new();
        for model_id in preferred_ids {
            if let Some((provider, model)) = self.find_model(model_id) {
                result.push((provider, model));
            }
        }
        result
    }

    /// Convert API model info to our internal ModelInfo format
    #[allow(dead_code)]
    pub fn to_model_info(&self, model: &ApiModelInfo, provider_id: &str) -> super::ModelInfo {
        super::ModelInfo {
            id: model.id.clone(),
            name: model.name.clone(),
            provider: provider_id.to_string(),
            context_window: model
                .limit
                .as_ref()
                .map(|l| l.context as usize)
                .unwrap_or(128_000),
            max_output_tokens: model.limit.as_ref().map(|l| l.output as usize),
            supports_vision: model.attachment,
            supports_tools: model.tool_call,
            supports_streaming: true,
            input_cost_per_million: model.cost.as_ref().map(|c| c.input),
            output_cost_per_million: model.cost.as_ref().map(|c| c.output),
        }
    }
}
