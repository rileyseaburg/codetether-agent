//! Tests for automatic embedder selection (resource gating + catalog).

use super::catalog::{BEST, COMPACT, best_fitting};
use super::resources::SystemCapability;

#[test]
fn catalog_picks_best_for_large_ram() {
    assert_eq!(best_fitting(64 * 1024 * 1024 * 1024).repo, BEST.repo);
}

#[test]
fn catalog_picks_compact_for_small_ram() {
    assert_eq!(best_fitting(5 * 1024 * 1024 * 1024).repo, COMPACT.repo);
}

#[test]
fn low_memory_host_rejects_local_embedding() {
    let caps = SystemCapability {
        total_memory_bytes: 2 * 1024 * 1024 * 1024,
        cpu_count: 2,
        cuda_build: false,
    };
    assert!(!caps.supports_local_embedding());
}

#[test]
fn capable_host_supports_local_embedding() {
    let caps = SystemCapability {
        total_memory_bytes: 16 * 1024 * 1024 * 1024,
        cpu_count: 8,
        cuda_build: false,
    };
    assert!(caps.supports_local_embedding());
}
