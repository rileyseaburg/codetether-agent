//! FIPS 140-3 self-check for the linked AWS-LC module and rustls provider.

use anyhow::{Result, bail};

/// Observed FIPS state of the running process.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FipsStatus {
    /// Binary was built with the `fips` feature (AWS-LC FIPS module linked).
    pub compiled: bool,
    /// AWS-LC reports its FIPS module is active (`FIPS_mode() == 1`).
    pub module_active: bool,
    /// Installed rustls provider uses only FIPS-approved algorithms.
    pub provider_approved: bool,
}

impl FipsStatus {
    /// True only when every layer reports FIPS operation.
    pub fn enforced(&self) -> bool {
        self.compiled && self.module_active && self.provider_approved
    }
}

/// Report the process FIPS state without changing it.
///
/// # Examples
///
/// ```
/// codetether_agent::tls::ensure_rustls_crypto_provider();
/// let status = codetether_agent::tls::fips_status();
/// assert_eq!(status.compiled, cfg!(feature = "fips"));
/// ```
pub fn fips_status() -> FipsStatus {
    let provider_approved =
        rustls::crypto::CryptoProvider::get_default().is_some_and(|provider| provider.fips());
    FipsStatus {
        compiled: cfg!(feature = "fips"),
        module_active: aws_lc_rs::try_fips_mode().is_ok(),
        provider_approved,
    }
}

/// Fail closed when FIPS is required but not fully active.
///
/// FIPS builds always require it; other builds require it only when
/// `CODETETHER_REQUIRE_FIPS` is truthy, so operators get a hard error
/// instead of silently running non-validated cryptography.
pub fn require_fips() -> Result<FipsStatus> {
    let status = fips_status();
    if required() && !status.enforced() {
        bail!("FIPS 140-3 mode required but not active: {status:?}");
    }
    Ok(status)
}

fn required() -> bool {
    cfg!(feature = "fips")
        || std::env::var("CODETETHER_REQUIRE_FIPS")
            .is_ok_and(|value| matches!(value.as_str(), "1" | "true" | "yes"))
}

#[cfg(test)]
#[path = "fips_tests.rs"]
mod tests;
