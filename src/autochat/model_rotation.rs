use crate::provider::{ModelInfo, Provider, ProviderRegistry};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::sync::Arc;

const PRIORITY_PROVIDERS: [&str; 8] = [
    "minimax",
    "zai",
    "github-copilot",
    "github-copilot-enterprise",
    "openai-codex",
    "openrouter",
    "minimax-credits",
    "openai",
];

const RESOLUTION_FALLBACKS: [&str; 12] = [
    "minimax",
    "zai",
    "github-copilot",
    "github-copilot-enterprise",
    "openai-codex",
    "openrouter",
    "minimax-credits",
    "openai",
    "anthropic",
    "novita",
    "moonshotai",
    "google",
];

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RelayModelRotation {
    #[serde(default)]
    pub model_refs: Vec<String>,
    #[serde(default)]
    pub cursor: usize,
}

impl RelayModelRotation {
    pub fn fallback(model_ref: &str) -> Self {
        Self {
            model_refs: vec![model_ref.to_string()],
            cursor: 0,
        }
    }

    pub fn next_model_ref(&mut self, fallback: &str) -> String {
        if self.model_refs.is_empty() {
            return fallback.to_string();
        }
        let idx = self.cursor % self.model_refs.len();
        let next = self.model_refs[idx].clone();
        self.cursor = self.cursor.saturating_add(1);
        next
    }
}

pub async fn build_round_robin_model_rotation(
    registry: &Arc<ProviderRegistry>,
    requested_model_ref: &str,
) -> RelayModelRotation {
    let mut rotation = RelayModelRotation::default();
    let mut used_providers = HashSet::<String>::new();

    if let Some(model_ref) = normalize_requested_model_ref(registry, requested_model_ref) {
        add_unique_model_ref(&mut rotation.model_refs, &mut used_providers, &model_ref);
    }

    for provider_name in PRIORITY_PROVIDERS {
        if let Some(model_ref) = preferred_model_ref_for_provider(registry, provider_name).await {
            add_unique_model_ref(&mut rotation.model_refs, &mut used_providers, &model_ref);
        }
    }

    if rotation.model_refs.is_empty() {
        rotation.model_refs.push(requested_model_ref.to_string());
    }

    rotation
}

pub fn resolve_provider_for_model_autochat(
    registry: &Arc<ProviderRegistry>,
    model_ref: &str,
) -> Option<(Arc<dyn Provider>, String)> {
    let (provider_name, model_name) = crate::provider::parse_model_string(model_ref);
    if let Some(provider_name) = provider_name {
        let normalized = normalize_provider_name(provider_name);
        return registry
            .get(normalized)
            .map(|provider| (provider, model_name.to_string()));
    }

    for provider_name in RESOLUTION_FALLBACKS {
        if let Some(provider) = registry.get(provider_name) {
            return Some((provider, model_ref.to_string()));
        }
    }

    registry
        .list()
        .first()
        .copied()
        .and_then(|name| registry.get(name))
        .map(|provider| (provider, model_ref.to_string()))
}

fn normalize_requested_model_ref(
    registry: &Arc<ProviderRegistry>,
    requested_model_ref: &str,
) -> Option<String> {
    let (provider_name, model_name) = crate::provider::parse_model_string(requested_model_ref);
    if let Some(provider_name) = provider_name {
        let normalized = normalize_provider_name(provider_name);
        if registry.get(normalized).is_some() {
            return Some(format!("{normalized}/{model_name}"));
        }
    }

    resolve_provider_for_model_autochat(registry, requested_model_ref)
        .map(|(provider, model)| format!("{}/{}", provider.name(), model))
}

fn add_unique_model_ref(
    model_refs: &mut Vec<String>,
    used_providers: &mut HashSet<String>,
    model_ref: &str,
) {
    let (provider_name, _) = crate::provider::parse_model_string(model_ref);
    let Some(provider_name) = provider_name else {
        return;
    };
    if used_providers.insert(provider_name.to_string()) {
        model_refs.push(model_ref.to_string());
    }
}

async fn preferred_model_ref_for_provider(
    registry: &Arc<ProviderRegistry>,
    provider_name: &str,
) -> Option<String> {
    let provider = registry.get(provider_name)?;
    let listed = provider.list_models().await.unwrap_or_default();
    if let Some(model_id) = choose_model_from_list(provider_name, &listed) {
        return Some(format!("{provider_name}/{model_id}"));
    }
    default_model_for_provider(provider_name).map(|model_id| format!("{provider_name}/{model_id}"))
}

fn choose_model_from_list(provider_name: &str, models: &[ModelInfo]) -> Option<String> {
    if models.is_empty() {
        return None;
    }

    if provider_name == "openrouter" {
        return choose_openrouter_model(models).or_else(|| models.first().map(|m| m.id.clone()));
    }

    if provider_name == "github-copilot" || provider_name == "github-copilot-enterprise" {
        return choose_copilot_model(models).or_else(|| models.first().map(|m| m.id.clone()));
    }

    let preferred = preferred_models_for_provider(provider_name);
    for model_id in preferred {
        if let Some(found) = models
            .iter()
            .find(|model| model.id.eq_ignore_ascii_case(model_id))
        {
            return Some(found.id.clone());
        }
    }

    models.first().map(|m| m.id.clone())
}

