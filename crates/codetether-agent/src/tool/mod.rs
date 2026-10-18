//! Tool system
//!
//! Tools are the executable capabilities available to agents.

pub use codetether_advanced_edit as advanced_edit;
pub mod agent;
pub use codetether_avatar_tool as avatar;
pub use codetether_native_edit::auto_apply;
pub use codetether_tool_core as alias;
include!("command_modules.rs");
use codetether_command_environment as bash_identity;
pub use codetether_command_guard as bash_file_edit_guard;
use codetether_command_shell as bash_noninteractive;
use codetether_github_auth as bash_github;
pub(crate) mod bash_shell;
pub mod batch;
pub use codetether_browser_tool as browserctl;
pub use codetether_bus_tool as bus_inspect;
pub use codetether_code_search as codesearch;
pub use codetether_computer_tool as computer_use;
pub use codetether_confirm_edit as confirm_edit;
pub use codetether_confirm_multiedit as confirm_multiedit;
pub use codetether_tool_core::budget;
pub mod context_browse;
pub mod context_budget;
pub(super) mod context_helpers;
pub mod context_pin;
pub use codetether_context_reset as context_reset;
pub mod context_summarize;
pub use codetether_file_tools as file;
pub use codetether_native_edit::edit;
pub(crate) use codetether_tool_core::feedback;
pub mod file_extras;
pub use codetether_git_tool as git;
pub mod go;
include!("image_modules.rs");
pub use codetether_k8s_tool as k8s_tool;
pub use codetether_native_lsp::tool as lsp;
pub use codetether_tool_core as invalid;
include!("mcp_modules.rs");
mod catalog;
pub mod goal;
pub use codetether_native_edit::morph_backend;
pub use codetether_native_edit::multiedit;
pub use codetether_native_mcp_client::mcp_tools;
pub use codetether_native_memory as memory;
pub use codetether_okr_tool as okr;
pub use codetether_patch as patch;
pub use codetether_plan as plan;
pub use codetether_podcast_tool as podcast;
pub use codetether_prd_tool as prd;
pub use codetether_tool_core::profile;
pub mod progress;
pub use codetether_question_tool as question;
pub mod ralph;
pub use codetether_tool_core::readonly;
pub(crate) mod registry_lifecycle;
pub use codetether_relay_tool as relay_autochat;
pub use codetether_ripgrep_tool as ripgrep;
pub mod rlm;
pub use codetether_grep_tool as search;
pub use codetether_native_search::tool as search_router;
use codetether_sandbox_policy as sandbox_network;
use codetether_sandbox_policy as sandbox_paths;
pub use codetether_sandbox_runtime as sandbox;
pub mod session_recall;
pub use codetether_skill_tool as skill;
pub mod swarm_execute;
pub use codetether_native_edit::undo;
#[cfg(feature = "tetherscript")]
pub use codetether_native_tetherscript as tetherscript;
pub use codetether_swarm_share as swarm_share;
pub use codetether_task_tool as task;
pub use codetether_todo as todo;
pub use codetether_voice_input_tool as voice_input;
pub use codetether_voice_stream_tool as voice_stream;
pub use codetether_voice_tool as voice;
pub use codetether_webfetch as webfetch;
pub use codetether_websearch as websearch;
pub use codetether_youtube_tool as youtube;

use std::sync::Arc;

use crate::provider::Provider;
pub use budget::{DEFAULT_TOOL_OUTPUT_MAX_BYTES, tool_output_budget};
pub use mcp_tools::{McpToolManager, McpToolWrapper};
pub use sandbox::{PluginManifest, PluginRegistry, SigningKey, hash_bytes, hash_file};

pub use codetether_tool_core::{Tool, ToolResult};

/// Application registry using native tool factories and runtime policy.
pub type ToolRegistry = codetether_application_registry::ToolRegistry<registry_lifecycle::Native>;
