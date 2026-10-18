//! Bind shared child-chat routing to the TUI.
use super::command::host::Native;
use crate::tui::app::state::App;
pub(crate) async fn route(app: &mut App, input: &str) -> bool {
    codetether_agent_command::chat::route(&mut Native { app, parent: None }, input).await
}
pub(crate) async fn send(app: &mut App, parent: &str, name: &str, message: &str) {
    codetether_agent_command::chat::send(
        &mut Native {
            app,
            parent: Some(parent),
        },
        parent,
        name,
        message,
    )
    .await;
}
