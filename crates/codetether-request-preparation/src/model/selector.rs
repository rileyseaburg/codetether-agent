//! Parse stored model selectors with provider aliases and provider-only names.
use codetether_provider_core::routing::parse_model_string;
pub(super) fn parse(model: Option<&str>, providers: &[&str]) -> (Option<String>, String) {
    let Some(value) = model else {
        return (None, String::new());
    };
    let (provider, model) = parse_model_string(value);
    let provider = provider.map(|name| match name {
        "zhipuai" | "z-ai" => "zai",
        other => other,
    });
    if let Some(provider) = provider {
        (Some(provider.to_string()), model.to_string())
    } else if providers.contains(&model) {
        (Some(model.to_string()), String::new())
    } else {
        (None, model.to_string())
    }
}
