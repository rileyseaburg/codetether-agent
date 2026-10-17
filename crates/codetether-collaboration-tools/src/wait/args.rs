//! V2 arguments with hidden V1 target compatibility.

use crate::RuntimeContext;
use serde::Deserialize;

#[derive(Deserialize)]
/// Wait timeout and hidden legacy target selectors with parent runtime context.
///
/// # Examples
/// ```
/// let args:codetether_collaboration_tools::WaitArgs=serde_json::from_value(serde_json::json!({})).unwrap();
/// assert!(args.requested_targets().is_empty());
/// ```
pub struct WaitArgs {
    #[serde(default)]
    target: Option<String>,
    #[serde(default)]
    targets: Vec<String>,
    #[serde(default = "default_timeout_ms")]
    /// Timeout in milliseconds, defaulting to 30,000.
    pub timeout_ms: u64,
    #[serde(flatten)]
    pub(super) context: RuntimeContext,
}

fn default_timeout_ms() -> u64 {
    30_000
}

impl WaitArgs {
    pub(super) fn requested_targets(&self) -> Vec<String> {
        let mut targets = self.targets.clone();
        if let Some(target) = &self.target {
            targets.push(target.clone());
        }
        targets
    }
}
