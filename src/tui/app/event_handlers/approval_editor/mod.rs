//! Approval-backed source editor orchestration.

mod close;
mod finish;
mod key;
#[cfg(test)]
#[path = "key_tests.rs"]
mod key_tests;
#[cfg(test)]
#[path = "orphan_tests.rs"]
mod orphan_tests;
#[cfg(test)]
#[path = "finish_tests.rs"]
mod tests;

pub(super) use key::handle;
