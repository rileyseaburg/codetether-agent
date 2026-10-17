//! Reserve a pending slot, evicting safe LRU children as needed.

use crate::Host;
use codetether_collaboration_state::residency::{Slot, usage};
use codetether_tool_core::ToolResult;

/// Reserve capacity for a spawn or reload, evicting safe residents if needed.
/// # Arguments
/// * `protected` — A resident identity that must not be selected for eviction.
/// # Returns
/// A pending slot; commit it after registration or drop it on failure.
/// # Errors
/// Returns a tool rejection if configuration fails or no capacity is available.
/// # Examples
/// ```rust,no_run
/// use codetether_agent_residency::{reserve, Host};
/// async fn example<H: Host>() {
///     if let Ok(slot) = reserve::<H>(None).await { drop(slot); }
/// }
/// ```
pub async fn reserve<H: Host>(protected: Option<&str>) -> Result<Slot, ToolResult> {
    let limit = crate::limit().await?;
    loop {
        if let Some(slot) = Slot::try_new(limit) {
            return Ok(slot);
        }
        if super::evict::one::<H>(protected).await {
            continue;
        }
        if let Some(slot) = Slot::try_new(limit) {
            return Ok(slot);
        }
        return Err(crate::rejected(usage(), limit));
    }
}
