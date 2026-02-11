//! Tool system
//!
//! Tools are the executable capabilities available to agents.

pub mod advanced_edit;
pub mod agent;
pub mod avatar;
pub mod bash;
pub mod batch;
pub mod codesearch;
pub mod confirm_edit;
pub mod confirm_multiedit;
pub mod edit;
pub mod file;
pub mod image;
pub mod invalid;
pub mod lsp;
pub mod mcp_bridge;
pub mod multiedit;
pub mod patch;
pub mod plan;
pub mod podcast;
pub mod prd;
pub mod question;
pub mod ralph;
pub mod rlm;
pub mod sandbox;
pub mod search;
pub mod skill;
pub mod swarm_share;
pub mod task;
pub mod todo;
pub mod undo;
pub mod voice;
pub mod webfetch;
pub mod websearch;
pub mod youtube;

use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;

use crate::provider::Provider;
pub use sandbox::{PluginManifest, PluginRegistry, SigningKey, hash_bytes, hash_file};

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

    /// Create a structured error with code, tool name, missing fields, and example
    ///
    /// This helps LLMs self-correct by providing actionable information about what went wrong.
    pub fn structured_error(
        code: &str,
        tool: &str,
        message: &str,
        missing_fields: Option<Vec<&str>>,
        example: Option<Value>,
    ) -> Self {
        let mut error_obj = serde_json::json!({
            "code": code,
            "tool": tool,
            "message": message,
        });

        if let Some(fields) = missing_fields {
            error_obj["missing_fields"] = serde_json::json!(fields);
        }

        if let Some(ex) = example {
            error_obj["example"] = ex;
        }

        let output = serde_json::to_string_pretty(&serde_json::json!({
            "error": error_obj
        }))
        .unwrap_or_else(|_| format!("Error: {}", message));

        let mut metadata = HashMap::new();
        metadata.insert("error_code".to_string(), serde_json::json!(code));
        metadata.insert("tool".to_string(), serde_json::json!(tool));

        Self {
            output,
            success: false,
            metadata,
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
    plugin_registry: PluginRegistry,
}

impl std::fmt::Debug for ToolRegistry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ToolRegistry")
            .field("tools", &self.tools.keys().collect::<Vec<_>>())
            .finish()
    }
}

impl ToolRegistry {
    pub fn new() -> Self {
        Self {
            tools: HashMap::new(),
            plugin_registry: PluginRegistry::from_env(),
        }
    }

