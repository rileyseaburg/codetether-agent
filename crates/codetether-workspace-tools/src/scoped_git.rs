//! Git adapter that roots operations and gates commits by capability.

use anyhow::Result;
use async_trait::async_trait;
use codetether_git_tool::GitTool;
use codetether_tool_core::{Tool, ToolResult};
use serde_json::Value;
use std::path::PathBuf;

#[path = "scoped_git_run.rs"]
mod run;

pub(crate) struct ScopedGitTool {
    inner: GitTool,
    root: PathBuf,
    allow_commit: bool,
}

impl ScopedGitTool {
    pub(crate) fn new(root: PathBuf, allow_commit: bool) -> Self {
        Self {
            inner: GitTool::new(),
            root,
            allow_commit,
        }
    }
}

#[async_trait]
impl Tool for ScopedGitTool {
    fn id(&self) -> &str {
        "git"
    }
    fn name(&self) -> &str {
        self.inner.name()
    }
    fn description(&self) -> &str {
        self.inner.description()
    }
    fn parameters(&self) -> Value {
        self.inner.parameters()
    }
    async fn execute(&self, args: Value) -> Result<ToolResult> {
        run::execute(self, args).await
    }
}
