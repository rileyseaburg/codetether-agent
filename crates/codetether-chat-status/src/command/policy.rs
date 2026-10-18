//! Workspace policy refresh for status reports.
use codetether_config::{Config, TrustPolicyStatus};
use std::path::Path;
pub(super) async fn policy_line(cwd: &Path) -> String {
    match Config::load_for_workspace(cwd).await {
        Ok(config) => {
            let status = TrustPolicyStatus::from_config(&config);
            codetether_trust_status::set_status(status);
            codetether_trust_status::format_status(&status)
        }
        Err(error) => format!("Policy: unavailable ({error})"),
    }
}
