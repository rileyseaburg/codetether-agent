//! Typed write_stdin arguments and bounded polling options.

use serde::Deserialize;

#[derive(Deserialize)]
pub(super) struct Input {
    pub session_id: u64,
    #[serde(default)]
    pub chars: String,
    #[serde(default)]
    pub yield_time_ms: Option<u64>,
    #[serde(default)]
    pub max_output_tokens: Option<usize>,
}

impl Input {
    pub fn yield_ms(&self) -> u64 {
        match (&*self.chars, self.yield_time_ms) {
            ("", Some(value)) => value.clamp(5_000, 300_000),
            ("", None) => 5_000,
            (_, Some(value)) => value.clamp(250, 30_000),
            (_, None) => 250,
        }
    }

    pub fn max_bytes(&self) -> usize {
        self.max_output_tokens
            .unwrap_or(10_000)
            .saturating_mul(4)
            .max(4)
            .min(crate::tool::tool_output_budget())
    }
}
