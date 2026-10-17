//! # Session Management
//!
//! A [`Session`] tracks the conversation history and execution state for a
//! single agent interaction. It is the primary abstraction used by the CLI,
//! TUI, HTTP server, and A2A worker to drive the agentic loop: send a user
//! message, let the model call tools until it converges, and return the
//! final answer.
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! # tokio::runtime::Runtime::new().unwrap().block_on(async {
//! use codetether_agent::session::Session;
//!
//! let mut session = Session::new().await.unwrap();
//! let result = session.prompt("List the files in the current directory").await.unwrap();
//! println!("Assistant: {}", result.text);
//! println!("Session ID: {}", result.session_id);
//! # });
//! ```
//!
//! ## Architecture
//!
//! Sessions are stored as JSON files in the platform data directory
//! (`~/.local/share/codetether/sessions` on Linux). Each session has a UUID,
//! an ordered list of [`Message`](crate::provider::Message)s, a record of
//! all [`ToolUse`](crate::agent::ToolUse) events, aggregated token
//! [`Usage`](crate::provider::Usage), and a [`SessionMetadata`] block.
//!
//! ## Module Layout
//!
//! The [`Session`] type is implemented across several single-responsibility
//! submodules and exposed here as a single facade:
//!
//! - [`types`] — [`Session`], [`SessionMetadata`], [`ImageAttachment`].
//! - [`events`] — [`SessionResult`], [`SessionEvent`].
//! - [`lifecycle`] — constructor, agent/provenance, message append.
//! - [`persistence`] — save / load / delete / directory lookup.
//! - [`title`] — title generation and context-change hook.
//! - [`prompt_api`] — public [`prompt`](Session::prompt) entry points.
//! - [`context`] — [`DerivedContext`] + [`derive_context`]: produce the
//!   per-step LLM context from the append-only chat history without
//!   mutating [`Session::messages`]. See the Phase A plan.
//! - [`helper`] — the agentic loop implementation (non-public details).

mod bus;
mod checkpoint;
mod checkpoint_prompt;
mod checkpoint_store;
mod event_compaction;
mod event_rlm;
mod event_token;
mod events;
mod header;
pub(crate) mod history_files;
mod lifecycle;
include!("storage_modules.rs");
mod prompt_api;
pub mod step_limit;
mod tail_load;
mod tail_seed;
mod title;
pub mod types;
mod workspace_index;
mod workspace_index_io;

pub mod codex_import;
pub mod context;
pub mod delegation;
pub mod delegation_skills;
pub mod derive_policy;
pub mod eval;
pub mod faults;
pub mod helper;
pub mod history;
pub mod history_sink;
pub mod index;
pub mod index_produce;
pub mod journal;
pub mod listing;
mod listing_all;
pub mod oracle;
pub mod oracle_replay;
pub mod pages;
pub mod rate_gate;
pub mod relevance;
pub mod runtime;
pub mod tasks;
pub mod thread_events;
pub mod thread_store;

mod exports;
pub use exports::*;

#[cfg(test)]
mod checkpoint_tests;
#[cfg(test)]
mod tests;
