//! Read-only `bash` for the goal verifier's `Verification` capability.
//!
//! The verifier must observe the workspace without changing it. Every
//! command runs through the command sandbox with no writable paths, so
//! bubblewrap/Seatbelt mount the workspace read-only. When no real sandbox
//! backend is available the command is refused instead of running unconfined.

use crate::tool::{Tool, ToolResult, bash::BashTool};
use anyhow::Result;
use async_trait::async_trait;
use serde_json::Value;
use std::path::PathBuf;

#[path = "readonly_bash_run.rs"]
mod run;

pub(super) struct ReadOnlyBashTool {
    inner: BashTool,
    root: PathBuf,
}

impl ReadOnlyBashTool {
    pub(super) fn new(root: PathBuf) -> Self {
        Self {
            inner: BashTool::with_cwd(root.clone()),
            root,
        }
    }
}

#[async_trait]
impl Tool for ReadOnlyBashTool {
    fn id(&self) -> &str {
        "bash"
    }
    fn name(&self) -> &str {
        self.inner.name()
    }
    fn description(&self) -> &str {
        "Run a shell command with the workspace mounted read-only (verification only)."
    }
    fn parameters(&self) -> Value {
        self.inner.parameters()
    }
    async fn execute(&self, args: Value) -> Result<ToolResult> {
        run::run(&self.root, &args).await
    }
}
