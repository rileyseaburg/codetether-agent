//! Sequential management API discovery and deterministic catalog order.
use super::*;
use codetether_bedrock_protocol::estimates::{estimate_context_window, estimate_max_output};
use codetether_provider_core::ModelInfo;
use foundation::foundation_model_to_info;
use profile::inference_profile_to_info;
use serde_json::Value;
use std::collections::HashMap;
impl BedrockClient<'_> {
    /// Discover foundation models and inference profiles.
    ///
    /// # Arguments
    /// Uses this client's region and authentication context.
    ///
    /// # Returns
    /// Deduplicated models sorted by ID; individual API failures are skipped.
    ///
    /// # Errors
    /// Retains a fallible result for discovery callers.
    ///
    /// # Examples
    /// See [BedrockClient] for construction.
    pub async fn discover_models(&self) -> Result<Vec<ModelInfo>> {
        let mut models: HashMap<String, ModelInfo> = HashMap::new();
        self.discover_foundation_models(&mut models).await;
        self.discover_inference_profiles(&mut models).await;

        let mut result: Vec<ModelInfo> = models.into_values().collect();
        result.sort_by(|a, b| a.id.cmp(&b.id));

        tracing::info!(
            provider = "bedrock",
            model_count = result.len(),
            "Discovered Bedrock models dynamically"
        );

        Ok(result)
    }
}

mod foundation;
mod foundation_fetch;
mod modalities;
mod profile;
mod profile_fetch;
mod profile_filter;
