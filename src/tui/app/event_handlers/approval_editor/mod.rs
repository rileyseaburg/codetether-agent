//! Approval-backed source editor orchestration.
mod borrow;
mod key;

#[cfg(test)]
use codetether_approval_edit::interactive as finish;
#[cfg(test)]
#[path = "key_tests.rs"]
mod key_tests;
#[cfg(test)]
#[path = "finish_tests.rs"]
mod tests;

pub(super) use key::handle;
