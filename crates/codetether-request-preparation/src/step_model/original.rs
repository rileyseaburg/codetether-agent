//! Resolve original provider/model identity without native session access.
use codetether_provider_core::routing::parse_model_string;
pub(super) fn resolve_original(
    model_str: Option<&str>,
    selected_provider: &str,
    model: &str,
) -> Option<(String, String)> {
    let model_str = model_str?;
    let (prov, mdl) = parse_model_string(model_str);
    let orig_provider = prov
        .map(|p| match p {
            "zhipuai" | "z-ai" => "zai",
            o => o,
        })
        .map(str::to_string)
        .unwrap_or_else(|| selected_provider.to_string());
    let orig_model = mdl.to_string();
    if selected_provider == orig_provider && model == orig_model {
        return None;
    }
    Some((orig_provider, orig_model))
}
