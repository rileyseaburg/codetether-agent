//! Native session and RLM adapters for shared parallel tool batches.
mod host;
mod record;
mod route;
pub(in crate::session::helper) use codetether_tool_parallel::{Batch, plan};
pub(in crate::session::helper) use record::try_execute;
#[cfg(test)]
mod image_recording_tests;
