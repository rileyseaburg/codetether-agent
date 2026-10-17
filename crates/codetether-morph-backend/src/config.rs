//! Opt-in flag, model selection and direct endpoint normalization.

pub(super) fn backend_enabled() -> bool {
    match std::env::var("CODETETHER_MORPH_TOOL_BACKEND") {
        Ok(v) => matches!(
            v.trim().to_ascii_lowercase().as_str(),
            "1" | "true" | "yes" | "on"
        ),
        Err(_) => false,
    }
}

pub(super) fn morph_model_ref() -> String {
    let configured = std::env::var("CODETETHER_MORPH_TOOL_MODEL")
        .ok()
        .filter(|v| !v.trim().is_empty())
        .unwrap_or_else(|| "openrouter/morph/morph-v3-large".to_string());
    let trimmed = configured.trim();
    if trimmed.starts_with("openrouter/") {
        return trimmed.to_string();
    }
    if trimmed.starts_with("morph/") {
        return format!("openrouter/{trimmed}");
    }
    trimmed.to_string()
}

pub(super) fn direct_openrouter_base_url() -> Option<String> {
    std::env::var("CODETETHER_OPENROUTER_BASE_URL")
        .ok()
        .filter(|v| !v.trim().is_empty())
        .map(|v| v.trim().trim_end_matches('/').to_string())
}
