//! One writer per canonical database path, including symlink aliases.
#[cfg(test)]
#[path = "registry_tests.rs"]
mod tests;

use super::{CoordinationError, worker::Writer};
use anyhow::{Context, Result};
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::{Arc, Mutex, OnceLock},
};

static WRITERS: OnceLock<Mutex<HashMap<PathBuf, Arc<Writer>>>> = OnceLock::new();

pub(super) fn get(locator: &Path) -> Result<Arc<Writer>> {
    anyhow::ensure!(!super::worker::is_current(), CoordinationError::Reentrant);
    let parent = locator.parent().context("session locator needs a parent")?;
    std::fs::create_dir_all(parent)?;
    let path = parent.canonicalize()?.join("sessions.sqlite3");
    let path = if path.exists() {
        path.canonicalize()?
    } else {
        path
    };
    let mut writers = WRITERS
        .get_or_init(Mutex::default)
        .lock()
        .map_err(|_| CoordinationError::RegistryPoisoned)?;
    if let Some(writer) = writers.get(&path) {
        return Ok(Arc::clone(writer));
    }
    let writer = Writer::spawn(path)?;
    writers.insert(writer.path.clone(), Arc::clone(&writer));
    Ok(writer)
}
