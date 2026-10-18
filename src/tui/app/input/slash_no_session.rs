//! Native bindings for commands available while the session is checked out.
use crate::{provider::ProviderRegistry, tui::app::state::App};
use std::{path::Path, sync::Arc};
#[path = "slash_no_session_helpers.rs"]
mod helpers;
#[path = "slash_no_session_host.rs"]
mod host;
#[path = "slash_no_session_image.rs"]
mod image;
/// Dispatch a UI-only command without accessing the current session.
pub(super) fn run(
    app: &mut App,
    cwd: &Path,
    registry: &Option<Arc<ProviderRegistry>>,
    prompt: &str,
) -> bool {
    codetether_slash_command::session_free::run(&mut host::Native { app, cwd, registry }, prompt)
}
