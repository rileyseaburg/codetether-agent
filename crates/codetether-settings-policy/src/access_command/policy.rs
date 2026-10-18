//! Reload and publish the current trust-policy summary.
use std::path::Path;

use codetether_config::{Config, TrustPolicyStatus};

pub(super) async fn summary(cwd: &Path) -> String {
    match Config::load_for_workspace(cwd).await {
        Ok(config) => format_and_store(&config),
        Err(error) => format!("Policy: unavailable ({error})"),
    }
}

fn format_and_store(config: &Config) -> String {
    let status = TrustPolicyStatus::from_config(config);
    codetether_trust_status::set_status(status);
    codetether_trust_status::format_status(&status)
}
