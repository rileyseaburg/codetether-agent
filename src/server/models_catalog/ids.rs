//! Model identifier helpers.

pub(crate) fn canonical_provider(provider: &str) -> &str {
    match provider {
        "zhipuai" => "zai",
        other => other,
    }
}

pub(crate) fn model_id(provider: &str, model_id: &str) -> String {
    let canonical = canonical_provider(provider);
    let trimmed = model_id.trim_start_matches('/');
    if trimmed.starts_with(&format!("{canonical}/")) {
        trimmed.to_string()
    } else if provider != canonical && trimmed.starts_with(&format!("{provider}/")) {
        format!("{canonical}/{}", &trimmed[provider.len() + 1..])
    } else {
        format!("{canonical}/{trimmed}")
    }
}

pub(crate) fn display_name(provider: &str, name: &str) -> String {
    let provider = canonical_provider(provider);
    format!("{}: {name}", provider.to_uppercase())
}
