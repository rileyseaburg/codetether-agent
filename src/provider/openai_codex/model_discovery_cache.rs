//! Process-wide cache of the last successful Codex model discovery.

use super::{DiscoveredModel, TTL};
use std::sync::RwLock;
use std::time::Instant;

struct Entry {
    at: Instant,
    models: Vec<DiscoveredModel>,
}

static CACHE: RwLock<Option<Entry>> = RwLock::new(None);

/// Replace the cached catalog with a fresh discovery result.
pub(crate) fn store(models: Vec<DiscoveredModel>) {
    if let Ok(mut slot) = CACHE.write() {
        *slot = Some(Entry {
            at: Instant::now(),
            models,
        });
    }
}

/// Cached models when a discovery is fresh (younger than [`TTL`]).
pub(crate) fn discovered() -> Option<Vec<DiscoveredModel>> {
    let slot = CACHE.read().ok()?;
    let entry = slot.as_ref()?;
    (entry.at.elapsed() < TTL).then(|| entry.models.clone())
}

/// Listed model slugs from the most recent discovery, fresh or stale.
///
/// Synchronous callers prefer a stale live catalog over the offline seed.
pub(crate) fn discovered_slugs() -> Option<Vec<String>> {
    let slot = CACHE.read().ok()?;
    let entry = slot.as_ref()?;
    let slugs: Vec<String> = entry
        .models
        .iter()
        .filter(|m| m.is_listed())
        .map(|m| m.slug.clone())
        .collect();
    (!slugs.is_empty()).then_some(slugs)
}

/// Capabilities from the latest discovery, retained even after its TTL.
pub(crate) fn model(slug: &str) -> Option<DiscoveredModel> {
    CACHE
        .read()
        .ok()?
        .as_ref()?
        .models
        .iter()
        .find(|m| m.slug == slug)
        .cloned()
}
