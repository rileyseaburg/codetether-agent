//! Native bindings for shared access-mode command policy.
use crate::{provider::ProviderRegistry, session::Session, tui::app::state::App};
use std::{path::Path, sync::Arc};
#[path = "access_mode_host.rs"]
mod host;
#[path = "verifier_model_command.rs"]
mod verifier_model;
pub(super) async fn run(
    app: &mut App,
    cwd: &Path,
    session: &mut Session,
    registry: Option<&Arc<ProviderRegistry>>,
    prompt: &str,
) -> bool {
    codetether_settings_policy::access_command::run(
        &mut host::Native {
            app,
            session,
            registry,
        },
        cwd,
        prompt,
    )
    .await
}
