//! Bind calling-session selection to the application's durable session store.
use crate::session::Session;
#[cfg(test)]
use anyhow::Result;
#[cfg(test)]
use serde_json::Value;
codetether_session_schema::impl_session_context_tools!(
    Session,
    crate::session::SessionBus,
    crate::session::helper::rlm_background::Background::resolve_model
);
/// Load the exact calling session, falling back to the latest only without an ID.
#[cfg(test)]
pub async fn load_calling_session(args: &Value) -> Result<Option<Session>> {
    codetether_context_loader::load_calling_session::<Session>(args).await
}
#[cfg(test)]
#[path = "context_helpers_tests.rs"]
mod tests;
