//! Session construction, persistence, bus and parent-policy host operations.
use crate::SessionAccess;
use anyhow::Result;
use async_trait::async_trait;
/// Existing session operations needed by managed-child lifecycle rules.
/// # Implementors
/// Applications implement this for their native, durable session type.
#[async_trait]
pub trait ChildSession: codetether_agent_registry::Session + SessionAccess + Send + Sync {
    /// Construct a fresh native session, returning its normal initialization error.
    async fn create() -> Result<Self>
    where
        Self: Sized;
    /// Apply the native agent-name setter.
    fn set_agent_name(&mut self, name: String);
    /// Apply the native title setter.
    fn set_title(&mut self, title: String);
    /// Attach the existing process-global bus.
    fn attach_bus(&mut self);
    /// Update the modification timestamp and save; propagate the native save error.
    async fn save_updated(&mut self) -> Result<()>;
    /// Load the parent's last 16 messages and return its effective access policy.
    /// Propagate the native load error so the caller can fail closed and log it.
    async fn load_parent_policy(parent_id: &str) -> Result<bool>;
}
