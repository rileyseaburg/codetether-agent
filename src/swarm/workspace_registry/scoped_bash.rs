//! Bash adapter that always executes from its assigned workspace.

use crate::tool::{Tool, ToolResult, bash::BashTool};
use anyhow::Result;
use async_trait::async_trait;
use serde_json::Value;
use std::path::PathBuf;

pub(super) struct ScopedBashTool {
    inner: BashTool,
    root: PathBuf,
}

impl ScopedBashTool {
    pub(super) fn new(root: PathBuf) -> Self {
        Self {
            inner: BashTool::with_cwd(root.clone()),
            root,
        }
    }
}

#[async_trait]
impl Tool for ScopedBashTool {
    fn id(&self) -> &str {
        "bash"
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
        if let Some(fields) = args.as_object_mut() {
            fields.insert("cwd".into(), Value::String(self.root.display().to_string()));
        }
        self.inner.execute(args).await
    }
}
