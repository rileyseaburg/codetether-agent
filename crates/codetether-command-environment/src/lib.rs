//! Command environment preparation from provenance, context and GitHub auth.
//!
//! [resolve] combines Git identity, runtime context and scoped credentials.
//! [Environment] keeps environment entries separate from output redactions.
mod environment;
mod identity;
pub use environment::{Environment, resolve};
pub use identity::git_identity_env_from_tool_args;
