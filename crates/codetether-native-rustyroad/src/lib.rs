//! Native RustyRoad tool identity and workspace authorization.
//!
//! [RustyRoadTool] connects the shared MCP backend to native runtime policy.
//! The paired type and constant preserve stateless tool construction.
#[cfg(test)]
use codetether_config as config;
#[cfg(test)]
use codetether_native_approval as approval;
use codetether_native_runtime_policy as runtime_policy;
use codetether_tool_core as tool;
mod rustyroad;
pub use rustyroad::*;
