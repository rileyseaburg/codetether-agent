//! # Structured Git Tool
//!
//! Exposes common Git operations with typed dispatch instead of raw shell
//! string-scraping: `status`, `diff`, `diff_staged`, `log`, `branch`, `show`,
//! and `commit`. Each operation lives in its own SRP module.

mod commit;
mod ops;
mod run;
mod schema;
#[cfg(test)]
mod tests;
#[cfg(test)]
mod tests_helpers;

use super::{Tool, ToolResult};
use anyhow::Result;
use async_trait::async_trait;
use serde_json::Value;

/// Git operations tool.
pub struct GitTool;

impl Default for GitTool {
    fn default() -> Self {
        Self::new()
    }
}

impl GitTool {
    /// Create a new git tool.
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl Tool for GitTool {
    fn id(&self) -> &str {
        "git"
    }

    fn name(&self) -> &str {
        "Git"
    }

    fn description(&self) -> &str {
        "git(op: string, path?: string, message?: string, paths?: string[], cwd?: string) - Run a structured git operation: status, diff, diff_staged, log, branch, show, or commit."
    }

    fn parameters(&self) -> Value {
        schema::parameters()
    }

    async fn execute(&self, args: Value) -> Result<ToolResult> {
        let op = args.get("op").and_then(|v| v.as_str()).unwrap_or("");
        if op == "commit" {
            commit::run_commit(&args).await
        } else {
            ops::run_readonly(op, &args).await
        }
    }
}
