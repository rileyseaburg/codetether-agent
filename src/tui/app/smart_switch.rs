use std::collections::HashSet;

use crate::tui::constants::{SMART_SWITCH_MAX_RETRIES, SMART_SWITCH_PROVIDER_PRIORITY};

/// Sentinel value meaning "follow the latest message position" (kept for compatibility)
pub const SCROLL_BOTTOM: usize = 1_000_000;

/// Normalizes provider aliases (zhipuai/z-ai -> zai)
pub fn normalize_provider_alias(provider_name: &str) -> &str {
    if provider_name.eq_ignore_ascii_case("zhipuai") || provider_name.eq_ignore_ascii_case("z-ai")
    {
        "zai"
    } else {
        provider_name
    }
}

/// Normalizes a model reference for use as a key in the attempted models set
pub fn smart_switch_model_key(model_ref: &str) -> String {
    model_ref.trim().to_ascii_lowercase()
}

/// Classifies provider errors as retryable (rate limit, timeout, 5xx, etc.) vs permanent
pub fn is_retryable_provider_error(err: &str) -> bool {
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
        "context length exceeded",
        "maximum context length",
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

/// Returns the ordered list of preferred fallback models for a given provider
pub fn smart_switch_preferred_models(provider_name: &str) -> &'static [&'static str] {
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

/// A pending smart switch retry with the prompt and target model
#[derive(Debug, Clone)]
pub struct PendingSmartSwitchRetry {
    pub prompt: String,
    pub target_model: String,
}

/// Returns the max number of smart switch retries (from env or default)
pub fn smart_switch_max_retries() -> u8 {
    std::env::var("CODETETHER_SMART_SWITCH_MAX_RETRIES")
        .ok()
        .and_then(|v| v.parse::<u8>().ok())
        .map(|v| v.clamp(1, 10))
        .unwrap_or(SMART_SWITCH_MAX_RETRIES)
}

/// Generates a list of candidate models for smart switching
pub fn smart_switch_candidates(
    current_model: Option<&str>,
    current_provider: Option<&str>,
    available_providers: &[String],
    attempted_models: &HashSet<String>,
) -> Vec<String> {
    let available: HashSet<String> = available_providers
        .iter()
        .map(|p| normalize_provider_alias(p).to_string())
        .collect();

    let current_provider_normalized = current_provider.map(normalize_provider_alias);

    let mut candidates = Vec::new();

    // First, try other models from the same provider
    if let Some(provider_name) = current_provider_normalized {
        if available.contains(provider_name) {
            for model_id in smart_switch_preferred_models(provider_name) {
                let candidate = format!("{provider_name}/{model_id}");
                if !attempted_models.contains(&smart_switch_model_key(&candidate)) {
                    candidates.push(candidate);
                }
            }
        }
    }

    // Then try providers in priority order
    for provider_name in SMART_SWITCH_PROVIDER_PRIORITY {
        let normalized_provider = normalize_provider_alias(provider_name);
        if Some(normalized_provider) == current_provider_normalized {
            continue;
        }
        if !available.contains(normalized_provider) {
            continue;
        }
        if let Some(model_id) = smart_switch_preferred_models(normalized_provider).first() {
            let candidate = format!("{normalized_provider}/{model_id}");
            if !attempted_models.contains(&smart_switch_model_key(&candidate)) {
                candidates.push(candidate);
            }
        }
    }

    // Finally, try any remaining providers
    let mut extra_providers: Vec<String> = available
        .into_iter()
        .filter(|provider_name| {
            if Some(provider_name.as_str()) == current_provider_normalized {
                return false;
            }
            !SMART_SWITCH_PROVIDER_PRIORITY
                .iter()
                .any(|priority| normalize_provider_alias(priority) == provider_name)
        })
        .collect();
    extra_providers.sort();

    for provider_name in extra_providers {
        if let Some(model_id) = smart_switch_preferred_models(&provider_name).first() {
            let candidate = format!("{provider_name}/{model_id}");
            if !attempted_models.contains(&smart_switch_model_key(&candidate)) {
                candidates.push(candidate);
            }
        }
    }

    candidates
}

/// Attempts to schedule a smart switch retry after a provider error.
///
/// Returns `Some(PendingSmartSwitchRetry)` if a retry was scheduled, `None` otherwise
/// (non-retryable error, max retries exceeded, or no fallback candidates available).
pub fn maybe_schedule_smart_switch_retry(
    error_msg: &str,
    current_model: Option<&str>,
    current_provider: Option<&str>,
    available_providers: &[String],
    prompt: &str,
    retry_count: u32,
    attempted_models: &[String],
) -> Option<PendingSmartSwitchRetry> {
    if !is_retryable_provider_error(error_msg) {
        return None;
    }

    let max = smart_switch_max_retries() as u32;
    if retry_count >= max {
        return None;
    }

    let attempted: HashSet<String> = attempted_models
        .iter()
        .map(|m| smart_switch_model_key(m))
        .collect();

    let candidates =
        smart_switch_candidates(current_model, current_provider, available_providers, &attempted);

    candidates.into_iter().next().map(|target_model| PendingSmartSwitchRetry {
        prompt: prompt.to_string(),
        target_model,
    });

/// Extracts the provider name from a model reference (e.g. "minimax/MiniMax-M2" -> "minimax").
/// Returns `None` if the reference doesn't contain a `/`.
pub fn extract_provider_from_model(model_ref: &str) -> Option<&str> {
    model_ref.split('/').next().filter(|p| !p.is_empty())
}

/// Returns `true` when the model has changed compared to the current session model,
/// indicating a smart switch should be executed.
pub fn should_execute_smart_switch(
    current_model: Option<&str>,
    pending: Option<&PendingSmartSwitchRetry>,
) -> bool {
    match (current_model, pending) {
        (Some(current), Some(retry)) => current != retry.target_model,
        (_, Some(_)) => true,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn smart_switch_marks_transient_provider_errors_retryable() {
        assert!(is_retryable_provider_error(
            "OpenAI API error (429 Too Many Requests): rate limit exceeded"
        ));
        assert!(is_retryable_provider_error(
            "Gemini returned protocol error code 469 with no text payload"
        ));
        assert!(is_retryable_provider_error(
            "Anthropic API error: unknown error, 520 (1000) (Some(\"api_error\"))"
        ));
        assert!(is_retryable_provider_error(
            "Anthropic API error: unknown error, 798 (1000) (Some(\"api_error\"))"
        ));
        assert!(is_retryable_provider_error(
            "context length exceeded, maximum context length is 128k"
        ));
    }

    #[test]
    fn smart_switch_ignores_non_retryable_bad_request_errors() {
        assert!(!is_retryable_provider_error(
            "OpenAI API error (400 Bad Request): Instructions are required"
        ));
        assert!(!is_retryable_provider_error(
            "OpenAI API error (401 Unauthorized): Invalid API key"
        ));
        assert!(!is_retryable_provider_error(
            "OpenAI API error (403 Forbidden): Access denied"
        ));
        assert!(!is_retryable_provider_error(
            "OpenAI API error (404 Not Found): Model not found"
        ));
        assert!(!is_retryable_provider_error(
            "OpenAI API error (422 Unprocessable Entity): Invalid request"
        ));
    }

    #[test]
    fn smart_switch_normalizes_provider_aliases() {
        assert_eq!(normalize_provider_alias("z-ai"), "zai");
        assert_eq!(normalize_provider_alias("ZhipuAI"), "zai");
        assert_eq!(normalize_provider_alias("openai"), "openai");
    }

    #[test]
    fn smart_switch_model_key_normalizes() {
        assert_eq!(
            smart_switch_model_key("  Minimax-Credits/MiniMax-M2.5-Highspeed "),
            "minimax-credits/minimax-m2.5-highspeed".to_string()
        );
    }

    #[test]
    fn smart_switch_preferred_models_returns_list_for_known_providers() {
        let models = smart_switch_preferred_models("minimax");
        assert!(!models.is_empty());
        assert!(models.contains(&"MiniMax-M2.5"));

        let models = smart_switch_preferred_models("zai");
        assert!(models.contains(&"glm-5"));
    }

    #[test]
    fn smart_switch_preferred_models_returns_empty_for_unknown_provider() {
        let models = smart_switch_preferred_models("unknown-provider");
        assert!(models.is_empty());
    }

    #[test]
    fn smart_switch_candidates_prioritizes_same_provider() {
        let available = vec!["minimax".to_string(), "openai".to_string()];
        let attempted = HashSet::new();
        let candidates = smart_switch_candidates(
            Some("minimax/MiniMax-M2"),
            Some("minimax"),
            &available,
            &attempted,
        );
        // First candidate should be from same provider
        assert!(candidates.first().unwrap().starts_with("minimax/"));
    }

    #[test]
    fn smart_switch_candidates_excludes_attempted_models() {
        let available = vec!["minimax".to_string(), "openai".to_string()];
        let mut attempted = HashSet::new();
        attempted.insert(smart_switch_model_key("minimax/MiniMax-M2.5"));

        let candidates = smart_switch_candidates(
            Some("minimax/MiniMax-M2.5"),
            Some("minimax"),
            &available,
            &attempted,
        );

        // Should not include the attempted model
        assert!(!candidates
            .iter()
            .any(|c| c == "minimax/MiniMax-M2.5"));
    }
}