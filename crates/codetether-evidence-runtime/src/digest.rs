//! Bound tool text while retaining an explicit full-artifact reminder.
const MAX_DIGEST_BYTES: usize = 4096;

/// Bound a tool-output digest while naming the original size and artifact duty.
/// # Arguments
/// `tool` labels the output; `output` is the complete tool text.
/// # Returns
/// Original text up to the limit, otherwise a UTF-8-safe prefix and notice.
/// # Examples
/// ```
/// assert_eq!(codetether_evidence_runtime::digest::compact_output("read", "short"), "short");
/// ```
pub fn compact_output(tool: &str, output: &str) -> String {
    if output.len() <= MAX_DIGEST_BYTES {
        return output.to_string();
    }
    let head = codetether_util::truncate_bytes_safe(output, MAX_DIGEST_BYTES);
    format!(
        "{head}\n[runtime digest: {tool} output was {} bytes; preserve full artifact externally]",
        output.len()
    )
}

#[cfg(test)]
mod tests {
    use super::compact_output;

    #[test]
    fn compact_marks_large_output() {
        let out = compact_output("read", &"x".repeat(5000));
        assert!(out.contains("runtime digest"));
    }
}
