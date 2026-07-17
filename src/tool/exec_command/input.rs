//! Typed decoding and bounded execution options.

use serde::Deserialize;

#[derive(Deserialize)]
pub(super) struct Input {
    pub cmd: String,
    #[serde(default)]
    pub workdir: Option<String>,
    #[serde(default)]
    pub shell: Option<String>,
    #[serde(default)]
    pub login: bool,
    #[serde(default)]
    pub tty: bool,
    #[serde(default = "default_yield_ms")]
    pub yield_time_ms: u64,
    #[serde(default)]
    pub max_output_tokens: Option<usize>,
}

impl Input {
    pub fn yield_ms(&self) -> u64 {
        self.yield_time_ms.clamp(250, 30_000)
    }

    pub fn max_bytes(&self) -> usize {
        self.max_output_tokens
            .unwrap_or(10_000)
            .saturating_mul(4)
            .max(4)
            .min(crate::tool::tool_output_budget())
    }
}

fn default_yield_ms() -> u64 {
    10_000
}
