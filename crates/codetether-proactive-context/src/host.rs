//! Prepared projection lookup layered over the existing worker storage host.
use anyhow::Result;
use async_trait::async_trait;
use serde_json::Value;
/// Native projection storage in addition to worker cursor and sidecar operations.
/// # Implementors
/// Use the existing native worker host and projection reader.
#[async_trait]
pub trait Host: codetether_proactive_worker::Host {
    /// Read one materialized projection by session and consumer name.
    /// # Errors
    /// Propagates native path and projection-store failures.
    async fn projection(&self, id: &str, name: &str) -> Result<Option<Value>>;
}
