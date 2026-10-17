use super::append;

#[test]
fn writes_fragments_to_the_capture_directory() {
    let dir = tempfile::tempdir().expect("tempdir");
    let path = dir.path().to_str().expect("utf-8 path");

    // Exercise `append` directly: `record` reads a process-wide env var, which
    // races against other tests in the same binary.
    append(path, "data: {\"choices\":[]}\n");
    append(path, "data: [DONE]\n");

    let captured: String = std::fs::read_dir(dir.path())
        .expect("readable")
        .filter_map(Result::ok)
        .map(|entry| std::fs::read_to_string(entry.path()).unwrap_or_default())
        .collect();

    assert!(captured.contains("choices"));
    assert!(captured.contains("[DONE]"));
}

#[test]
fn blank_directory_value_is_ignored() {
    // Must not panic or create stray files when the variable is whitespace.
    append("   ", "data: x\n");
}
