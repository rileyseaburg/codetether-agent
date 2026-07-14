//! Git adapter that roots operations and gates commits by capability.

use crate::tool::{Tool, ToolResult, git::GitTool};
use anyhow::Result;
use async_trait::async_trait;
use serde_json::Value;
use std::path::PathBuf;

pub(super) struct ScopedGitTool {
    inner: GitTool,
    root: PathBuf,
    allow_commit: bool,
}

impl ScopedGitTool {
    pub(super) fn new(root: PathBuf, allow_commit: bool) -> Self {
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
    async fn execute(&self, mut args: Value) -> Result<ToolResult> {
        if !self.allow_commit && args.get("op").and_then(Value::as_str) == Some("commit") {
            return Ok(ToolResult::error(
                "git commit is unavailable for this swarm task",
            ));
        }
        if let Some(fields) = args.as_object_mut() {
            fields.insert("cwd".into(), Value::String(self.root.display().to_string()));
        }
        self.inner.execute(args).await
    }
}
