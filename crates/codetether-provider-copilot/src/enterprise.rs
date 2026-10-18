//! Enterprise endpoint normalization.
/// Normalize an enterprise domain or URL.
///
/// # Arguments
/// * `input` - Domain, optionally with an HTTP scheme and trailing slash.
///
/// # Returns
/// The trimmed domain.
///
/// # Examples
/// ```
/// assert_eq!(codetether_provider_copilot::normalize_enterprise_domain("https://example.com/"), "example.com");
/// ```
pub fn normalize_enterprise_domain(input: &str) -> String {
    input
        .trim()
        .trim_start_matches("https://")
        .trim_start_matches("http://")
        .trim_end_matches('/')
        .to_string()
}
/// Build the Copilot API URL for an enterprise.
///
/// # Arguments
/// * `enterprise_url` - Enterprise domain or URL.
///
/// # Returns
/// An HTTPS URL with the Copilot API subdomain.
///
/// # Examples
/// ```
/// assert_eq!(codetether_provider_copilot::enterprise_base_url("example.com"), "https://copilot-api.example.com");
/// ```
pub fn enterprise_base_url(enterprise_url: &str) -> String {
    format!(
        "https://copilot-api.{}",
        normalize_enterprise_domain(enterprise_url)
    )
}
