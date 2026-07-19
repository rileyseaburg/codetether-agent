//! Validation and race-safe reads for persisted runtime identities.

use anyhow::{Context, Result};
use std::path::Path;
use std::time::Duration;

pub(super) fn immediate(path: &Path) -> Option<String> {
    let identity = std::fs::read_to_string(path).ok()?.trim().to_string();
    valid(&identity).then_some(identity)
}

pub(super) fn after_create(path: &Path) -> Result<String> {
    for _ in 0..20 {
        if let Some(identity) = immediate(path) {
            return Ok(identity);
        }
        std::thread::sleep(Duration::from_millis(5));
    }
    immediate(path).context("Existing A2A identity is invalid or unreadable")
}

fn valid(identity: &str) -> bool {
    identity
        .strip_prefix("ctagent_")
        .is_some_and(|suffix| suffix.len() == 32 && suffix.chars().all(|ch| ch.is_ascii_hexdigit()))
}
