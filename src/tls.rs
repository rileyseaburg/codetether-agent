//! TLS / crypto provider initialization helpers.
//!
//! Rustls 0.23+ requires selecting a process-level `CryptoProvider` before
//! performing TLS operations. Every TLS path in CodeTether uses AWS-LC via
//! `aws-lc-rs`; `ring` is deliberately excluded because it has no FIPS 140-3
//! validation. Building with `--features fips` links the NIST-validated
//! AWS-LC FIPS module and selects its FIPS-approved provider.
//!
//! # Examples
//!
//! ```
//! codetether_agent::tls::ensure_rustls_crypto_provider();
//! assert!(rustls::crypto::CryptoProvider::get_default().is_some());
//! ```

mod fips;
mod provider;

pub use fips::{FipsStatus, fips_status, require_fips};
pub use provider::ensure_rustls_crypto_provider;
