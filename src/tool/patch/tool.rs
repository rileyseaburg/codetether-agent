//! Public apply_patch tool wrapper.

use super::{pipeline, schema};
use crate::tool::{Tool, ToolResult};
use anyhow::Result;
use async_trait::async_trait;
use serde_json::Value;
use std::path::PathBuf;

/// Tool that applies unified diff patches under a workspace root.
pub struct ApplyPatchTool {
    root: PathBuf,
}

impl Default for ApplyPatchTool {
    fn default() -> Self {
        Self::new()
    }
}

impl ApplyPatchTool {
    /// Create a tool rooted at the current process directory.
    pub fn new() -> Self {
        Self {
            root: std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")),
        }
    }

    /// Create a tool rooted at a specific workspace path.
    pub fn with_root(root: PathBuf) -> Self {
        Self { root }
    }
}

#[async_trait]
impl Tool for ApplyPatchTool {
    fn id(&self) -> &str {
        "apply_patch"
    }
    fn name(&self) -> &str {
        "Apply Patch"
    }
    fn description(&self) -> &str {
        "Apply a unified diff patch to files in the workspace."
    }
    fn parameters(&self) -> Value {
        schema::parameters()
    }
    async fn execute(&self, params: Value) -> Result<ToolResult> {
        pipeline::execute(&self.root, params).await
    }
}
