//! Internal helpers used by [`Session`](super::Session) and its prompt
//! loops.
//!
//! These submodules exist so the Session facade in `src/session/mod.rs` can
//! stay focused on the durable data model and public API while the agentic
//! loop, compression strategy, error classification, tool-output routing,
//! etc. live next to each other.

pub mod archive;
pub mod bootstrap;
pub mod build;
pub mod compression;
pub mod confirmation;
pub mod cost_guard;
pub mod defaults;
pub mod edit;
pub mod error;
pub mod experimental;
pub mod loop_constants;
pub mod markup;
pub mod prompt;
pub mod prompt_events;
pub mod provider;
pub mod router;
pub mod runtime;
pub mod stream;
pub mod text;
pub mod token;
pub mod validation;
