//! Reserve a pending slot, evicting safe LRU children as needed.

use super::slot::Slot;
use crate::tool::ToolResult;

/// Reserve capacity for a spawn or reload, evicting safe residents if needed.
pub(in crate::tool::agent) async fn reserve(protected: Option<&str>) -> Result<Slot, ToolResult> {
    let limit = super::super::spawn_validation::spawn_threads::limit().await?;
    loop {
        if let Some(slot) = Slot::try_new(limit) {
            return Ok(slot);
        }
        if super::evict::one(protected).await {
            continue;
        }
        if let Some(slot) = Slot::try_new(limit) {
            return Ok(slot);
        }
        return Err(super::super::spawn_validation::spawn_threads::rejected(
            super::order::usage(),
            limit,
        ));
    }
}
