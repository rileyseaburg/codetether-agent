//! Tool system
//!
//! Tools are the executable capabilities available to agents.

pub mod advanced_edit;
pub mod agent;
pub mod alias;
pub mod auto_apply;
pub mod avatar;
include!("command_modules.rs");
pub mod bash_file_edit_guard;
mod bash_github;
mod bash_identity;
mod bash_noninteractive;
pub(crate) mod bash_shell;
pub mod batch;
pub mod browserctl;
pub use codetether_tool_core::budget;
pub mod bus_inspect;
pub mod codesearch;
pub mod computer_use;
pub mod confirm_edit;
pub mod confirm_multiedit;
pub mod context_browse;
pub mod context_budget;
pub(super) mod context_helpers;
pub mod context_pin;
pub mod context_reset;
pub mod context_summarize;
pub mod edit;
pub(crate) mod feedback;
pub mod file;
pub mod file_extras;
pub mod git;
pub mod go;
include!("image_modules.rs");
pub mod invalid;
pub mod k8s_tool;
pub mod lsp;
include!("mcp_modules.rs");
mod catalog;
pub mod goal;
pub mod mcp_tools;
pub mod memory;
pub mod morph_backend;
pub mod multiedit;
pub mod okr;
pub mod patch;
pub mod plan;
pub mod podcast;
pub mod prd;
pub mod profile;
pub mod progress;
pub mod question;
pub mod ralph;
pub mod readonly;
mod registry_lifecycle;
pub mod relay_autochat;
pub mod ripgrep;
pub mod rlm;
pub mod sandbox;
mod sandbox_network;
mod sandbox_paths;
pub mod search;
pub mod search_router;
pub mod session_recall;
pub mod skill;
pub mod swarm_execute;
pub mod swarm_share;
pub mod task;
#[cfg(feature = "tetherscript")]
pub mod tetherscript;
pub mod todo;
pub mod undo;
pub mod voice;
pub mod voice_input;
pub mod voice_stream;
pub mod webfetch;
pub mod websearch;
pub mod youtube;

use std::sync::Arc;

use crate::provider::Provider;
pub use budget::{DEFAULT_TOOL_OUTPUT_MAX_BYTES, tool_output_budget};
pub use mcp_tools::{McpToolManager, McpToolWrapper};
pub use sandbox::{PluginManifest, PluginRegistry, SigningKey, hash_bytes, hash_file};

pub use codetether_tool_core::{Tool, ToolResult};

/// Registry of available tools
pub struct ToolRegistry {
    inner: codetether_tool_registry::ToolRegistry,
}

impl std::fmt::Debug for ToolRegistry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.inner, f)
    }
}

impl ToolRegistry {
    pub fn new() -> Self {
        Self {
            inner: codetether_tool_registry::ToolRegistry::new(),
        }
    }

    /// Get a reference to the plugin registry for managing signed plugins.
    pub fn plugins(&self) -> &PluginRegistry {
        self.inner.plugins()
    }

    /// Register a tool
    pub fn register(&mut self, tool: Arc<dyn Tool>) {
        self.inner.register(tool);
    }

    /// Get a tool by ID
    pub fn get(&self, id: &str) -> Option<Arc<dyn Tool>> {
        self.inner.get(id)
    }

    /// List all tool IDs
    pub fn list(&self) -> Vec<&str> {
        self.inner.list()
    }

    /// Get tool definitions for LLM
    pub fn definitions(&self) -> Vec<crate::provider::ToolDefinition> {
        self.inner.definitions()
    }

    /// Register multiple tools at once
    pub fn register_all(&mut self, tools: Vec<Arc<dyn Tool>>) {
        self.inner.register_all(tools);
    }

    /// Remove a tool by ID
    pub fn unregister(&mut self, id: &str) -> Option<Arc<dyn Tool>> {
        self.inner.unregister(id)
    }

    /// Check if a tool exists
    pub fn contains(&self, id: &str) -> bool {
        self.inner.contains(id)
    }

    /// Get the number of registered tools
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// Check if the registry is empty
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Create registry with all default tools (without batch)
    pub fn with_defaults() -> Self {
        let mut registry = Self::new();
        codetether_tool_registry::catalog::populate(
            &mut catalog::Native {
                registry: &mut registry,
                provider: None,
            },
            false,
            cfg!(feature = "tetherscript"),
        );
        registry
    }

    /// Create registry with provider for tools that need it (like RalphTool)
    pub fn with_provider(provider: Arc<dyn Provider>, model: String) -> Self {
        let mut registry = Self::new();
        codetether_tool_registry::catalog::populate(
            &mut catalog::Native {
                registry: &mut registry,
                provider: Some((provider, model)),
            },
            true,
            cfg!(feature = "tetherscript"),
        );
        registry
    }

    /// Create Arc-wrapped registry with batch tool properly initialized.
    /// The batch tool needs a weak reference to the registry, so we use
    /// a two-phase initialization pattern.
    pub fn with_defaults_arc() -> Arc<Self> {
        codetether_tool_registry::lifecycle::with_defaults::<registry_lifecycle::Native>()
    }

    /// Create Arc-wrapped registry with provider and batch tool properly initialized.
    /// The batch tool needs a weak reference to the registry, so we use
    /// a two-phase initialization pattern.
    #[allow(dead_code)]
    pub fn with_provider_arc(provider: Arc<dyn Provider>, model: String) -> Arc<Self> {
        codetether_tool_registry::lifecycle::with_provider::<registry_lifecycle::Native>(
            provider, model,
        )
    }
}

impl Default for ToolRegistry {
    fn default() -> Self {
        Self::with_defaults()
    }
}

impl ToolRegistry {
    /// Register the LLM-routed `search` tool.
    ///
    /// Needs the full [`crate::provider::ProviderRegistry`] because the
    /// router may talk to multiple providers to pick a model.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// use std::sync::Arc;
    /// use codetether_agent::provider::ProviderRegistry;
    /// use codetether_agent::tool::ToolRegistry;
    ///
    /// let mut registry = ToolRegistry::with_defaults();
    /// let providers = Arc::new(ProviderRegistry::from_vault().await.unwrap());
    /// registry.register_search(providers);
    /// assert!(registry.contains("search"));
    /// # });
    /// ```
    pub fn register_search(&mut self, providers: Arc<crate::provider::ProviderRegistry>) {
        self.register(Arc::new(search_router::SearchTool::new(providers)));
    }
}
