//! Select the first available provider using the existing MCP fallback order.
use super::ProviderHost;
use std::sync::Arc;
pub(super) async fn registry<H: ProviderHost>() -> Arc<H::Tools> {
    match H::providers().await {
        Ok(provider_registry) => {
            let fallbacks = [
                "zai",
                "openai",
                "github-copilot",
                "anthropic",
                "openrouter",
                "novita",
                "moonshotai",
                "google",
            ];
            let provider_and_model = fallbacks
                .iter()
                .find_map(|name| H::get(&provider_registry, name).map(|p| (p, name)));
            match provider_and_model {
                Some((provider, name)) => {
                    let model = match *name {
                        "zai" => "glm-5".to_string(),
                        "anthropic" => "claude-sonnet-4-20250514".to_string(),
                        "openai" => "gpt-4.1".to_string(),
                        _ => "default".to_string(),
                    };
                    tracing::info!(provider = %name, model = %model, "MCP server loaded provider from Vault");
                    H::provider_tools(provider, model)
                }
                None => {
                    tracing::warn!(
                        "Vault connected but no known provider found, Ralph/Go will be limited"
                    );
                    H::default_tools()
                }
            }
        }
        Err(e) => {
            tracing::warn!(error = %e, "Failed to load providers from Vault, Ralph/Go will be limited");
            H::default_tools()
        }
    }
}
