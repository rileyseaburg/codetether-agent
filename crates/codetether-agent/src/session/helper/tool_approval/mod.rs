//! Native services for the shared tool approval flow.
mod gate;
mod host;
#[path = "preflight/mod.rs"]
mod preflight;
pub(in crate::session::helper) use codetether_tool_approval::annotate;
pub(in crate::session::helper) use gate::gate;
#[cfg(test)]
#[path = "gate_preflight_tests.rs"]
mod gate_preflight_tests;
