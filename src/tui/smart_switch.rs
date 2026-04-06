//! Smart provider switching and retry logic

use super::*;

fn is_glm5_model(model: &str) -> bool {
    let normalized = model.trim().to_ascii_lowercase();
    matches!(
        normalized.as_str(),
        "zai/glm-5"
            | "z-ai/glm-5"
            | "openrouter/z-ai/glm-5"
            | "glm5/glm-5-fp8"
            | "glm5/glm-5"
            | "glm5:glm-5-fp8"
            | "glm5:glm-5"
    )
}

fn is_minimax_m25_model(model: &str) -> bool {
    let normalized = model.trim().to_ascii_lowercase();
    matches!(
        normalized.as_str(),
        "minimax/minimax-m2.5"
            | "minimax-m2.5"
            | "minimax-credits/minimax-m2.5-highspeed"
            | "minimax-m2.5-highspeed"
    )
}

fn next_go_model(current_model: Option<&str>) -> String {
    match current_model {
        Some(model) if is_glm5_model(model) => GO_SWAP_MODEL_MINIMAX.to_string(),
        Some(model) if is_minimax_m25_model(model) => GO_SWAP_MODEL_GLM.to_string(),
        _ => GO_SWAP_MODEL_MINIMAX.to_string(),
    }
}

fn normalize_provider_alias(provider_name: &str) -> &str {
    if provider_name.eq_ignore_ascii_case("zhipuai") || provider_name.eq_ignore_ascii_case("z-ai") {
        "zai"
    } else {
        provider_name
    }
}

fn smart_switch_model_key(model_ref: &str) -> String {
    model_ref.trim().to_ascii_lowercase()
}

fn is_retryable_provider_error(err: &str) -> bool {
    let normalized = err.to_ascii_lowercase();
    let transient_status_codes = [
        " 429 ", " 500 ", " 502 ", " 503 ", " 504 ", " 520 ", " 521 ", " 522 ", " 523 ", " 524 ",
        " 525 ", " 526 ", " 529 ", " 598 ", " 599 ", " 798 ",
    ];
    let non_retryable_status_codes = [" 400 ", " 401 ", " 403 ", " 404 ", " 422 "];
    let markers = [
        "429",
        "rate limit",
        "too many requests",
        "quota exceeded",
        "service unavailable",
        "temporarily unavailable",
        "bad gateway",
        "gateway timeout",
        "timed out",
        "timeout",
        "connection reset",
        "connection refused",
        "network error",
        "unknown api error",
        "api_error",
        "unknown error",
        "protocol error code 469",
        "no text payload",
    ];

    // Check for non-retryable client errors first (4xx except 429)
    if non_retryable_status_codes
        .iter()
        .any(|code| normalized.contains(code))
    {
        return false;
    }

    markers.iter().any(|marker| normalized.contains(marker))
        || transient_status_codes
            .iter()
            .any(|code| normalized.contains(code))
}

fn smart_switch_preferred_models(provider_name: &str) -> &'static [&'static str] {
    match provider_name {
        "minimax" => &["MiniMax-M2.5", "MiniMax-M2.1", "MiniMax-M2"],
        "minimax-credits" => &["MiniMax-M2.5-highspeed", "MiniMax-M2.1-highspeed"],
        "zai" => &["glm-5", "glm-4.7", "glm-4.7-flash"],
        "openai-codex" => &["gpt-5-mini", "gpt-5", "gpt-5.1-codex"],
        "openrouter" => &["z-ai/glm-5:free", "z-ai/glm-5", "moonshotai/kimi-k2:free"],
        "github-copilot" | "github-copilot-enterprise" => &["gpt-5-mini", "gpt-4.1", "gpt-4o"],
        "openai" => &["gpt-4o-mini", "gpt-4.1"],
        "anthropic" => &["claude-sonnet-4-20250514", "claude-3-5-sonnet-20241022"],
        "google" => &["gemini-2.5-flash", "gemini-2.5-pro"],
        "gemini-web" => &["gemini-web-pro"],
        _ => &[],
    }
}

fn normalize_local_model_ref(model: &str) -> String {
    let trimmed = model.trim();
    if trimmed.is_empty() {
        return AUTOCHAT_LOCAL_DEFAULT_MODEL.to_string();
    }

    let (provider_name, _) = crate::provider::parse_model_string(trimmed);
    if provider_name.is_some() {
        trimmed.to_string()
    } else {
        format!("local_cuda/{trimmed}")
    }
}

fn resolve_local_loop_model(current_model: Option<&str>) -> String {
    if let Ok(explicit) = std::env::var("CODETETHER_TUI_LOCAL_MODEL")
        .or_else(|_| std::env::var("CODETETHER_LOCAL_CUDA_MODEL"))
        .or_else(|_| std::env::var("LOCAL_CUDA_MODEL"))
    {
        let normalized = normalize_local_model_ref(&explicit);
        if !normalized.trim().is_empty() {
            return normalized;
        }
    }

    if let Some(model) = current_model {
        let normalized = normalize_local_model_ref(model);
        if normalized.starts_with("local_cuda/") {
            return normalized;
        }
    }

    AUTOCHAT_LOCAL_DEFAULT_MODEL.to_string()
}

