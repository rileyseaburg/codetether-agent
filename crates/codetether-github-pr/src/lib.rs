//! Shared GitHub PR context, lifecycle policy, and body rendering.
//!
//! [context] reads repository metadata, [body] renders execution signatures,
//! [provenance_block] renders signed provenance, and [autopilot] plus [ci_watcher]
//! interpret lifecycle and CI state. Credential acquisition remains in the host;
//! [worktree] owns the explicit Git/gh CLI publication workflow.
pub mod autopilot;
pub mod body;
pub mod ci_watcher;
pub mod context;
pub mod provenance_block;

/// PR HTTP operations using caller-provided credentials.
pub mod http;

/// Worktree PR drafting and explicit publication operations.
pub mod worktree;
