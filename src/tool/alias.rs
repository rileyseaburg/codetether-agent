//! Compatibility aliases for renamed tools.

use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use serde_json::Value;

use super::{Tool, ToolResult};

tokio::task_local! {
    static POLICY_ID: String;
}

pub(crate) fn policy_id(default: &str) -> String {
    POLICY_ID
        .try_with(Clone::clone)
        .unwrap_or_else(|_| default.to_string())
}

pub(crate) async fn scoped<T>(id: &str, future: impl Future<Output = T>) -> T {
    POLICY_ID.scope(id.to_string(), future).await
}

pub struct AliasTool {
    id: String,
    name: String,
    inner: Arc<dyn Tool>,
}

impl AliasTool {
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
        scoped(&self.id, self.inner.execute(args)).await
    }
}