    /// Get a reference to the plugin registry for managing signed plugins.
    pub fn plugins(&self) -> &PluginRegistry {
        &self.plugin_registry
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

    /// Create registry with all default tools (without batch)
    pub fn with_defaults() -> Self {
        let mut registry = Self::new();

        registry.register(Arc::new(file::ReadTool::new()));
        registry.register(Arc::new(file::WriteTool::new()));
        registry.register(Arc::new(file::ListTool::new()));
        registry.register(Arc::new(file::GlobTool::new()));
        registry.register(Arc::new(search::GrepTool::new()));
        registry.register(Arc::new(advanced_edit::AdvancedEditTool::new()));
        registry.register(Arc::new(bash::BashTool::new()));
        registry.register(Arc::new(lsp::LspTool::with_root(
            std::env::current_dir()
                .map(|p| format!("file://{}", p.display()))
                .unwrap_or_default(),
        )));
        registry.register(Arc::new(webfetch::WebFetchTool::new()));
        registry.register(Arc::new(multiedit::MultiEditTool::new()));
        registry.register(Arc::new(websearch::WebSearchTool::new()));
        registry.register(Arc::new(codesearch::CodeSearchTool::new()));
        registry.register(Arc::new(patch::ApplyPatchTool::new()));
        registry.register(Arc::new(todo::TodoReadTool::new()));
        registry.register(Arc::new(todo::TodoWriteTool::new()));
        registry.register(Arc::new(question::QuestionTool::new()));
        registry.register(Arc::new(task::TaskTool::new()));
        registry.register(Arc::new(plan::PlanEnterTool::new()));
        registry.register(Arc::new(plan::PlanExitTool::new()));
        registry.register(Arc::new(skill::SkillTool::new()));
        registry.register(Arc::new(rlm::RlmTool::new()));
        registry.register(Arc::new(ralph::RalphTool::new()));
        registry.register(Arc::new(prd::PrdTool::new()));
        registry.register(Arc::new(confirm_edit::ConfirmEditTool::new()));
        registry.register(Arc::new(confirm_multiedit::ConfirmMultiEditTool::new()));
        registry.register(Arc::new(undo::UndoTool));
        registry.register(Arc::new(voice::VoiceTool::new()));
        registry.register(Arc::new(podcast::PodcastTool::new()));
        registry.register(Arc::new(youtube::YouTubeTool::new()));
        registry.register(Arc::new(avatar::AvatarTool::new()));
        registry.register(Arc::new(image::ImageTool::new()));
        registry.register(Arc::new(mcp_bridge::McpBridgeTool::new()));
        // Register the invalid tool handler for graceful error handling
        registry.register(Arc::new(invalid::InvalidTool::new()));
        // Agent orchestration tool
        registry.register(Arc::new(agent::AgentTool::new()));

        registry
    }

    /// Create registry with provider for tools that need it (like RalphTool)
    pub fn with_provider(provider: Arc<dyn Provider>, model: String) -> Self {
        let mut registry = Self::new();

        registry.register(Arc::new(file::ReadTool::new()));
        registry.register(Arc::new(file::WriteTool::new()));
        registry.register(Arc::new(file::ListTool::new()));
        registry.register(Arc::new(file::GlobTool::new()));
        registry.register(Arc::new(search::GrepTool::new()));
        registry.register(Arc::new(advanced_edit::AdvancedEditTool::new()));
        registry.register(Arc::new(bash::BashTool::new()));
        registry.register(Arc::new(lsp::LspTool::with_root(
            std::env::current_dir()
                .map(|p| format!("file://{}", p.display()))
                .unwrap_or_default(),
        )));
        registry.register(Arc::new(webfetch::WebFetchTool::new()));
        registry.register(Arc::new(multiedit::MultiEditTool::new()));
        registry.register(Arc::new(websearch::WebSearchTool::new()));
        registry.register(Arc::new(codesearch::CodeSearchTool::new()));
        registry.register(Arc::new(patch::ApplyPatchTool::new()));
        registry.register(Arc::new(todo::TodoReadTool::new()));
        registry.register(Arc::new(todo::TodoWriteTool::new()));
        registry.register(Arc::new(question::QuestionTool::new()));
        registry.register(Arc::new(task::TaskTool::new()));
        registry.register(Arc::new(plan::PlanEnterTool::new()));
        registry.register(Arc::new(plan::PlanExitTool::new()));
        registry.register(Arc::new(skill::SkillTool::new()));
        registry.register(Arc::new(rlm::RlmTool::new()));
        // RalphTool with provider for autonomous execution
        registry.register(Arc::new(ralph::RalphTool::with_provider(provider, model)));
        registry.register(Arc::new(prd::PrdTool::new()));
        registry.register(Arc::new(undo::UndoTool));
        registry.register(Arc::new(voice::VoiceTool::new()));
        registry.register(Arc::new(podcast::PodcastTool::new()));
        registry.register(Arc::new(youtube::YouTubeTool::new()));
        registry.register(Arc::new(avatar::AvatarTool::new()));
        registry.register(Arc::new(image::ImageTool::new()));
        registry.register(Arc::new(mcp_bridge::McpBridgeTool::new()));
        // Register the invalid tool handler for graceful error handling
        registry.register(Arc::new(invalid::InvalidTool::new()));
        // Agent orchestration tool
        registry.register(Arc::new(agent::AgentTool::new()));

        registry
    }

    /// Create Arc-wrapped registry with batch tool properly initialized.
    /// The batch tool needs a weak reference to the registry, so we use
    /// a two-phase initialization pattern.
    pub fn with_defaults_arc() -> Arc<Self> {
        let mut registry = Self::with_defaults();

        // Create batch tool without registry reference
        let batch_tool = Arc::new(batch::BatchTool::new());
        registry.register(batch_tool.clone());

        // Wrap registry in Arc
        let registry = Arc::new(registry);

        // Now give batch tool a weak reference to the registry
        batch_tool.set_registry(Arc::downgrade(&registry));

        registry
    }

    /// Create Arc-wrapped registry with provider and batch tool properly initialized.
    /// The batch tool needs a weak reference to the registry, so we use
    /// a two-phase initialization pattern.
    #[allow(dead_code)]
    pub fn with_provider_arc(provider: Arc<dyn Provider>, model: String) -> Arc<Self> {
        let mut registry = Self::with_provider(provider, model);

        // Create batch tool without registry reference
        let batch_tool = Arc::new(batch::BatchTool::new());
        registry.register(batch_tool.clone());

        // Wrap registry in Arc
        let registry = Arc::new(registry);

        // Now give batch tool a weak reference to the registry
        batch_tool.set_registry(Arc::downgrade(&registry));

        registry
    }
}

impl Default for ToolRegistry {
    fn default() -> Self {
        Self::with_defaults()
    }
}
