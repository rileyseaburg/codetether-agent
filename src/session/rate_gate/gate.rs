//! Rate-gate: check budget before sending, record after.
//!
//! `try_acquire` returns `None` if the model has headroom, or
//! `Some(cooldown_ms)` if the caller should route elsewhere and retry
//! the primary model after that many milliseconds.

use std::collections::HashMap;
use std::sync::Mutex;
use std::time::Duration;

use super::limits::limits_for;
use super::window::SlidingWindow;

/// Global rate-gate state, keyed by `"provider/model"`.
#[derive(Debug, Default)]
pub struct RateGate {
    windows: Mutex<HashMap<String, SlidingWindow>>,
}

impl RateGate {
    /// Check whether `provider/model` has headroom for a request
    /// estimated to use `est_tokens` tokens.
    ///
    /// Returns `None` → OK to proceed.
    /// Returns `Some(ms)` → throttled; route elsewhere, retry after `ms`.
    pub fn try_acquire(&self, provider: &str, model: &str, est_tokens: u32) -> Option<Duration> {
        let limits = limits_for(provider, model);
        if limits.rpm == 0 && limits.tpm == 0 {
            return None; // unlimited
        }
        let key = format!("{provider}/{model}");
        let mut map = self.windows.lock().unwrap_or_else(|e| e.into_inner());
        let w = map.entry(key).or_default();
        let rpm_ok = limits.rpm == 0 || w.rpm() < limits.rpm;
        let tpm_ok = limits.tpm == 0 || w.tpm() + est_tokens <= limits.tpm;
        if rpm_ok && tpm_ok {
            return None;
        }
        let cooldown_ms = w.ms_until_oldest_expires().max(1_000);
        Some(Duration::from_millis(cooldown_ms))
    }

    /// Record a completed request so future `try_acquire` calls see it.
    pub fn record(&self, provider: &str, model: &str, tokens: u32) {
        let key = format!("{provider}/{model}");
        let mut map = self.windows.lock().unwrap_or_else(|e| e.into_inner());
        map.entry(key).or_default().record(tokens);
    }
}
