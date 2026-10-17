//! Shared patch tool with application approval regression tests.
pub(crate) use codetether_patch::proposed;
pub use codetether_patch::{ApplyPatchTool, approval_resource_from_patch};
#[cfg(test)]
mod approval_tests;
#[cfg(test)]
mod approval_verify_tests;
#[cfg(test)]
mod preview_tests;
#[cfg(test)]
mod test_support;
