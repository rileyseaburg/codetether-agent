//! Resolve original provider/model before within-step failover.

use crate::provider::parse_model_string;
use crate::session::Session;

/// Resolve the original (pre-failover) provider and model from session metadata.
///
/// Returns `None` when the session has no stored model or when the current
/// selection already matches the original.
pub(crate) fn resolve_original(
    session: &Session,
    selected_provider: &str,
    model: &str,
) -> Option<(String, String)> {
    let model_str = session.metadata.model.as_ref()?;
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
