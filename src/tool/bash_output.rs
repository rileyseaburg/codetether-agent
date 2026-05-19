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
