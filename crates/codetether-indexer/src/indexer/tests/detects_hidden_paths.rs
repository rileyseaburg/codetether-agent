//! Indexer behavior regression.

use super::*;

#[test]
fn detects_hidden_paths() {
    assert!(is_hidden_path(Path::new(".git/config")));
    assert!(is_hidden_path(Path::new("src/.cache/file")));
    assert!(!is_hidden_path(Path::new("src/main.rs")));
}
