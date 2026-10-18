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
pub mod budget;
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
mod register_media;
mod register_search_tools;
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

    fn register_compat_alias(&mut self, id: &str, inner: Arc<dyn Tool>) {
        self.inner.register_compat_alias(id, inner);
    }

    fn register_compat_aliases(&mut self) {
        self.register_compat_alias("patch", Arc::new(patch::ApplyPatchTool::new()));
        self.register_compat_alias("file_info", Arc::new(file_extras::FileInfoTool::new()));
        self.register_compat_alias("head_tail", Arc::new(file_extras::HeadTailTool::new()));
        self.register_compat_alias("todo_read", Arc::new(todo::TodoReadTool::new()));
        self.register_compat_alias("todo_write", Arc::new(todo::TodoWriteTool::new()));
        self.register_compat_alias("mcp_bridge", Arc::new(mcp_bridge::McpBridgeTool::new()));
        self.register_compat_alias("k8s_tool", Arc::new(k8s_tool::K8sTool::new()));
        self.register(Arc::new(computer_use::ComputerUseTool::new()));
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

        registry.register(Arc::new(file::ReadTool::new()));
        registry.register(Arc::new(file::WriteTool::new()));
        registry.register(Arc::new(file::ListTool::new()));
        registry.register(Arc::new(file::GlobTool::new()));
        registry.register(Arc::new(file_extras::TreeTool::new()));
        registry.register(Arc::new(file_extras::FileInfoTool::new()));
        registry.register(Arc::new(file_extras::HeadTailTool::new()));
        registry.register(Arc::new(file_extras::DiffTool::new()));
        registry.register(Arc::new(git::GitTool::new()));
        register_search_tools::register(&mut registry);
        registry.register(Arc::new(advanced_edit::AdvancedEditTool::new()));
        registry.register(Arc::new(edit::EditTool::new()));
        command_session::register(&mut registry, None);
        registry.register(Arc::new(lsp::LspTool::with_root(
            std::env::current_dir()
                .map(|p| format!("file://{}", p.display()))
                .unwrap_or_default(),
        )));
        registry.register(Arc::new(webfetch::WebFetchTool::new()));
        registry.register(Arc::new(multiedit::MultiEditTool::new()));
        registry.register(Arc::new(websearch::WebSearchTool::new()));
        registry.register(Arc::new(browserctl::BrowserCtlTool::new()));
        registry.register(Arc::new(patch::ApplyPatchTool::new()));
        registry.register(Arc::new(todo::TodoReadTool::new()));
        registry.register(Arc::new(todo::TodoWriteTool::new()));
        goal::register(&mut registry);
        registry.register(Arc::new(context_reset::ContextResetTool));
        registry.register(Arc::new(context_browse::ContextBrowseTool));
        registry.register(Arc::new(context_budget::ContextBudgetTool));
        registry.register(Arc::new(context_pin::ContextPinTool));
        registry.register(Arc::new(
            context_summarize::ContextSummarizeTool::cached_only(),
        ));
        registry.register(Arc::new(question::QuestionTool::new()));
        registry.register(Arc::new(task::TaskTool::new()));
        registry.register(Arc::new(plan::PlanEnterTool::new()));
        registry.register(Arc::new(plan::PlanExitTool::new()));
        registry.register(Arc::new(skill::SkillTool::new()));
        registry.register(Arc::new(memory::MemoryTool::new()));
        registry.register(Arc::new(ralph::RalphTool::new()));
        registry.register(Arc::new(prd::PrdTool::new()));
        registry.register(Arc::new(undo::UndoTool));
        registry.register(Arc::new(voice::VoiceTool::new()));
        registry.register(Arc::new(voice_input::VoiceInputTool::new()));
        registry.register(Arc::new(voice_stream::VoiceStreamTool::new()));
        register_media::register(&mut registry);
        // Edit tools with confirmation (diff display before applying)
        registry.register(Arc::new(confirm_edit::ConfirmEditTool::new()));
        registry.register(Arc::new(confirm_multiedit::ConfirmMultiEditTool::new()));
        // Swarm result sharing between sub-agents
        registry.register(Arc::new(swarm_share::SwarmShareTool::with_defaults()));
        // Register the invalid tool handler for graceful error handling
        registry.register(Arc::new(invalid::InvalidTool::new()));
        // Agent orchestration tool
        collaboration::register(&mut registry);
        // Swarm execution tool for parallel task execution
        registry.register(Arc::new(swarm_execute::SwarmExecuteTool::new()));
        // Relay autochat tool for autonomous agent communication
        registry.register(Arc::new(relay_autochat::RelayAutoChatTool::new()));
        // Go tool for autonomous OKR→PRD→Ralph pipeline
        registry.register(Arc::new(go::GoTool::new()));
        // Kubernetes management tool
        registry.register(Arc::new(k8s_tool::K8sTool::new()));
        // TetherScript plugin runtime for project-local extension hooks
        #[cfg(feature = "tetherscript")]
        tetherscript::register(&mut registry);
        registry.register_compat_aliases();

        registry
    }

    /// Create registry with provider for tools that need it (like RalphTool)
    pub fn with_provider(provider: Arc<dyn Provider>, model: String) -> Self {
        let mut registry = Self::new();

        registry.register(Arc::new(file::ReadTool::new()));
        registry.register(Arc::new(file::WriteTool::new()));
        registry.register(Arc::new(file::ListTool::new()));
        registry.register(Arc::new(file::GlobTool::new()));
        registry.register(Arc::new(file_extras::TreeTool::new()));
        registry.register(Arc::new(file_extras::FileInfoTool::new()));
        registry.register(Arc::new(file_extras::HeadTailTool::new()));
        registry.register(Arc::new(file_extras::DiffTool::new()));
        registry.register(Arc::new(git::GitTool::new()));
        register_search_tools::register(&mut registry);
        registry.register(Arc::new(advanced_edit::AdvancedEditTool::new()));
        registry.register(Arc::new(edit::EditTool::new()));
        command_session::register(&mut registry, None);
        registry.register(Arc::new(lsp::LspTool::with_root(
            std::env::current_dir()
                .map(|p| format!("file://{}", p.display()))
                .unwrap_or_default(),
        )));
        registry.register(Arc::new(webfetch::WebFetchTool::new()));
        registry.register(Arc::new(multiedit::MultiEditTool::new()));
        registry.register(Arc::new(websearch::WebSearchTool::new()));
        registry.register(Arc::new(browserctl::BrowserCtlTool::new()));
        registry.register(Arc::new(patch::ApplyPatchTool::new()));
        registry.register(Arc::new(todo::TodoReadTool::new()));
        registry.register(Arc::new(todo::TodoWriteTool::new()));
        goal::register(&mut registry);
        registry.register(Arc::new(context_reset::ContextResetTool));
        registry.register(Arc::new(context_browse::ContextBrowseTool));
        registry.register(Arc::new(context_budget::ContextBudgetTool));
        registry.register(Arc::new(context_pin::ContextPinTool));
        registry.register(Arc::new(context_summarize::ContextSummarizeTool::new(
            Arc::clone(&provider),
            model.clone(),
        )));
        registry.register(Arc::new(question::QuestionTool::new()));
        registry.register(Arc::new(task::TaskTool::new()));
        registry.register(Arc::new(plan::PlanEnterTool::new()));
        registry.register(Arc::new(plan::PlanExitTool::new()));
        registry.register(Arc::new(skill::SkillTool::new()));
        registry.register(Arc::new(memory::MemoryTool::new()));
        registry.register(Arc::new(rlm::RlmTool::new(
            Arc::clone(&provider),
            model.clone(),
            crate::rlm::RlmConfig::default(),
        )));
        registry.register(Arc::new(session_recall::SessionRecallTool::new(
            Arc::clone(&provider),
            model.clone(),
            crate::rlm::RlmConfig::default(),
        )));
        // RalphTool with provider for autonomous execution
        registry.register(Arc::new(ralph::RalphTool::with_provider(provider, model)));
        registry.register(Arc::new(prd::PrdTool::new()));
        registry.register(Arc::new(undo::UndoTool));
        registry.register(Arc::new(voice::VoiceTool::new()));
        registry.register(Arc::new(voice_input::VoiceInputTool::new()));
        registry.register(Arc::new(voice_stream::VoiceStreamTool::new()));
        register_media::register(&mut registry);
        // Edit tools with confirmation (diff display before applying)
        registry.register(Arc::new(confirm_edit::ConfirmEditTool::new()));
        registry.register(Arc::new(confirm_multiedit::ConfirmMultiEditTool::new()));
        // Swarm result sharing between sub-agents
        registry.register(Arc::new(swarm_share::SwarmShareTool::with_defaults()));
        // Register the invalid tool handler for graceful error handling
        registry.register(Arc::new(invalid::InvalidTool::new()));
        // Agent orchestration tool
        collaboration::register(&mut registry);
        // Swarm execution tool for parallel task execution
        registry.register(Arc::new(swarm_execute::SwarmExecuteTool::new()));
        // Relay autochat tool for autonomous agent communication
        registry.register(Arc::new(relay_autochat::RelayAutoChatTool::new()));
        // Go tool for autonomous OKR→PRD→Ralph pipeline
        registry.register(Arc::new(go::GoTool::new()));
        // Kubernetes management tool
        registry.register(Arc::new(k8s_tool::K8sTool::new()));
        // TetherScript plugin runtime for project-local extension hooks
        #[cfg(feature = "tetherscript")]
        tetherscript::register(&mut registry);
        registry.register_compat_aliases();

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
        let mut registry = Self::with_provider(Arc::clone(&provider), model);

        // Create batch tool without registry reference
        let batch_tool = Arc::new(batch::BatchTool::new());
        registry.register(batch_tool.clone());

        // Auto-register the LLM-routed `search` tool. Build a single-provider
        // `ProviderRegistry` from the provider we already have so the router
        // can call back into it without the full vault registry.
        let mut providers = crate::provider::ProviderRegistry::new();
        providers.register(Arc::clone(&provider));
        registry.register(Arc::new(search_router::SearchTool::new(Arc::new(
            providers,
        ))));

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
