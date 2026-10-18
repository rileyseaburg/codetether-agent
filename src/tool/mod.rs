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
use codetether_command_environment as bash_identity;
use codetether_command_shell as bash_noninteractive;
use codetether_github_auth as bash_github;
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
pub(crate) mod registry_lifecycle;
pub mod relay_autochat;
pub mod ripgrep;
pub mod rlm;
pub mod sandbox;
use codetether_sandbox_policy as sandbox_network;
use codetether_sandbox_policy as sandbox_paths;
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

/// Application registry using native tool factories and runtime policy.
pub type ToolRegistry = codetether_application_registry::ToolRegistry<registry_lifecycle::Native>;
