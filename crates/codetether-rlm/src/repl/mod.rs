//! In-memory context inspection and compatibility DSL evaluation.
//!
//! [RlmRepl] owns context lines and variables. Its query methods inspect text,
//! while [RlmRepl::execute] returns a [ReplResult] and individual commands use
//! [DslResult]. Runtime labels are preserved through [ReplRuntime]; evaluation
//! here always uses the existing DSL without starting external processes.
pub use result::{DslResult, ReplResult};
pub use runtime::ReplRuntime;
pub use state::RlmRepl;
mod arguments;
mod ast;
mod context;
mod control;
mod evaluate;
mod execute;
mod expand;
mod parse;
mod pattern;
mod query_expression;
mod result;
mod runtime;
mod slice;
mod state;
mod structure_expression;
#[cfg(test)]
mod tests;
mod variables;
