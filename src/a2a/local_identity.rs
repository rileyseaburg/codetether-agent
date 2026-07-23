//! Process-local identity for the active A2A LAN endpoint.
//!
//! Background peer startup records the exact advertised name here so model
//! prompts and collaboration tools can distinguish this process from peers.

use std::sync::{LazyLock, RwLock};

static NAME: LazyLock<RwLock<Option<String>>> = LazyLock::new(|| RwLock::new(None));

/// Record the exact LAN peer name advertised by this process.
pub(crate) fn activate(name: &str) {
    *NAME
        .write()
        .unwrap_or_else(std::sync::PoisonError::into_inner) = Some(name.to_string());
}

/// Return the active local LAN peer name, when an endpoint is running.
pub(crate) fn current_name() -> Option<String> {
    NAME.read()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
        .clone()
}

pub(crate) fn deactivate(name: &str) {
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
