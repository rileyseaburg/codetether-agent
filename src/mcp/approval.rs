//! Shared MCP approval endpoint bindings and native integration tests.
pub(super) use codetether_mcp_approval::{capability, handle};

#[cfg(test)]
#[path = "approval/session_decision_tests.rs"]
mod session_decision_tests;
#[cfg(test)]
mod tests;
