use std::path::Path;

use crate::config::{Config, TrustPolicyStatus};
use crate::tui::app::state::App;

pub(super) async fn summary(cwd: &Path) -> String {
    match Config::load_for_workspace(cwd).await {
        Ok(config) => format_and_store(&config),
        Err(error) => format!("Policy: unavailable ({error})"),
    }
}

pub(super) fn release_active(app: &mut App) -> bool {
    let live = crate::approval::live::latest_id();
    let queued = crate::tui::app::state::approval_queue::active_id();
    (live.is_some() || queued.is_some())
        && crate::tui::app::input::approval_command::run(app, "/approve")
}

fn format_and_store(config: &Config) -> String {
    let status = TrustPolicyStatus::from_config(config);
    crate::tui::ui::trust_status::set_status(status);
    crate::tui::ui::trust_status::format_status(&status)
}
