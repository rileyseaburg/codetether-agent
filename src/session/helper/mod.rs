//! Internal helpers used by [`Session`](super::Session) and its prompt
//! loops.
//!
//! These submodules exist so the Session facade in `src/session/mod.rs` can
//! stay focused on the durable data model and public API while the agentic
//! loop, compression strategy, error classification, tool-output routing,
//! etc. live next to each other.

pub mod archive;
pub mod bare_json;
pub mod bootstrap;
pub mod build;
pub mod compression;
pub mod confirmation;
pub mod cost_guard;
pub mod defaults;
pub mod edit;
pub mod error;
pub mod event_payload;
mod evidence;
pub mod experimental;
mod live_bus;
pub mod loop_constants;
pub mod markup;
mod persist;
pub mod persistence_cap;
pub mod prompt;
pub mod prompt_call;
pub mod prompt_events;
mod prompt_loop;
pub(crate) mod prompt_too_long;
pub mod provider;
pub(crate) mod publish_user_prompt;
pub mod recall_context;
mod refactor_guard;
mod repeat_guard;
pub mod request_state;
mod retry_error;
pub(crate) mod rlm_background;
mod rlm_model;
pub mod router;
pub mod runtime;
pub(crate) mod steering;
pub mod stream;
pub mod stream_caps;
pub mod text;
pub mod token;
mod tool_modules;
pub(in crate::session::helper) use tool_modules::*;
mod step_begin;
mod step_model_restore;
mod tool_heartbeat;
mod usage_record;
pub mod validation;
mod workspace_tools;

#[cfg(test)]
include!("tests_manifest.rs");
