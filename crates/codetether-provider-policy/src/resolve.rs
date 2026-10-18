//! Explicit provider selection never silently falls back.
use crate::choose_default_provider;
use anyhow::Result;

/// Apply the established resolve provider for session request policy.
/// # Arguments
/// Receives the provider, request or error inputs indicated by the signature.
/// # Returns
/// The policy decision without invoking providers or mutating session state.
/// # Errors
/// Returns an error for missing explicit providers or an empty provider list.
/// # Examples
/// ```
/// assert!(codetether_provider_policy::resolve_provider_for_session_request(&["openai"], Some("missing")).is_err());
/// ```
pub fn resolve_provider_for_session_request<'a>(
    providers: &'a [&'a str],
    explicit_provider: Option<&str>,
) -> Result<&'a str> {
    if let Some(explicit) = explicit_provider {
        if let Some(found) = providers.iter().copied().find(|p| *p == explicit) {
            return Ok(found);
        }
        anyhow::bail!(
            "Provider '{}' selected explicitly but is unavailable. Available providers: {}",
            explicit,
            providers.join(", ")
        );
    }

    choose_default_provider(providers).ok_or_else(|| anyhow::anyhow!("No providers available"))
}
