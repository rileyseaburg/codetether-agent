//! Environment-driven preferred failover references.

pub(super) fn preferred_failover_model_refs() -> Vec<String> {
    std::env::var("CODETETHER_PREFERRED_MODELS")
        .or_else(|_| std::env::var("CODETETHER_FAILOVER_MODELS"))
        .ok()
        .map(|raw| parse_model_ref_list(&raw))
        .unwrap_or_default()
}

fn parse_model_ref_list(raw: &str) -> Vec<String> {
    raw.split([',', ';', '\n'])
        .map(str::trim)
        .filter(|entry| !entry.is_empty() && entry.contains('/'))
        .map(ToString::to_string)
        .collect()
}
