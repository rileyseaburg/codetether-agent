//! Output combination and final truncation for bash results.

pub(super) fn combine(stdout: &str, stderr: &str) -> String {
    if stderr.is_empty() {
        stdout.to_string()
    } else if stdout.is_empty() {
        stderr.to_string()
    } else {
        format!("{stdout}\n--- stderr ---\n{stderr}")
    }
}

pub(super) fn truncate(
    combined: String,
    max_len: usize,
    source_bytes: usize,
    already_truncated: bool,
) -> (String, bool) {
    if combined.len() <= max_len && !already_truncated {
        return (combined, false);
    }
    let shown = crate::util::truncate_bytes_safe(&combined, max_len);
    let total = source_bytes.max(combined.len());
    (
        format!("{shown}...\n[Output truncated, {total} bytes total]"),
        true,
    )
}

pub(super) fn looks_like_auth_prompt(output: &str) -> bool {
    let lower = output.to_ascii_lowercase();
    [
        "[sudo] password for",
        "password:",
        "passphrase",
        "no tty present and no askpass program specified",
        "a terminal is required to read the password",
        "permission denied (publickey,password",
    ]
    .iter()
    .any(|needle| lower.contains(needle))
}

pub(super) fn redact(mut output: String, secrets: &[String]) -> String {
    for secret in secrets {
        if !secret.is_empty() {
            output = output.replace(secret, "[REDACTED]");
        }
    }
    output
}
