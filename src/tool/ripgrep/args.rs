//! Typed view over the `rg` tool's JSON arguments.

use serde::Deserialize;

/// Default number of output lines returned to the model.
pub(crate) const DEFAULT_LIMIT: usize = 200;
/// Hard ceiling on returned output lines.
pub(crate) const MAX_LIMIT: usize = 2000;
/// Default wall-clock budget for one search.
pub(crate) const DEFAULT_TIMEOUT_SECS: u64 = 30;
/// Hard ceiling on the wall-clock budget.
pub(crate) const MAX_TIMEOUT_SECS: u64 = 120;

/// Deserialized `rg` invocation arguments.
#[derive(Debug, Default, Deserialize)]
pub(crate) struct RgArgs {
    pub pattern: String,
    #[serde(default)]
    pub paths: Vec<String>,
    #[serde(default)]
    pub glob: Vec<String>,
    #[serde(default)]
    pub fixed_strings: bool,
    #[serde(default)]
    pub case_insensitive: bool,
    #[serde(default)]
    pub files_with_matches: bool,
    #[serde(default)]
    pub context_lines: Option<usize>,
    #[serde(default)]
    pub max_count: Option<usize>,
    #[serde(default)]
    pub limit: Option<usize>,
    #[serde(default)]
    pub hidden: bool,
    #[serde(default)]
    pub no_ignore: bool,
    #[serde(default)]
    pub timeout_secs: Option<u64>,
}

impl RgArgs {
    /// Clamped output-line budget.
    pub fn limit(&self) -> usize {
        self.limit.unwrap_or(DEFAULT_LIMIT).clamp(1, MAX_LIMIT)
    }

    /// Clamped wall-clock budget.
    pub fn timeout(&self) -> std::time::Duration {
        let secs = self
            .timeout_secs
            .unwrap_or(DEFAULT_TIMEOUT_SECS)
            .clamp(1, MAX_TIMEOUT_SECS);
        std::time::Duration::from_secs(secs)
    }
}
