//! Tool system
//!
//! Tools are the executable capabilities available to agents.

pub mod bash;
pub mod edit;
pub mod file;
pub mod search;

use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;

/// A tool that can be executed by an agent
#[async_trait]
pub trait Tool: Send + Sync {
    /// Tool identifier
    fn id(&self) -> &str;

    /// Human-readable name
    fn name(&self) -> &str;

    /// Description for the LLM
    fn description(&self) -> &str;

    /// JSON Schema for parameters
    fn parameters(&self) -> Value;

    /// Execute the tool with given arguments
    async fn execute(&self, args: Value) -> Result<ToolResult>;
}

/// Result from tool execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolResult {
    pub output: String,
    pub success: bool,
    #[serde(default)]
    pub metadata: HashMap<String, Value>,
}

impl ToolResult {
    pub fn success(output: impl Into<String>) -> Self {
        Self {
            output: output.into(),
            success: true,
            metadata: HashMap::new(),
        }
    }

    pub fn error(message: impl Into<String>) -> Self {
        Self {
            output: message.into(),
            success: false,
            metadata: HashMap::new(),
        }
    }

    pub fn with_metadata(mut self, key: impl Into<String>, value: Value) -> Self {
        self.metadata.insert(key.into(), value);
        self
    }
}

/// Registry of available tools
pub struct ToolRegistry {
    tools: HashMap<String, Arc<dyn Tool>>,
}

impl ToolRegistry {
    pub fn new() -> Self {
        Self {
            tools: HashMap::new(),
        }
    }

    /// Register a tool
    pub fn register(&mut self, tool: Arc<dyn Tool>) {
        self.tools.insert(tool.id().to_string(), tool);
    }

    /// Get a tool by ID
    pub fn get(&self, id: &str) -> Option<Arc<dyn Tool>> {
        self.tools.get(id).cloned()
    }

    /// List all tool IDs
    pub fn list(&self) -> Vec<&str> {
        self.tools.keys().map(|s| s.as_str()).collect()
    }

    /// Get tool definitions for LLM
    pub fn definitions(&self) -> Vec<crate::provider::ToolDefinition> {
        self.tools
            .values()
            .map(|t| crate::provider::ToolDefinition {
                name: t.id().to_string(),
                description: t.description().to_string(),
                parameters: t.parameters(),
            })
            .collect()
    }

    /// Create registry with all default tools
    pub fn with_defaults() -> Self {
        let mut registry = Self::new();
        
        registry.register(Arc::new(file::ReadTool::new()));
        registry.register(Arc::new(file::WriteTool::new()));
        registry.register(Arc::new(file::ListTool::new()));
        registry.register(Arc::new(file::GlobTool::new()));
        registry.register(Arc::new(search::GrepTool::new()));
        registry.register(Arc::new(edit::EditTool::new()));
        registry.register(Arc::new(bash::BashTool::new()));
        
        registry
    }
}

impl Default for ToolRegistry {
    fn default() -> Self {
        Self::with_defaults()
    }
}
