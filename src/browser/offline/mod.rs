//! TetherScript-backed offline browser primitives.
//!
//! Net-new capability surface introduced for tetherscript v0.1.0-alpha.12.
//! See `src/cli/browserctl/offline.rs` for the CLI dispatch.

pub mod auth_trace;
pub(crate) mod auth_trace_run;
#[cfg(feature = "tetherscript")]
pub(crate) mod auth_trace_serde;
pub mod cookie_diff;
pub(crate) mod cookie_diff_index;
pub mod cookie_parse;
pub mod explain_cors;
pub(crate) mod explain_cors_analyse;
pub mod record;
pub mod replay;

#[cfg(test)]
mod tests;
#[cfg(test)]
mod tests_helpers;
#[cfg(test)]
mod tests_net;
