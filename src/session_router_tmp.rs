fn is_retryable_upstream_error(err: &anyhow::Error) -> bool {
    let msg = err.to_string().to_ascii_lowercase();
    [
        " 500 ",
        " 504 ",
        "status code 500",
        "status code 504",
        "internal server error",
        "gateway timeout",
        "upstream request timeout",
        "server error: 500",
        "server error: 504",
    ]
    .iter()
    .any(|needle| msg.contains(needle))
}

fn known_good_router_candidates(provider: &str, failed_model: &str) -> Vec<String> {
    let failed = failed_model.trim();
    let mut candidates: Vec<String> = match provider {
        "openrouter" => vec![
            "openrouter/qwen/qwen3-coder:free".to_string(),
            "openrouter/openai/gpt-oss-120b:free".to_string(),
            "openrouter/google/gemma-3-27b-it:free".to_string(),
            "openrouter/meta-llama/llama-3.3-70b-instruct:free".to_string(),
        ],
        "zai" => vec!["zai/glm-5".to_string()],
        "glm5" => vec!["glm5/glm-5".to_string()],
        "github-copilot" | "github-copilot-enterprise" => {
            vec![format!("{provider}/gpt-5-mini")]
        }
        "openai-codex" => {
            vec!["openai-codex/gpt-5.5".to_string(), "openai-codex/gpt-5-mini".to_string()]
        }
        "gemini-web" => vec!["gemini-web/gemini-2.5-flash".to_string()],
        "local_cuda" => vec!["local_cuda/qwen3.5-9b".to_string()],
        "google" => vec!["google/gemini-2.5-flash".to_string()],
        "anthropic" => vec!["anthropic/claude-3-5-haiku-latest".to_string()],
        _ => Vec::new(),
    };

    candidates.retain(|candidate| {
        !candidate.eq_ignore_ascii_case(failed)
            && candidate
                .split('/')
                .next_back()
                .map(|model| !model.eq_ignore_ascii_case(failed))
                .unwrap_or(true)
    });
    candidates
}

fn choose_router_target(
    registry: &crate::provider::ProviderRegistry,
    selected_provider: &str,
    current_model: &str,
) -> Option<(String, String)> {
    let current_provider = selected_provider.to_ascii_lowercase();

    for candidate in known_good_router_candidates(selected_provider, current_model) {
        let (provider_name, model_name) = crate::provider::parse_model_string(&candidate);
        let provider_name = provider_name.unwrap_or(selected_provider);
        if registry.get(provider_name).is_some() {
            return Some((provider_name.to_string(), model_name.to_string()));
        }
    }

    if current_provider != "zai" && registry.get("zai").is_some() {
        return Some(("zai".to_string(), "glm-5".to_string()));
    }
    if current_provider != "glm5" && registry.get("glm5").is_some() {
        return Some(("glm5".to_string(), "glm-5".to_string()));
    }
    if current_provider != "openrouter" && registry.get("openrouter").is_some() {
        return Some(("openrouter".to_string(), "openai/gpt-oss-120b:free".to_string()));
    }

    None
}
