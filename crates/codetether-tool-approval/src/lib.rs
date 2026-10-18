//! Tool approval decisions independent of application session ownership.
//!
//! Implement [Host] with application services, then call [gate]. [ApprovalGate]
//! returns approved arguments or a blocked tool result, retaining diagnostic
//! warnings. [annotate] attaches those warnings to the eventual execution result.
mod args;
mod gate;
mod host;
mod request;
mod result;
mod types;
mod warnings;
pub use gate::gate;
pub use host::Host;
pub use types::ApprovalGate;
pub use warnings::annotate;
#[cfg(test)]
mod args_tests;
#[cfg(test)]
mod request_tests;
#[cfg(test)]
mod result_tests;
