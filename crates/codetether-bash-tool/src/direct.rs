//! Execute a prepared direct command with bounded capture.
use super::{Call, ToolResult};
use anyhow::Result;
use codetether_process_capture as bash_capture;
pub(super) async fn execute(call: Call<'_>, unsafe_reason: &str) -> Result<ToolResult> {
    let (cmd, github_auth) = super::direct_command::prepare(&call).await;
    let max_len = codetether_tool_core::tool_output_budget();
    let result = bash_capture::run(cmd, call.timeout_secs, max_len).await;

    match result {
        Ok(bash_capture::CaptureOutcome::Finished(output)) => {
            super::direct_finished::finish(&call, unsafe_reason, github_auth, output, max_len)
        }
        Err(e) => super::direct_error::finish(&call, unsafe_reason, e),
        Ok(bash_capture::CaptureOutcome::TimedOut) => {
            super::direct_timeout::finish(&call, unsafe_reason)
        }
    }
}
