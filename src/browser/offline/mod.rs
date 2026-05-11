//! TetherScript-backed offline browser primitives.
//!
//! Net-new capability surface introduced for tetherscript v0.1.0-alpha.12.
//! See `src/cli/browserctl/offline.rs` for the CLI dispatch.

pub mod auth_trace;
pub mod cookie_diff;
pub mod cookie_parse;
pub mod explain_cors;
pub mod record;
pub mod replay;

#[cfg(test)]
mod tests;
#[cfg(test)]
mod tests_net;
