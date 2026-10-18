//! Application binding for shared structured child-input submission.
pub(super) type SendInputTool =
    codetether_collaboration_tools::SendInputTool<super::host::ApplicationHost>;
#[allow(non_upper_case_globals)]
pub(super) const SendInputTool: SendInputTool = SendInputTool::new();
#[cfg(test)]
use codetether_collaboration_tools as args;
#[cfg(test)]
#[path = "send_input/input.rs"]
mod input;
