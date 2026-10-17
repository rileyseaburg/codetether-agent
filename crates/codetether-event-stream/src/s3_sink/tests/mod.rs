//! Environment configuration regressions.

use super::*;
use std::sync::{Mutex, OnceLock};

fn env_lock() -> &'static Mutex<()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
}

// Helper to ensure clean environment before each test
fn clean_env() {
    // Note: set_var/remove_var are unsafe in Rust 2024+, but this is test-only code
    unsafe {
        // Remove both legacy CODETETHER_S3_* and new S3_* variables
        std::env::remove_var("CODETETHER_S3_BUCKET");
        std::env::remove_var("CODETETHER_S3_PREFIX");
        std::env::remove_var("CODETETHER_S3_ENDPOINT");
        std::env::remove_var("CODETETHER_S3_REGION");
        std::env::remove_var("CODETETHER_S3_ACCESS_KEY");
        std::env::remove_var("CODETETHER_S3_SECRET_KEY");
        std::env::remove_var("CODETETHER_S3_STUB_MODE");
        std::env::remove_var("S3_BUCKET");
        std::env::remove_var("S3_PREFIX");
        std::env::remove_var("S3_ENDPOINT");
        std::env::remove_var("S3_REGION");
        std::env::remove_var("S3_ACCESS_KEY");
        std::env::remove_var("S3_SECRET_KEY");
        std::env::remove_var("S3_STUB_MODE");
        std::env::remove_var("AWS_REGION");
    }
}
mod configuration;
mod configured;
mod defaults;