fn choose_openrouter_model(models: &[ModelInfo]) -> Option<String> {
    let free_models: Vec<&ModelInfo> = models
        .iter()
        .filter(|model| is_openrouter_free_model(&model.id))
        .collect();
    if !free_models.is_empty() {
        return free_models
            .into_iter()
            .max_by_key(|model| score_openrouter_model(&model.id))
            .map(|model| model.id.clone());
    }
    models
        .iter()
        .max_by_key(|model| score_openrouter_model(&model.id))
        .map(|model| model.id.clone())
}

fn choose_copilot_model(models: &[ModelInfo]) -> Option<String> {
    let preferred = ["gpt-5-mini", "gpt-4.1", "gpt-4o"];
    for model_id in preferred {
        if let Some(found) = models
            .iter()
            .find(|model| model.id.eq_ignore_ascii_case(model_id))
        {
            return Some(found.id.clone());
        }
    }

    models
        .iter()
        .find(|model| {
            model.input_cost_per_million == Some(0.0) && model.output_cost_per_million == Some(0.0)
        })
        .map(|model| model.id.clone())
}

fn is_openrouter_free_model(model_id: &str) -> bool {
    let id = model_id.to_ascii_lowercase();
    id.contains(":free") || id.ends_with("-free")
}

fn score_openrouter_model(model_id: &str) -> i32 {
    let id = model_id.to_ascii_lowercase();
    let mut score = 0;
    if is_openrouter_free_model(&id) {
        score += 1000;
    }
    if id.contains("glm-5") {
        score += 250;
    }
    if id.contains("minimax") {
        score += 220;
    }
    if id.contains("gpt-5-mini") {
        score += 180;
    }
    if id.contains("coder") {
        score += 70;
    }
    score
}

fn preferred_models_for_provider(provider_name: &str) -> &'static [&'static str] {
    match provider_name {
        "minimax" => &["MiniMax-M2.5", "MiniMax-M2.1", "MiniMax-M2"],
        "minimax-credits" => &["MiniMax-M2.5-highspeed", "MiniMax-M2.1-highspeed"],
        "zai" => &["glm-5", "glm-4.7", "glm-4.7-flash"],
        "openai-codex" => &["gpt-5-mini", "gpt-5", "gpt-5.1-codex"],
        "github-copilot" | "github-copilot-enterprise" => &["gpt-5-mini", "gpt-4.1", "gpt-4o"],
        "openrouter" => &["z-ai/glm-5:free", "z-ai/glm-5", "z-ai/glm-4.7:free"],
        _ => &[],
    }
}

fn default_model_for_provider(provider_name: &str) -> Option<&'static str> {
    let defaults = preferred_models_for_provider(provider_name);
    defaults.first().copied()
}

fn normalize_provider_name(provider_name: &str) -> &str {
    if provider_name.eq_ignore_ascii_case("zhipuai") || provider_name.eq_ignore_ascii_case("z-ai") {
        "zai"
    } else {
        provider_name
    }
}

#[cfg(test)]
mod tests {
    use super::{
        RelayModelRotation, choose_copilot_model, choose_openrouter_model, normalize_provider_name,
    };
    use crate::provider::ModelInfo;

    #[test]
    fn relay_rotation_cycles_models() {
        let mut rotation = RelayModelRotation {
            model_refs: vec!["a/x".to_string(), "b/y".to_string()],
            cursor: 0,
        };
        assert_eq!(rotation.next_model_ref("fallback"), "a/x");
        assert_eq!(rotation.next_model_ref("fallback"), "b/y");
        assert_eq!(rotation.next_model_ref("fallback"), "a/x");
    }

    #[test]
    fn provider_alias_normalizes_zhipu_and_z_ai() {
        assert_eq!(normalize_provider_name("zhipuai"), "zai");
        assert_eq!(normalize_provider_name("z-ai"), "zai");
        assert_eq!(normalize_provider_name("openrouter"), "openrouter");
    }

    #[test]
    fn openrouter_prefers_free_glm_models() {
        let models = vec![
            model("openai/gpt-4.1"),
            model("z-ai/glm-5:free"),
            model("moonshot/kimi-k2:free"),
        ];
        assert_eq!(
            choose_openrouter_model(&models).as_deref(),
            Some("z-ai/glm-5:free")
        );
    }

    #[test]
    fn copilot_prefers_gpt_5_mini_when_available() {
        let models = vec![
            model("gpt-4o"),
            model("gpt-5-mini"),
            model("claude-sonnet-4"),
        ];
        assert_eq!(choose_copilot_model(&models).as_deref(), Some("gpt-5-mini"));
    }

    fn model(id: &str) -> ModelInfo {
        ModelInfo {
            id: id.to_string(),
            name: id.to_string(),
            provider: "test".to_string(),
            context_window: 128_000,
            max_output_tokens: Some(16_384),
            supports_vision: false,
            supports_tools: true,
            supports_streaming: true,
            input_cost_per_million: Some(0.0),
            output_cost_per_million: Some(0.0),
        }
    }
}
