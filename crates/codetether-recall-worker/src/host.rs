//! Native cursor and listing capabilities used by recall workers.
use anyhow::Result;
use async_trait::async_trait;
use codetether_session_store::{Delivery, Ticket};
use serde_json::Value;
use std::path::Path;
/// Extend the recall query host with worker-only operations.
/// # Implementors
/// Native session storage supplies delivery snapshots and atomic cursor commits.
#[async_trait]
pub trait Host: codetether_recall_query::Host {
    /// List session IDs for a workspace in the native listing order.
    /// # Errors
    /// Propagates native session-listing failures.
    async fn list_sessions(&self, workspace: &Path) -> Result<Vec<String>>;
    /// Load the next bounded delivery for the requested consumer.
    /// # Errors
    /// Propagates native cursor and snapshot errors.
    async fn next(
        &self,
        id: &str,
        consumer: &str,
        limit: usize,
    ) -> Result<Option<Delivery<Self::Session>>>;
    /// Commit a materialized projection and its cursor acknowledgement together.
    /// # Errors
    /// Propagates native projection-store errors.
    async fn commit(&self, ticket: Ticket, view: Value) -> Result<bool>;
}
