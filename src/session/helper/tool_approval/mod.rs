//! Live approval gate for session tool execution.

mod args;
mod gate;
mod request;
mod result;
mod types;

pub(in crate::session::helper) use gate::gate;

#[cfg(test)]
mod request_tests;
#[cfg(test)]
#[path = "result_tests.rs"]
mod result_tests;
