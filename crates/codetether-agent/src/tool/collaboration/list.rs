//! Application binding for the shared list tool.
pub(super) type ListAgentsTool =
    codetether_collaboration_tools::ListAgentsTool<super::host::ApplicationHost>;
#[allow(non_upper_case_globals)]
pub(super) const ListAgentsTool: ListAgentsTool = ListAgentsTool::new();
#[cfg(test)]
use codetether_collaboration_tools::ListArgs as Args;
#[cfg(test)]
#[path = "list_tests.rs"]
mod tests;
