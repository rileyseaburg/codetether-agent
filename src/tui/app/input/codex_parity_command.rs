use std::{path::Path, sync::Arc};

use crate::provider::ProviderRegistry;
use crate::session::Session;
use crate::tui::app::state::App;

#[path = "access_mode_command.rs"]
mod access_mode_command;
#[path = "codex_parity_copy.rs"]
mod copy;
#[path = "codex_parity_diff.rs"]
mod diff;
#[path = "codex_parity_host.rs"]
mod host;
#[path = "codex_parity_review_host.rs"]
mod review_host;
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
    codetether_slash_command::utility::run(
        &mut host::Native {
            app,
            cwd,
            session,
            registry,
        },
        prompt,
    )
    .await
}
