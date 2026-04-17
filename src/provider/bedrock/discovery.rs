//! Dynamic model discovery via Bedrock management APIs.
//!
//! Merges [`ListFoundationModels`](https://docs.aws.amazon.com/bedrock/latest/APIReference/API_ListFoundationModels.html)
//! and [`ListInferenceProfiles`](https://docs.aws.amazon.com/bedrock/latest/APIReference/API_ListInferenceProfiles.html)
//! so both on-demand foundation models and cross-region inference profiles
//! appear in the returned catalog.

use super::BedrockProvider;
use super::estimates::{estimate_context_window, estimate_max_output};
use crate::provider::ModelInfo;
use anyhow::Result;
use serde_json::Value;
use std::collections::HashMap;

impl BedrockProvider {
    /// Dynamically discover available Bedrock models.
    ///
    /// Queries both foundation models and system-defined inference profiles,
    /// filters to text-output chat models, and emits [`ModelInfo`] records.
    ///
    /// # Errors
    ///
    /// Returns an error only on unrecoverable network setup issues; individual
    /// API failures are logged and skipped, and the returned list may be empty.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// use codetether_agent::provider::bedrock::{AwsCredentials, BedrockProvider};
    /// use codetether_agent::provider::Provider;
    ///
    /// let creds = AwsCredentials::from_environment().unwrap();
    /// let p = BedrockProvider::with_credentials(creds, "us-west-2".into()).unwrap();
    /// let models = p.list_models().await.unwrap();
    /// assert!(!models.is_empty());
    /// # });
    /// ```
    pub(super) async fn discover_models(&self) -> Result<Vec<ModelInfo>> {
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

    async fn discover_foundation_models(&self, models: &mut HashMap<String, ModelInfo>) {
        let fm_url = format!("{}/foundation-models", self.management_url());
        let Ok(resp) = self.send_request("GET", &fm_url, None, "bedrock").await else {
            return;
        };
        if !resp.status().is_success() {
            return;
        }
        let Ok(data) = resp.json::<Value>().await else {
            return;
        };
        let Some(summaries) = data.get("modelSummaries").and_then(|v| v.as_array()) else {
            return;
        };

        for m in summaries {
            if let Some(info) = foundation_model_to_info(m) {
                models.insert(info.id.clone(), info);
            }
        }
    }

    async fn discover_inference_profiles(&self, models: &mut HashMap<String, ModelInfo>) {
        let ip_url = format!(
            "{}/inference-profiles?typeEquals=SYSTEM_DEFINED&maxResults=200",
            self.management_url()
        );
        let Ok(resp) = self.send_request("GET", &ip_url, None, "bedrock").await else {
            return;
        };
        if !resp.status().is_success() {
            return;
        }
        let Ok(data) = resp.json::<Value>().await else {
            return;
        };
        let Some(profiles) = data
            .get("inferenceProfileSummaries")
            .and_then(|v| v.as_array())
        else {
            return;
        };

        for p in profiles {
            if let Some(info) = inference_profile_to_info(p, models) {
                models.insert(info.id.clone(), info);
            }
        }
    }
}

fn foundation_model_to_info(m: &Value) -> Option<ModelInfo> {
    let model_id = m.get("modelId").and_then(|v| v.as_str()).unwrap_or("");
    let model_name = m.get("modelName").and_then(|v| v.as_str()).unwrap_or("");

    let output_modalities: Vec<&str> = m
        .get("outputModalities")
        .and_then(|v| v.as_array())
        .map(|a| a.iter().filter_map(|v| v.as_str()).collect::<Vec<_>>())
        .unwrap_or_default();

    let input_modalities: Vec<&str> = m
        .get("inputModalities")
        .and_then(|v| v.as_array())
        .map(|a| a.iter().filter_map(|v| v.as_str()).collect::<Vec<_>>())
        .unwrap_or_default();

    let inference_types: Vec<&str> = m
        .get("inferenceTypesSupported")
        .and_then(|v| v.as_array())
        .map(|a| a.iter().filter_map(|v| v.as_str()).collect::<Vec<_>>())
        .unwrap_or_default();

    if !output_modalities.contains(&"TEXT")
        || (!inference_types.contains(&"ON_DEMAND")
            && !inference_types.contains(&"INFERENCE_PROFILE"))
    {
        return None;
    }

    let name_lower = model_name.to_lowercase();
    if ["rerank", "embed", "safeguard", "sonic", "pegasus"]
        .iter()
        .any(|n| name_lower.contains(n))
    {
        return None;
    }

    let streaming = m
        .get("responseStreamingSupported")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let vision = input_modalities.contains(&"IMAGE");

    let actual_id = if model_id.starts_with("amazon.") {
        model_id.to_string()
    } else if inference_types.contains(&"INFERENCE_PROFILE") {
        format!("us.{model_id}")
    } else {
        model_id.to_string()
    };

    Some(ModelInfo {
        id: actual_id.clone(),
        name: format!("{model_name} (Bedrock)"),
        provider: "bedrock".to_string(),
        context_window: estimate_context_window(model_id),
        max_output_tokens: Some(estimate_max_output(model_id)),
        supports_vision: vision,
        supports_tools: true,
        supports_streaming: streaming,
        input_cost_per_million: None,
        output_cost_per_million: None,
    })
}

fn inference_profile_to_info(
    p: &Value,
    models: &HashMap<String, ModelInfo>,
) -> Option<ModelInfo> {
    let pid = p
        .get("inferenceProfileId")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    let pname = p
        .get("inferenceProfileName")
        .and_then(|v| v.as_str())
        .unwrap_or("");

    if !pid.starts_with("us.") || models.contains_key(pid) {
        return None;
    }

    let name_lower = pname.to_lowercase();
    let skip_tokens = [
        "image",
        "stable ",
        "upscale",
        "embed",
        "marengo",
        "outpaint",
        "inpaint",
        "erase",
        "recolor",
        "replace",
        "style ",
        "background",
        "sketch",
        "control",
        "transfer",
        "sonic",
        "pegasus",
        "rerank",
    ];
    if skip_tokens.iter().any(|t| name_lower.contains(t)) {
        return None;
    }

    let vision = pid.contains("llama3-2-11b")
        || pid.contains("llama3-2-90b")
        || pid.contains("pixtral")
        || pid.contains("claude-3")
        || pid.contains("claude-sonnet-4")
        || pid.contains("claude-opus-4")
        || pid.contains("claude-haiku-4");

    let display_name = pname.replace("US ", "");
    let display_name = format!("{} (Bedrock)", display_name.trim());

    Some(ModelInfo {
        id: pid.to_string(),
        name: display_name,
        provider: "bedrock".to_string(),
        context_window: estimate_context_window(pid),
        max_output_tokens: Some(estimate_max_output(pid)),
        supports_vision: vision,
        supports_tools: true,
        supports_streaming: true,
        input_cost_per_million: None,
        output_cost_per_million: None,
    })
}
