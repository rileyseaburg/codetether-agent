//! TUI application state - the central `AppState` struct and its methods.
//!
//! Methods are split into focused submodules by concern:
//!
//! - `agent_profile` - spawned agent types and profile lookup
//! - `profile_defs` - named profile constants
//! - `slash_commands` - `/command` constant table
//! - `input_cursor` - cursor movement and text editing
//! - `slash_suggest` - slash autocomplete suggestion methods
//! - `scroll` - chat scroll sentinel scheme and tool-preview scroll
//! - `session_nav` - session list filtering and selection
//! - `worker_bridge` - A2A worker connection state
//! - `history` - command history up/down navigation
//! - `model_picker` - async model refresh from providers
//! - `model_picker_nav` - synchronous model picker navigation
//! - `model_store` - local filesystem cache of the model list
//! - `timing` - request latency tracking
//! - `steering` - queued steering messages
//! - `settings_nav` - settings selection and view-mode switching
//! - `message_cache` - render-line cache for performance

pub mod agent_profile;
pub mod agent_spawn_guard;
pub mod agent_tree;
pub mod approval_queue;
#[path = "latency/chat.rs"]
pub mod chat_latency;
pub use codetether_context_health as context_health;
pub mod default_impl;
pub use codetether_composer_view::state_binding::history;
pub use codetether_git_view as git_state;
pub mod history_page;
pub use codetether_composer_view::state_binding::cursor as input_cursor;
pub mod input_edit;
pub use codetether_composer_view::state_binding::replace as input_replace;
#[path = "../interlude.rs"]
pub(crate) mod interlude;
pub mod message_cache;
pub use codetether_model_picker as model_filter;
pub mod model_picker;
pub use codetether_agent_identity as profile_defs;
pub use codetether_model_picker::state_binding::navigation as model_picker_nav;
pub use codetether_model_picker::state_binding::store as model_store_sync;
pub use codetether_model_store as model_store;
pub use codetether_telemetry::state_binding::tool as pending_tool;
pub mod prompt_queue;
pub mod scroll;
pub mod scroll_page;
pub use codetether_util::fuzzy as session_fuzzy;
#[cfg(test)]
mod session_fuzzy_tests;
pub mod session_nav;
#[cfg(test)]
mod session_test_fixtures;
pub mod settings_nav;
pub use codetether_composer_view::state_binding::completion as slash_suggest;
pub use codetether_slash_command as slash_commands;
pub use codetether_slash_command as slash_hints;
pub mod streaming;
pub mod timing;
pub mod types;
pub mod view_save;
pub mod worker_bridge;

#[cfg(test)]
mod tests;

// Re-exports so external `use crate::tui::app::state::X` still works.
pub use agent_profile::{SpawnedAgent, agent_profile};
pub use profile_defs::AgentProfile;
pub use slash_commands::SLASH_COMMANDS;
pub use types::{App, AppState};

pub use crate::session::SessionEvent;
