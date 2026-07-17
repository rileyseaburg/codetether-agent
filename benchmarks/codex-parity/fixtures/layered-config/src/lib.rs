//! Layered application configuration.

/// Values supplied by one configuration layer.
///
/// # Examples
///
/// ```
/// let config = layered_config::Config {
///     timeout: Some(30), retries: Some(2), labels: vec!["api".into()],
/// };
/// assert_eq!(config.timeout, Some(30));
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    /// Request timeout in seconds.
    pub timeout: Option<u64>,
    /// Retry count.
    pub retries: Option<u8>,
    /// Ordered labels applied to the process.
    pub labels: Vec<String>,
}

/// Merge `override_config` over `base`.
///
/// # Arguments
///
/// * `base` - Values inherited when an override is absent.
/// * `override_config` - Higher-priority values and labels.
///
/// # Returns
///
/// The combined configuration.
///
/// # Examples
///
/// ```
/// use layered_config::{Config, merge};
/// let base = Config { timeout: Some(30), retries: None, labels: vec![] };
/// let overrides = Config { timeout: Some(5), retries: None, labels: vec![] };
/// assert_eq!(merge(base, overrides).timeout, Some(5));
/// ```
pub fn merge(base: Config, override_config: Config) -> Config {
    let mut labels = base.labels;
    labels.extend(override_config.labels);
    Config {
        timeout: override_config.timeout,
        retries: override_config.retries,
        labels,
    }
}
