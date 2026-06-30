use std::{path::Path, sync::Arc};

use crate::provider::ProviderRegistry;
use crate::session::Session;
use crate::tui::app::commands::handle_slash_command;
use crate::tui::app::state::App;

#[path = "access_mode_command.rs"]
mod access_mode_command;
#[path = "codex_parity_copy.rs"]
mod copy;
#[path = "codex_parity_diff.rs"]
mod diff;
#[path = "codex_parity_review.rs"]
mod review;
#[path = "codex_parity_status.rs"]
mod status;
#[cfg(test)]
#[path = "codex_parity_command_tests.rs"]
mod tests;

pub(super) async fn run(
    app: &mut App,
    cwd: &Path,
    session: &mut Session,
    registry: Option<&Arc<ProviderRegistry>>,
    prompt: &str,
) -> bool {
    if access_mode_command::run(app, cwd, session, registry, prompt).await {
        return true;
    }
    match prompt.split_whitespace().next().unwrap_or("") {
        "/status" => status::show(app, cwd, session, "Status").await,
        "/permissions" => status::show(app, cwd, session, "Permissions").await,
        "/diff" => diff::show(app, cwd),
        "/copy" => copy::latest(app),
        "/review" => review::prepare(app),
        "/clear" => reroute(app, cwd, session, registry, "/new").await,
        "/resume" => reroute(app, cwd, session, registry, "/sessions").await,
        _ => return false,
    }
    true
}

async fn reroute(
    app: &mut App,
    cwd: &Path,
    session: &mut Session,
    registry: Option<&Arc<ProviderRegistry>>,
    command: &str,
) {
    handle_slash_command(app, cwd, session, registry, command).await;
    app.state.clear_input();
}
