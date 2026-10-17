//! Exercise the remote embedding contract without credentials or network access.

use super::*;
use codetether_provider_core::{EmbeddingRequest, EmbeddingResponse, Usage};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

#[derive(Default)]
struct Calls {
    resolutions: AtomicUsize,
    requests: Mutex<Vec<EmbeddingRequest>>,
}
struct Resolver {
    calls: Arc<Calls>,
    failures: usize,
    dimensions: usize,
}
struct Provider {
    calls: Arc<Calls>,
    failures: usize,
    dimensions: usize,
}
fn fixture(failures: usize, dimensions: usize) -> (tempfile::TempDir, Resolver, BuildOptions) {
    let dir = tempfile::tempdir().unwrap();
    for name in ["a.rs", "b.rs", "c.rs"] {
        std::fs::write(dir.path().join(name), "pub fn run() {}\n").unwrap();
    }
    let resolver = Resolver {
        calls: Arc::default(),
        failures,
        dimensions,
    };
    let options = BuildOptions {
        embedding_provider: "remote".into(),
        embedding_model: "model".into(),
        embedding_dimensions: 64,
        embedding_batch_size: 2,
        embedding_max_retries: 1,
        embedding_retry_initial_ms: 1,
        embedding_retry_max_ms: 1,
        ..Default::default()
    };
    (dir, resolver, options)
}
mod dimensions;
mod local;
mod provider;
mod resolver;
mod retries;
