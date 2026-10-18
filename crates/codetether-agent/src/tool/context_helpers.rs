//! Bind calling-session selection to the application's durable session store.
use crate::session::Session;
use anyhow::Result;
use serde_json::Value;
codetether_session_schema::impl_session_context_tools!(
    Session,
    crate::session::SessionBus,
    crate::session::helper::rlm_background::Background::resolve_model
);
/// Load the exact calling session, falling back to the latest only without an ID.
pub async fn load_calling_session(args: &Value) -> Result<Option<Session>> {
    codetether_context_loader::load_calling_session::<Session>(args).await
}
/// Load a bounded window of the exact calling session.
pub(super) async fn load_calling_window(args: &Value, window: usize) -> Result<Option<Session>> {
    codetether_context_loader::load_calling_window::<Session>(args, window).await
}
/// Load the latest session for the current directory, without caller identity.
pub async fn load_latest_session() -> Result<Option<Session>> {
    codetether_context_loader::load_latest_session::<Session>().await
}
#[cfg(test)]
#[path = "context_helpers_tests.rs"]
mod tests;
