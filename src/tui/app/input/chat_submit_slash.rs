//! Slash-command and `!shell` prefix dispatch for chat submit.

mod host;
use std::path::Path;
use std::sync::Arc;

use crate::provider::ProviderRegistry;
use crate::tui::app::session_runtime::SessionSlot;
use crate::tui::app::state::App;

#[path = "mux_command/mod.rs"]
mod mux_command;
#[path = "shell/mod.rs"]
mod shell;
#[path = "slash_no_session.rs"]
mod slash_no_session;
#[cfg(test)]
#[path = "slash_no_session_tests.rs"]
mod slash_no_session_tests;

pub(super) async fn run(
    app: &mut App,
    cwd: &Path,
    slot: &mut SessionSlot,
    registry: &Option<Arc<ProviderRegistry>>,
    prompt: &str,
) -> bool {
    codetether_slash_command::intake::run(
        &mut host::Native {
            app,
            cwd,
            slot,
            registry,
        },
        prompt,
    )
    .await
}
