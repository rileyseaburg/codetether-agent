//! Compatibility aliases for renamed tools.

use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use serde_json::Value;

use crate::{Tool, ToolResult};

/// Alternate identifier for an existing tool, preserving its execution and schema.
///
/// # Examples
/// ```
/// use codetether_tool_core::{AliasTool, Tool};
/// # fn wrap(tool: std::sync::Arc<dyn Tool>) {
/// let alias = AliasTool::new("legacy", tool);
/// assert_eq!(alias.id(), "legacy");
/// # }
/// ```
pub struct AliasTool {
    id: String,
    name: String,
    inner: Arc<dyn Tool>,
}

impl AliasTool {
    /// Wrap a tool under the supplied identifier.
    ///
    /// # Arguments
    /// * `id` - Alias registered by a caller.
    /// * `inner` - Shared underlying implementation.
    ///
    /// # Returns
    /// An alias with the underlying tool's name, description, schema, and execution.
    pub fn new(id: impl Into<String>, inner: Arc<dyn Tool>) -> Self {
        Self {
            id: id.into(),
            name: inner.name().to_string(),
            inner,
        }
    }
}

#[async_trait]
impl Tool for AliasTool {
    fn id(&self) -> &str {
        &self.id
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn description(&self) -> &str {
        self.inner.description()
    }

    fn parameters(&self) -> Value {
        self.inner.parameters()
    }

    async fn execute(&self, args: Value) -> Result<ToolResult> {
        self.inner.execute(args).await
    }
}
