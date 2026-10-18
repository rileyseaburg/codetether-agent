//! Compatibility exports for shared policy results and tool-name evaluation.

pub use codetether_runtime_policy::blocking::blocking_result;
pub use codetether_runtime_policy::evaluate_tool::{evaluate_tool, evaluate_tool_with_config};
#[cfg(test)]
pub use codetether_runtime_policy::result_approval::blocking_result_with_approval_request;
