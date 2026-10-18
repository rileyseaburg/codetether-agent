//! Shared managed-child checkout allocation and integration handoff.
pub(super) use codetether_child_workspace::{Handoff, allocate};
#[cfg(test)]
#[path = "workspace/tests.rs"]
mod tests;
