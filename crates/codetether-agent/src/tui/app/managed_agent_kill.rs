//! Bind child removal to the active parent session.
use crate::{session::Session, tui::app::state::App};
pub(crate) async fn kill(app: &mut App, session: &Session, rest: &str) {
    codetether_agent_command::kill(
        &mut super::super::host::Native {
            app,
            parent: Some(&session.id),
        },
        rest,
    )
    .await;
}
