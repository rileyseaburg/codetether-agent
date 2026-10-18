//! Native history and persistence operations for intentional interruption.
use crate::ControlHost;
use anyhow::Result;
use async_trait::async_trait;
use codetether_provider_core::Message;
/// Preserve normal session page classification when appending an interruption.
/// # Implementors
/// Applications delegate message append and save to their existing session APIs.
#[async_trait]
pub trait MarkerHost: ControlHost {
    /// Append the developer marker through the native session method.
    fn add_message(session: &mut Self::Session, message: Message);
    /// Persist the marker, propagating the native save error.
    async fn save(session: &Self::Session) -> Result<()>;
    /// Replace the live registry snapshot after a successful save.
    fn update_session(agent_id: &str, session: Self::Session);
}
