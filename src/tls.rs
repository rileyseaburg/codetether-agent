//! TLS / crypto provider initialization helpers.
//!
//! Rustls 0.23+ requires selecting a process-level `CryptoProvider` before
//! performing TLS operations. In binaries this is typically done at startup,
//! but our unit tests exercise code paths that create HTTPS clients directly.
//!
//! This module provides a single, idempotent initializer.

use std::sync::OnceLock;

static RUSTLS_PROVIDER_INSTALLED: OnceLock<()> = OnceLock::new();

/// Ensure the rustls crypto provider is installed.
///
/// Safe to call multiple times.
pub fn ensure_rustls_crypto_provider() {
    RUSTLS_PROVIDER_INSTALLED.get_or_init(|| {
        // We compile rustls with the `ring` provider.
        if let Err(e) = rustls::crypto::ring::default_provider().install_default() {
            // Ignore "already installed" style errors. Any other error is still
            // non-fatal here; downstream TLS operations will surface failures.
            tracing::debug!(error = ?e, "rustls crypto provider install_default() returned error");
        }
    });
}
