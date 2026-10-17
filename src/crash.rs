//! Application-version and memory-guard adapter for shared crash reporting.
use crate::config::Config;
pub use codetether_crash::maybe_prompt_for_consent;
/// Initialize crash reporting with the application's version and memory guards.
///
/// # Arguments
/// * `config` - Current application consent and reporting settings.
/// # Returns
/// Completes after initialization and optional pending-report uploads.
/// # Examples
/// ```no_run
/// # async fn example(config: &codetether_agent::config::Config) {
/// codetether_agent::crash::initialize(config).await;
/// # }
/// ```
pub async fn initialize(config: &Config) {
    codetether_crash::initialize(
        config,
        env!("CARGO_PKG_VERSION"),
        crate::telemetry::install_memory_guards,
    )
    .await;
}
