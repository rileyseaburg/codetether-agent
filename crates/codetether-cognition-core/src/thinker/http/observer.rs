//! Host-owned observation of HTTP thinker requests.
use super::ThinkerOutput;
use std::time::Instant;

/// Receives diagnostics at the original points in the retry lifecycle.
/// Implementations decide where to record logs; callbacks must not retain borrowed data.
///
/// # Implementors
/// Hosts implement this trait to preserve their logging targets and metrics.
///
/// # Examples
/// ```
/// use codetether_cognition_core::thinker::{ThinkerOutput, http::HttpObserver};
/// struct Quiet;
/// impl HttpObserver for Quiet {
///     fn transient_transport(&self, _: &reqwest::Error) {}
///     fn transient_status(&self, _: reqwest::StatusCode) {}
///     fn retry(&self, _: u32) {}
///     fn success(&self, _: &ThinkerOutput, _: std::time::Instant, _: u32) {}
/// }
/// ```
pub trait HttpObserver: Send + Sync {
    /// A retryable transport failure occurred.
    fn transient_transport(&self, error: &reqwest::Error);
    /// A retryable HTTP status was received and its body consumed.
    fn transient_status(&self, status: reqwest::StatusCode);
    /// The delay before a retry completed.
    fn retry(&self, attempt: u32);
    /// A response was decoded successfully.
    fn success(&self, output: &ThinkerOutput, started_at: Instant, attempt: u32);
}
