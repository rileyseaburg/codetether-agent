//! Model eligibility policy for sub-agent spawning.

use crate::provider::{ProviderRegistry, parse_model_string};

pub(super) fn normalize_model(model: &str) -> String {
    model.trim().to_ascii_lowercase()
}

const SUBSCRIPTION_PROVIDERS: &[&str] = &[
    "openai-codex",
    "github-copilot",
    "github-copilot-enterprise",
    "gemini-web",
    "local_cuda",
    "zai",
    "glm5",
];

const OPENROUTER_BUDGET_ALLOWLIST: &[&str] = &["qwen/qwen3.5-35ba3b"];

fn is_subscription_provider(p: &str) -> bool {
    SUBSCRIPTION_PROVIDERS.contains(&p)
}

pub(super) fn is_free_model_id(id: &str) -> bool {
    let lower = id.to_ascii_lowercase();
    lower.contains(":free") || lower.ends_with("-free")
}

pub(super) fn is_budget_allowlisted_model(provider: &str, model_id: &str) -> bool {
    provider.eq_ignore_ascii_case("openrouter")
        && OPENROUTER_BUDGET_ALLOWLIST
            .iter()
            .any(|allowed| allowed.eq_ignore_ascii_case(model_id.trim()))
}

pub(super) async fn is_free_or_eligible(model: &str, registry: &ProviderRegistry) -> bool {
    let trimmed = model.trim();
    if trimmed.is_empty() {
        return false;
    }

    let (provider_name, model_id) = parse_model_string(trimmed);
    if provider_name.is_none() {
        return is_free_model_id(trimmed);
    }

    let pn = provider_name.unwrap();
    if is_subscription_provider(&pn.to_ascii_lowercase())
        || is_free_model_id(model_id)
        || is_budget_allowlisted_model(pn, model_id)
    {
        return true;
    }

    let Some(p) = registry.get(pn) else {
        return false;
    };
    match p.list_models().await {
        Ok(ms) => ms.into_iter().any(|m| {
            m.id.eq_ignore_ascii_case(model_id)
                && m.input_cost_per_million.unwrap_or(1.0) <= 0.0
                && m.output_cost_per_million.unwrap_or(1.0) <= 0.0
        }),
        Err(_) => false,
    }
}

#[cfg(test)]
#[path = "policy_tests.rs"]
mod tests;
