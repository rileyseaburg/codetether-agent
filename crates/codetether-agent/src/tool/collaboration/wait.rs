//! Application binding for the shared collaboration wait tool.
pub(super) use codetether_collaboration_tools::WaitArgs as Args;
pub(super) type WaitAgentTool =
    codetether_collaboration_tools::WaitAgentTool<super::host::ApplicationHost>;
#[allow(non_upper_case_globals)]
pub(super) const WaitAgentTool: WaitAgentTool = WaitAgentTool::new();
#[cfg(test)]
#[path = "wait/run.rs"]
mod run;
#[cfg(test)]
#[path = "wait_tests.rs"]
mod tests;
