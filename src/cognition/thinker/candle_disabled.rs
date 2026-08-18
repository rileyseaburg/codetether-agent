//! `CandleThinker` stand-in for builds without the `candle` feature.
//!
//! Construction always fails with a rebuild hint so the Candle backend is a
//! runtime configuration error rather than a compile-time dependency.

use super::{ThinkerConfig, ThinkerOutput};
use anyhow::{Result, anyhow};

/// Placeholder runtime that cannot execute local inference.
pub(crate) struct CandleThinker;

/// Error returned whenever the Candle backend is requested in this build.
fn unsupported() -> anyhow::Error {
    anyhow!("candle thinker backend requires --features candle")
}

impl CandleThinker {
    /// Always fails; the Candle dependency is not compiled in.
    pub(crate) fn new(_config: &ThinkerConfig) -> Result<Self> {
        Err(unsupported())
    }

    /// Always fails; retained for signature parity with Candle builds.
    pub(crate) fn think(&mut self, _system: &str, _user: &str) -> Result<ThinkerOutput> {
        Err(unsupported())
    }
}
