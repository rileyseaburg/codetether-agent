//! Assemble bounded, redacted direct-command output and metadata.
use super::bash_unsandboxed_metadata;
use super::{Call, ToolResult};
use anyhow::Result;
use codetether_github_auth::GitHubCommandAuth;
use codetether_process_capture::CapturedOutput;
use serde_json::json;
pub(super) fn finish(
    call: &Call<'_>,
    unsafe_reason: &str,
    github_auth: Option<GitHubCommandAuth>,
    output: CapturedOutput,
    max_len: usize,
) -> Result<ToolResult> {
    let exit_code = output.status.code().unwrap_or(-1);

    let (output_str, truncated, auth_prompt_blocked, success, source_len) =
        super::direct_output::prepare(&output, github_auth.as_ref(), max_len);
    super::direct_telemetry::record(call, success, exit_code, source_len, &output_str);
    Ok(bash_unsandboxed_metadata::mark(
        ToolResult {
            output: output_str,
            success,
            metadata: [
                ("exit_code".to_string(), json!(exit_code)),
                ("truncated".to_string(), json!(truncated)),
                (
                    "interactive_auth_prompt".to_string(),
                    json!(auth_prompt_blocked),
                ),
            ]
            .into_iter()
            .collect(),
        },
        call.effective_cwd.as_deref(),
        unsafe_reason,
    ))
}
