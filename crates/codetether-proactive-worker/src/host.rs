//! Native storage operations required by proactive workers.
use anyhow::Result;
use async_trait::async_trait;
use codetether_prepared_context::Prepared;
use codetether_provider_core::Message;
use codetether_session_store::{Delivery, Ticket};
use serde_json::Value;
/// Storage binding used by both incremental and legacy worker paths.
/// # Implementors
/// Delegate to existing cursor, projection and prepared-sidecar operations.
#[async_trait]
pub trait Host: Send + Sync {
    /// Native session snapshot supplied by a cursor delivery.
    type Session: Send;
    /// Number of messages in a native delivery before it is consumed.
    fn message_count(session: &Self::Session) -> usize;
    /// Consume the native snapshot into its retained message window.
    fn into_messages(session: Self::Session) -> Vec<Message>;
    /// Read the next durable cursor delivery.
    async fn next(
        &self,
        id: &str,
        consumer: &str,
        limit: usize,
    ) -> Result<Option<Delivery<Self::Session>>>;
    /// Commit the cursor ticket with the materialized summary view.
    async fn commit(&self, ticket: Ticket, view: Value) -> Result<()>;
    /// Load the native tail using its configured window size.
    async fn load_tail(&self, id: &str) -> Result<Vec<Message>>;
    /// Read a validated prepared-index sidecar.
    async fn read_prepared(&self, id: &str) -> Option<Prepared>;
    /// Persist a prepared-index sidecar after native path validation.
    async fn write_prepared(&self, id: &str, value: &Prepared) -> Result<()>;
    /// Remove a prepared sidecar after a deletion race.
    async fn remove_prepared(&self, id: &str);
}
