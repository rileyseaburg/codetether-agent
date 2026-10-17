//! Bind shared spawn admission to existing durable and live identities.
use crate::tool::agent::{persistence, store};
use anyhow::Result;
use async_trait::async_trait;
use codetether_spawn_policy::IdentityHost;
/// Application identity stores used by spawn admission.
pub struct ApplicationHost;
#[async_trait]
impl IdentityHost for ApplicationHost {
    async fn persisted_exists(owner: Option<&str>, name: &str) -> Result<bool> {
        persistence::exists(owner, name).await
    }
    fn contains_name(name: &str, owner: Option<&str>) -> bool {
        store::contains_name_for_owner(name, owner)
    }
}
