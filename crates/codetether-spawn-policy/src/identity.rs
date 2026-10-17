//! Owner-scoped child-name reservation around asynchronous spawn setup.

use crate::IdentityHost;
use codetether_tool_core::ToolResult;
use std::sync::{Arc, LazyLock};

#[path = "identity/registry.rs"]
mod registry;
#[path = "identity/reservation.rs"]
mod reservation;
pub use reservation::Reservation;

static REGISTRY: LazyLock<Arc<registry::Registry>> =
    LazyLock::new(|| Arc::new(registry::Registry::default()));

/// Reserve a name after checking both live and durable owner-scoped identity.
/// # Arguments
/// * `name` — Requested display name.
/// * `owner` — Exact owner scope; none denotes an unowned child.
/// # Returns
/// A pending claim released on drop or after a durable handoff via commit.
/// # Errors
/// Returns a tool error on duplicate live, durable or pending identity, or
/// when the durable identity lookup fails.
/// # Examples
/// ```rust,no_run
/// use codetether_spawn_policy::{reserve, IdentityHost};
/// async fn example<H: IdentityHost>() {
///     if let Ok(claim) = reserve::<H>("reviewer", Some("owner")).await { drop(claim); }
/// }
/// ```
pub async fn reserve<H: IdentityHost>(
    name: &str,
    owner: Option<&str>,
) -> Result<Reservation, ToolResult> {
    let reservation =
        Reservation::try_with(Arc::clone(&REGISTRY), name, owner).ok_or_else(|| duplicate(name))?;
    let persisted = H::persisted_exists(owner, name)
        .await
        .map_err(|error| ToolResult::error(error.to_string()))?;
    if H::contains_name(name, owner) || persisted {
        return Err(duplicate(name));
    }
    Ok(reservation)
}

fn duplicate(name: &str) -> ToolResult {
    ToolResult::error(format!("Agent @{name} exists. Resume or kill it first."))
}

#[cfg(test)]
#[path = "identity/concurrency_tests.rs"]
mod concurrency_tests;
