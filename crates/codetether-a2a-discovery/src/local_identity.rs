//! Process-local identity for the active A2A LAN endpoint.
//!
//! Background peer startup records the exact advertised name here so model
//! prompts and collaboration tools can distinguish this process from peers.

use std::sync::{LazyLock, RwLock};

static NAME: LazyLock<RwLock<Option<String>>> = LazyLock::new(|| RwLock::new(None));

/// Record the exact LAN peer name advertised by this process.
///
/// # Arguments
/// * `name` - Exact name advertised for the local peer.
/// # Returns
/// Returns unit after replacing the active name.
/// # Examples
/// ```rust
/// use codetether_a2a_discovery::local_identity::{activate, current_name};
/// activate("local-peer");
/// assert_eq!(current_name().as_deref(), Some("local-peer"));
/// ```
pub fn activate(name: &str) {
    *NAME
        .write()
        .unwrap_or_else(std::sync::PoisonError::into_inner) = Some(name.to_string());
}

/// Return the active local LAN peer name, when an endpoint is running.
///
/// # Arguments
/// Takes no arguments.
/// # Returns
/// A snapshot of the active name, or `None` when no peer is active.
/// # Examples
/// ```rust
/// use codetether_a2a_discovery::local_identity::{activate, current_name};
/// activate("local-peer");
/// assert_eq!(current_name().as_deref(), Some("local-peer"));
/// ```
pub fn current_name() -> Option<String> {
    NAME.read()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
        .clone()
}

/// Clear the active name only if it still belongs to this peer.
///
/// # Arguments
/// * `name` - Name of the peer whose handle is shutting down.
/// # Returns
/// Returns unit, retaining a newer peer's identity when names differ.
/// # Examples
/// ```rust
/// use codetether_a2a_discovery::local_identity::{activate, current_name, deactivate};
/// activate("replacement");
/// deactivate("previous");
/// assert_eq!(current_name().as_deref(), Some("replacement"));
/// ```
pub fn deactivate(name: &str) {
    let mut active = NAME
        .write()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    if active.as_deref() == Some(name) {
        *active = None;
    }
}

#[cfg(test)]
#[path = "local_identity_tests.rs"]
mod tests;
