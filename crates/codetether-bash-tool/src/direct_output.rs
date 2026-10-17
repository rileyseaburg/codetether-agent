//! Redact credentials, detect blocked prompts and truncate captured output.
use super::{bash_output, looks_like_auth_prompt, redact_output};
use codetether_github_auth::GitHubCommandAuth;
use codetether_process_capture::CapturedOutput;
pub(super) fn prepare(
    output: &CapturedOutput,
    github_auth: Option<&GitHubCommandAuth>,
    max_len: usize,
) -> (String, bool, bool, bool, usize) {
    let combined = bash_output::combine(&output.stdout.text, &output.stderr.text);
    let combined = redact_output(
        combined,
        github_auth
            .map(|auth| auth.redactions.as_slice())
            .unwrap_or(&[]),
    );

    let success = output.status.success();
    let auth_prompt_blocked = !success && looks_like_auth_prompt(&combined);

    if auth_prompt_blocked {
        tracing::warn!(target: "codetether_agent::tool::bash", "Interactive auth prompt detected in output");
    }
    let combined = if auth_prompt_blocked {
        format!(
            "{combined}\n\n[CodeTether] Interactive password prompts are disabled for agent-run commands. Use non-interactive credentials or commands such as `sudo -n`."
        )
    } else {
        combined
    };

    let source_len = output.total_bytes();
    let (output_str, truncated) =
        bash_output::truncate(combined, max_len, source_len, output.truncated());

    (
        output_str,
        truncated,
        auth_prompt_blocked,
        success,
        source_len,
    )
}
