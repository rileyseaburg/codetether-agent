//! Environment capability parsing and stable deduplication.

pub(super) fn is_knative() -> bool {
    std::env::var("KNATIVE_SERVICE")
        .map(|value| {
            matches!(
                value.trim().to_ascii_lowercase().as_str(),
                "1" | "true" | "yes"
            )
        })
        .unwrap_or(false)
}

pub(super) fn append_env_capabilities(capabilities: &mut Vec<String>, env_var: &str) {
    let Ok(raw) = std::env::var(env_var) else {
        return;
    };
    for capability in raw.split(|ch: char| ch == ',' || ch.is_whitespace()) {
        push_capability(capabilities, capability);
    }
}

pub(super) fn push_capability(capabilities: &mut Vec<String>, capability: &str) {
    let capability = capability.trim();
    if capability.is_empty() || capabilities.iter().any(|existing| existing == capability) {
        return;
    }
    capabilities.push(capability.to_string());
}
