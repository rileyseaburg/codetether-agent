//! Application identity lookups used by atomic spawn admission.
use anyhow::Result;
use async_trait::async_trait;
/// Existing live and durable identities in one owner scope.
/// # Implementors
/// Applications delegate these methods to their canonical stores.
#[async_trait]
pub trait IdentityHost: Send + Sync {
    /// Check durable identity existence; propagate persistence errors unchanged.
    async fn persisted_exists(owner: Option<&str>, name: &str) -> Result<bool>;
    /// Check a name in the exact live owner scope, including absent owners.
    fn contains_name(name: &str, owner: Option<&str>) -> bool;
}
