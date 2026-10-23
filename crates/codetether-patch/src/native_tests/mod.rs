//! Shared patch tool with application approval regression tests.
pub(crate) use crate::proposed;
pub use crate::{ApplyPatchTool, approval_resource_from_patch};
#[cfg(test)]
mod approval_tests;
#[cfg(test)]
mod approval_verify_tests;
#[cfg(test)]
mod preview_tests;
#[cfg(test)]
mod test_support;
#[cfg(test)]
mod write_guard_tests;
