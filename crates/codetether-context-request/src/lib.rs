//! Context-aware request construction and completion retries.
//!
//! [Host] supplies native derivation and recall. [build_request_with_context]
//! builds and guards the resulting request; [complete_with_context] invokes
//! the provider and applies the existing context-recovery option transitions.
mod complete;
mod derivation;
mod guard;
mod host;
mod request;
mod source;
pub use codetether_context_policy::derivation::options::RequestOptions;
pub use complete::complete_with_context;
pub use derivation::Derivation;
pub use guard::finish;
pub use host::Host;
pub use request::build_request_with_context;
pub use source::Source;
#[cfg(test)]
mod tests;

mod recover;

/// Prepared prompt-loop request construction and reasoning compatibility.
pub mod attempt;
