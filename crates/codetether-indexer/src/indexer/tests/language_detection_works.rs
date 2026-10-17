//! Indexer behavior regression.

use super::*;

#[test]
fn language_detection_works() {
    assert_eq!(detect_language(Path::new("src/main.rs")), "rust");
    assert_eq!(detect_language(Path::new("app.py")), "python");
    assert_eq!(detect_language(Path::new("Dockerfile")), "dockerfile");
}
