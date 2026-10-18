//! Process-wide shared usage windows.
use crate::RateGate;
use once_cell::sync::Lazy;
use std::sync::Arc;

/// Process-wide rate gate singleton.
pub static RATE_GATE: Lazy<Arc<RateGate>> = Lazy::new(|| Arc::new(RateGate::default()));
