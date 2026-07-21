//! Spawn arguments and fork-history compatibility defaults.

use crate::tool::collaboration::context::RuntimeContext;
use serde::Deserialize;

#[derive(Deserialize)]
pub(super) struct Args {
    pub(super) task_name: Option<String>,
    pub(super) message: String,
    pub(super) fork_turns: Option<String>,
    pub(super) fork_context: Option<bool>,
    pub(super) model: Option<String>,
    #[serde(flatten)]
    pub(super) context: RuntimeContext,
}

impl Args {
    pub(super) fn resolved_fork_turns(&self) -> String {
        self.fork_turns
            .clone()
            .unwrap_or_else(|| match self.fork_context {
                Some(false) => "none".into(),
                Some(true) | None => "all".into(),
            })
    }
}

#[cfg(test)]
#[path = "args_tests.rs"]
mod tests;
