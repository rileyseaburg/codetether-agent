//! Persistent fallback identity for one CodeTether workspace.

use anyhow::{Context, Result};
use std::path::Path;
use uuid::Uuid;

pub(super) fn load_or_create() -> Option<String> {
    let path = crate::config::Config::data_dir()?
        .join("a2a")
        .join("agent-identity");
    match load_or_create_at(&path) {
        Ok(identity) => Some(identity),
        Err(error) => {
            tracing::warn!(path = %path.display(), error = %error,
                "Unable to load durable A2A agent identity");
            None
        }
    }
}

fn load_or_create_at(path: &Path) -> Result<String> {
    if let Some(identity) = super::runtime_identity_read::immediate(path) {
        return Ok(identity);
    }
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).context("Failed to create A2A identity directory")?;
    }
    let identity = format!("ctagent_{}", Uuid::new_v4().simple());
    match super::runtime_identity_write::publish(path, &identity)? {
        true => Ok(identity),
        false => super::runtime_identity_read::after_create(path),
    }
}

#[cfg(test)]
#[path = "runtime_identity_store_tests.rs"]
mod tests;
