const MAX_DIGEST_BYTES: usize = 4096;

pub(crate) fn compact_output(tool: &str, output: &str) -> String {
    if output.len() <= MAX_DIGEST_BYTES {
        return output.to_string();
    }
    let head = crate::util::truncate_bytes_safe(output, MAX_DIGEST_BYTES);
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
