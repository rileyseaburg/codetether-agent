//! Evidence-first RLM engine.
//!
//! The engine classifies a request, gathers deterministic evidence,
//! answers oracle-friendly cases without a model, and uses the LLM only
//! for semantic synthesis over a bounded evidence pack.

mod ast;
mod bus;
mod classify;
mod complete;
mod continuation;
#[cfg(test)]
mod continuation_coverage_tests;
#[cfg(test)]
mod continuation_tests;
mod deterministic;
mod evidence;
mod file;
mod pack;
mod pattern;
mod query;
mod response;
mod run;
mod semantic;
mod trace;

pub use classify::QueryKind;
pub use run::process;
