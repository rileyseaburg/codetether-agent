//! Internal helpers used by [`Session`](super::Session) and its prompt
//! loops.
//!
//! These submodules exist so the Session facade in `src/session/mod.rs` can
//! stay focused on the durable data model and public API while the agentic
//! loop, compression strategy, error classification, tool-output routing,
//! etc. live next to each other.

pub mod archive;
pub use codetether_build_policy as build;
pub use codetether_tool_call_parser as bare_json;
pub use codetether_tool_catalog as bootstrap;
pub mod compression;
pub mod confirmation;
mod context_refresh;
pub use codetether_cost_guard as cost_guard;
pub mod defaults;
pub use codetether_edit_policy as edit;
pub mod error;
pub mod event_payload;
mod evidence;
pub use codetether_context_compression::experimental;
pub mod loop_constants;
pub use codetether_tool_call_parser as markup;
mod persist;
pub use codetether_snapshot_io as persistence_cap;
pub mod prompt;
pub mod prompt_call;
pub mod prompt_events;
mod prompt_loop;
pub(crate) mod prompt_too_long;
pub use codetether_provider_policy as provider;
pub(crate) mod publish_user_prompt;
pub mod recall_context;
pub mod request_state;
use codetether_provider_policy as retry_error;
pub(crate) mod rlm_background;
pub mod router;
pub mod runtime;
pub mod steering;
pub mod stream;
pub use codetether_stream_bounds as stream_caps;
pub mod text;
pub mod token;
mod tool_modules;
pub(in crate::session::helper) use tool_modules::*;
mod step_begin;
mod step_model_restore;
pub mod validation;
mod workspace_coordination;
mod workspace_tools;

#[cfg(test)]
include!("tests_manifest.rs");
