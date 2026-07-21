//! V2 arguments with hidden V1 target compatibility.

use super::RuntimeContext;
use serde::Deserialize;

#[derive(Deserialize)]
pub(super) struct Args {
    #[serde(default)]
    target: Option<String>,
    #[serde(default)]
    targets: Vec<String>,
    #[serde(default = "default_timeout_ms")]
    pub(super) timeout_ms: u64,
    #[serde(flatten)]
    pub(super) context: RuntimeContext,
}

fn default_timeout_ms() -> u64 {
    30_000
}

impl Args {
    pub(super) fn requested_targets(&self) -> Vec<String> {
        let mut targets = self.targets.clone();
        if let Some(target) = &self.target {
            targets.push(target.clone());
        }
        targets
    }
}
