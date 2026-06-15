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
}
