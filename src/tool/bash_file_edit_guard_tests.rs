//! Tests for the bash file-edit guard.

use super::file_edit_guard_reason;

#[test]
fn blocks_cat_heredoc() {
    assert!(file_edit_guard_reason("cat > src/x.rs <<'EOF'\nfoo\nEOF").is_some());
}

#[test]
fn blocks_cat_redirect() {
    assert!(file_edit_guard_reason("cat >> notes.txt").is_some());
}

#[test]
fn blocks_python_write() {
    assert!(file_edit_guard_reason("python3 - <<'EOF'\nopen('x','w')\nEOF").is_some());
    assert!(file_edit_guard_reason("python -c \"open('x', 'w').write('y')\"").is_some());
}

/// Agents route around the `.write(` check with `Path.write_text`, `shutil`,
/// or `os.replace`. Any inline program is unreviewable, so the shape itself
/// is blocked regardless of what it does.
#[test]
fn blocks_inline_interpreter_programs_even_without_write_calls() {
    for command in [
        "cd repo && python3 - <<'EOF'\nimport pathlib\npathlib.Path('x').write_text('y')\nEOF",
        "python3 -c 'import shutil; shutil.copy(\"a\",\"b\")'",
        "/usr/bin/python3.12 -c 'print(1)'",
        "perl -e 'print 1'",
        "perl -pi -e 's/a/b/' file.rs",
        "node -e 'require(\"fs\").writeFileSync(\"x\",\"y\")'",
        "ruby -e 'File.write(\"x\",\"y\")'",
    ] {
        assert!(file_edit_guard_reason(command).is_some(), "{command}");
    }
}

#[test]
fn blocks_sed_in_place() {
    assert!(file_edit_guard_reason("sed -i 's/a/b/' file.rs").is_some());
}

#[test]
fn blocks_tee_write() {
    assert!(file_edit_guard_reason("echo hi | tee out.txt").is_some());
}

#[test]
fn allows_read_only_usage() {
    assert!(file_edit_guard_reason("cat src/x.rs").is_none());
    assert!(file_edit_guard_reason("grep -n foo src/x.rs").is_none());
    assert!(file_edit_guard_reason("python3 script.py --flag").is_none());
    assert!(file_edit_guard_reason("python3 -m pytest tests/").is_none());
    assert!(file_edit_guard_reason("node scripts/build.js").is_none());
    assert!(file_edit_guard_reason("cargo test --lib mux -- --nocapture").is_none());
}
