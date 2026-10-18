//! State-independent native terminal UI modules.
//!
//! [chat] exposes message, copy and Mermaid support. [agent_identity],
//! [constants] and [utils] share frontend presentation contracts. [lsp] and
//! [terminal] render panels without requiring the application state type.
pub mod agent_identity;
pub mod chat;
pub mod constants;
pub mod lsp;
pub mod terminal;
pub mod utils;

/// Interactive background execution with application-selected backends.
pub mod runs;
