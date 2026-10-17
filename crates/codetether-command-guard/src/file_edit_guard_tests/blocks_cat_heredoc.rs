//! File-edit guard regression.
use super::*;

#[test]
fn blocks_cat_heredoc() {
    assert!(file_edit_guard_reason("cat > src/x.rs <<'EOF'\nfoo\nEOF").is_some());
}
