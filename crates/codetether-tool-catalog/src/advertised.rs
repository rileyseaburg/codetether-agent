//! Select provider-facing schemas and retain discovery when the set is compact.
use crate::list_tools_bootstrap_definition;
use codetether_provider_core::ToolDefinition;

/// Select the provider's tool profile, apply policy, and add catalog discovery.
/// # Arguments
/// `model_supports_tools` enables native schemas; `tools` is the full active
/// catalog; `provider` selects the profile; `apply_policy` adds policy requirements.
/// # Returns
/// Advertised schemas, including discovery when fewer schemas remain than tools.
/// Providers without native tool support receive an empty list without policy calls.
/// # Examples
/// ```
/// let schemas = codetether_tool_catalog::advertised_tools(false, &[], "local_cuda", |v| v);
/// assert!(schemas.is_empty());
/// ```
pub fn advertised_tools(
    model_supports_tools: bool,
    tools: &[ToolDefinition],
    provider: &str,
    apply_policy: impl FnOnce(Vec<ToolDefinition>) -> Vec<ToolDefinition>,
) -> Vec<ToolDefinition> {
    if !model_supports_tools {
        return Vec::new();
    }
    let advertised = codetether_tool_core::profile::apply_for_provider(tools.to_vec(), provider);
    let mut advertised = apply_policy(advertised);
    if advertised.len() < tools.len() {
        advertised.push(list_tools_bootstrap_definition());
    }
    advertised
}
