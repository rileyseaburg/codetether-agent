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
//! - [`helper`] — the agentic loop implementation (non-public details).

mod events;
mod lifecycle;
mod persistence;
mod prompt_api;
mod title;
mod types;

pub mod codex_import;
pub mod helper;
mod listing;
mod listing_all;

pub use self::codex_import::{
    import_codex_session_by_id, import_codex_sessions_for_directory, load_or_import_session,
};
pub use self::events::{SessionEvent, SessionResult};
pub use self::listing::{SessionSummary, list_sessions};
pub use self::listing_all::list_all_sessions_for_directory;
pub use self::types::{DEFAULT_MAX_STEPS, ImageAttachment, Session, SessionMetadata};

#[cfg(test)]
mod tests;
