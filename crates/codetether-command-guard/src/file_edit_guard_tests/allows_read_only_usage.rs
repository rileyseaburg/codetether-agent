//! File-edit guard regression.
use super::*;

#[test]
fn allows_read_only_usage() {
    assert!(file_edit_guard_reason("cat src/x.rs").is_none());
    assert!(file_edit_guard_reason("grep -n foo src/x.rs").is_none());
    assert!(file_edit_guard_reason("python3 script.py --flag").is_none());
    assert!(file_edit_guard_reason("python3 -m pytest tests/").is_none());
    assert!(file_edit_guard_reason("node scripts/build.js").is_none());
    assert!(file_edit_guard_reason("cargo test --lib mux -- --nocapture").is_none());
}
