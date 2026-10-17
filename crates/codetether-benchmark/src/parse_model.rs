//! Parse the benchmark provider:model notation.

/// Parse model string into (provider, model) tuple
///
/// # Arguments
/// * `model_str` - Benchmark provider:model notation.
/// # Returns
/// Owned provider and model identifiers.
/// # Errors
/// Returns an error for invalid input or unavailable benchmark files.
/// # Examples
/// ```
/// assert_eq!(codetether_benchmark::parse_model("provider:model").unwrap(), ("provider".to_string(), "model".to_string()));
/// ```
pub fn parse_model(model_str: &str) -> anyhow::Result<(String, String)> {
    let parts: Vec<&str> = model_str.splitn(2, ':').collect();
    if parts.len() != 2 {
        anyhow::bail!(
            "Invalid model format '{}'. Expected 'provider:model'",
            model_str
        );
    }
    Ok((parts[0].to_string(), parts[1].to_string()))
}
