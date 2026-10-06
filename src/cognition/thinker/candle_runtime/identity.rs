//! Harness model identity captured when a native runtime is loaded.

use crate::cognition::ThinkerConfig;

/// Keep request identity separate from diagnostic device/checkpoint labels.
pub(crate) struct NativeModelIdentity(String);

impl NativeModelIdentity {
    /// Snapshot the harness model name; later config changes cannot relabel it.
    pub(crate) fn from_config(config: &ThinkerConfig) -> Self {
        Self(config.model.clone())
    }

    /// Return the model name supplied for this loaded runtime.
    pub(crate) fn model(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
#[path = "identity_tests.rs"]
mod tests;
