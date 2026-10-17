//! Managed child-agent prompt and result message contracts.
//!
//! [system_message] embeds workspace guidance and the [deliverable_contract].
//! [effective_error] retains execution errors and explicit non-completion.
//! [build_message_result] renders output, thinking, tool calls and warnings.
mod contract;
mod outcome;
mod result;
mod system;
pub use contract::contract as deliverable_contract;
pub use outcome::effective_error;
pub use result::build_message_result;
pub use system::build as system_message;
#[cfg(test)]
mod contract_tests;
#[cfg(test)]
mod result_tests;
#[cfg(test)]
mod system_tests;
