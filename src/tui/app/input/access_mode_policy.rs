use std::path::Path;

use crate::config::{Config, TrustPolicyStatus};

pub(super) async fn summary(cwd: &Path) -> String {
    match Config::load_for_workspace(cwd).await {
        Ok(config) => format_and_store(&config),
        Err(error) => format!("Policy: unavailable ({error})"),
    }
}

fn format_and_store(config: &Config) -> String {
    let status = TrustPolicyStatus::from_config(config);
    crate::tui::ui::trust_status::set_status(status);
    crate::tui::ui::trust_status::format_status(&status)
}
