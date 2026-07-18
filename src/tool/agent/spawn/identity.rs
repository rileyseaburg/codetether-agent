//! Owner-scoped child-name reservation around asynchronous spawn setup.

use crate::tool::ToolResult;
use std::sync::{Arc, LazyLock};

#[path = "identity/registry.rs"]
mod registry;
#[path = "identity/reservation.rs"]
mod reservation;
pub(super) use reservation::Reservation;

static REGISTRY: LazyLock<Arc<registry::Registry>> =
    LazyLock::new(|| Arc::new(registry::Registry::default()));

/// Reserve a name after checking both live and durable owner-scoped identity.
pub(super) async fn reserve(name: &str, owner: Option<&str>) -> Result<Reservation, ToolResult> {
    let reservation =
        Reservation::try_with(Arc::clone(&REGISTRY), name, owner).ok_or_else(|| duplicate(name))?;
    let persisted = super::super::persistence::exists(owner, name)
        .await
        .map_err(|error| ToolResult::error(error.to_string()))?;
    if super::super::store::contains_name_for_owner(name, owner) || persisted {
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
#[cfg(test)]
#[path = "identity/scope_tests.rs"]
mod scope_tests;
