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
mod event_compaction;
mod event_rlm;
mod event_token;
mod events;
mod header;
pub(crate) mod history_files;
mod lifecycle;
mod persistence;
mod prompt_api;
mod tail_load;
mod tail_seed;
mod title;
mod types;
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
pub mod journal;
pub mod listing;
mod listing_all;
pub mod oracle;
pub mod pages;
pub mod relevance;
pub mod tasks;

pub use self::bus::{DurableSink, NoopSink, SessionBus};
pub use self::codex_import::{
    import_codex_session_by_id, import_codex_sessions_for_directory, load_or_import_session,
};
pub use self::context::{DerivedContext, derive_context, derive_with_policy, effective_policy};
pub use self::delegation::{BetaPosterior, DelegationConfig, DelegationState};
pub use self::derive_policy::DerivePolicy;
pub use self::eval::{PolicyRunResult, pareto_frontier, reuse_rate};
pub use self::event_compaction::{
    CompactionFailure, CompactionOutcome, CompactionStart, ContextTruncation, FallbackStrategy,
};
pub use self::event_rlm::{RlmCompletion, RlmOutcome, RlmProgressEvent, RlmSubcallFallback};
pub use self::event_token::{TokenDelta, TokenEstimate, TokenSource};
pub use self::events::{SessionEvent, SessionResult};
pub use self::faults::Fault;
pub use self::history::History;
pub use self::history_sink::{HistorySinkConfig, PointerHandle};
pub use self::index::{Granularity, SummaryIndex, SummaryNode, SummaryRange};
pub use self::journal::{JournalEntry, Op, RejectReason, TxnId, WritebackJournal};
pub use self::listing::{SessionSummary, list_sessions};
pub use self::listing_all::list_all_sessions_for_directory;
pub use self::oracle::{OracleReport, replay_oracle};
pub use self::pages::{PageKind, ResidencyLevel};
pub use self::relevance::{
    Bucket, Dependency, Difficulty, RelevanceMeta, ToolUse, bucket_for_messages,
};
pub use self::tail_load::TailLoad;
pub use self::tasks::{TaskEvent, TaskLog, TaskState, TaskStatus};
pub use self::types::{DEFAULT_MAX_STEPS, ImageAttachment, Session, SessionMetadata};

#[cfg(test)]
mod tests;
