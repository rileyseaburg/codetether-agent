//! Live approval gate for session tool execution.

mod args;
mod gate;
#[path = "preflight/mod.rs"]
mod preflight;
mod request;
mod result;
mod review;
mod types;

pub(in crate::session::helper) use gate::gate;

#[cfg(test)]
#[path = "args_tests.rs"]
mod args_tests;

#[cfg(test)]
#[path = "gate_generic_retry_tests.rs"]
mod gate_generic_retry_tests;
#[cfg(test)]
#[path = "gate_preflight_tests.rs"]
mod gate_preflight_tests;
#[cfg(test)]
#[path = "gate_retry_tests.rs"]
mod gate_retry_tests;

#[cfg(test)]
mod request_tests;
#[cfg(test)]
#[path = "result_tests.rs"]
mod result_tests;
