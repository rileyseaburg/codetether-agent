//! RLM tool: Recursive Language Model for large context analysis.
//!
//! This module exposes the tool shell and delegates schema, input loading,
//! and processing to focused submodules.

mod collect;
mod ctx;
mod execute;
mod fallback;
mod schema;

use super::{Tool, ToolResult};
use crate::provider::Provider;
use crate::rlm::RlmConfig;
use anyhow::Result;
use async_trait::async_trait;
use serde_json::Value;
use std::sync::Arc;

/// RLM Tool - Invoke the Recursive Language Model subsystem
/// for analyzing large codebases that exceed the context window.
pub struct RlmTool {
    pub(super) provider: Arc<dyn Provider>,
    pub(super) model: String,
    pub(super) config: RlmConfig,
}

impl RlmTool {
    /// Build an `RlmTool` backed by `provider`/`model`.
    pub fn new(provider: Arc<dyn Provider>, model: String, config: RlmConfig) -> Self {
        Self {
            provider,
            model,
            config,
        }
    }
}

#[async_trait]
impl Tool for RlmTool {
    fn id(&self) -> &str {
        "rlm"
    }

    fn name(&self) -> &str {
        "RLM"
    }

    fn description(&self) -> &str {
        schema::DESCRIPTION
    }

    fn parameters(&self) -> Value {
        schema::parameters()
    }

    async fn execute(&self, args: Value) -> Result<ToolResult> {
        execute::run(self, args).await
    }
}
