//! Load run configuration and publish its effective access mode.
use crate::effective_access_mode;
use codetether_cli_options::RunArgs;
use codetether_config::Config;

/// Load run configuration and apply CLI access policy in process order.
/// # Arguments
/// * `args` - CLI access mode and yolo options.
/// # Returns
/// Loaded or default configuration with effective access mode published.
/// # Examples
/// ```no_run
/// # async fn example(args: &codetether_cli_options::RunArgs) {
/// let config = codetether_run_policy::load(args).await;
/// # }
/// ```
pub async fn load(args: &RunArgs) -> Config {
    let mut config = Config::load().await.unwrap_or_default();
    let access_mode = effective_access_mode(args);
    Config::apply_process_access_mode_override(access_mode);
    config.apply_access_mode_override(access_mode);
    codetether_runtime_policy::justification_schema::note_access_mode(
        config.effective_access_mode(),
    );
    config
}
