//! File-edit guard regression.
use super::*;

#[test]
fn blocks_python_write() {
    assert!(file_edit_guard_reason("python3 - <<'EOF'\nopen('x','w')\nEOF").is_some());
    assert!(file_edit_guard_reason("python -c \"open('x', 'w').write('y')\"").is_some());
}
