//! Process-wide rustls provider selection (AWS-LC, FIPS when compiled in).

use rustls::crypto::CryptoProvider;
use std::sync::OnceLock;

static RUSTLS_PROVIDER_INSTALLED: OnceLock<()> = OnceLock::new();

/// Install the AWS-LC rustls provider once for the whole process.
///
/// Safe to call multiple times. With the `fips` feature the FIPS-approved
/// provider (FIPS cipher suites, key exchange groups, and signature
/// algorithms only) is installed instead of the default one.
///
/// # Examples
///
/// ```
/// codetether_agent::tls::ensure_rustls_crypto_provider();
/// codetether_agent::tls::ensure_rustls_crypto_provider();
/// assert!(rustls::crypto::CryptoProvider::get_default().is_some());
/// ```
pub fn ensure_rustls_crypto_provider() {
    RUSTLS_PROVIDER_INSTALLED.get_or_init(|| {
        if let Err(error) = selected_provider().install_default() {
            tracing::debug!(?error, "rustls crypto provider was already installed");
        }
    });
}

#[cfg(feature = "fips")]
fn selected_provider() -> CryptoProvider {
    rustls::crypto::default_fips_provider()
}

#[cfg(not(feature = "fips"))]
fn selected_provider() -> CryptoProvider {
    rustls::crypto::aws_lc_rs::default_provider()
}
