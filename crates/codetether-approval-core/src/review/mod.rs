//! Review contracts, prompt assembly, and tolerant verdict parsing.
//!
//! [ReviewSubject] describes a proposed action. [prompt::user] builds its
//! review request, and [parse] extracts a [ReviewVerdict] from model output.
//! These helpers do not execute agents or authorize actions.
//!
//! # Examples
//! ```
//! use codetether_approval_core::review::{parse, ReviewOutcome};
//! assert_eq!(parse("no verdict").outcome, ReviewOutcome::Escalate);
//! ```
mod parse;
pub mod prompt;
mod verdict;
pub use parse::parse;
pub use prompt::ReviewSubject;
pub use verdict::{ReviewOutcome, ReviewVerdict};
#[cfg(test)]
mod tests;
