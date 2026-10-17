//! Preserved LSP argument regression.
use crate::args::get_file_path_arg;
use serde_json::json;

#[test]
fn accepts_path_alias_for_file_path() {
    let args_with_file_path = json!({
        "action": "documentSymbol",
        "file_path": "src/main.rs"
    });
    assert_eq!(get_file_path_arg(&args_with_file_path), Some("src/main.rs"));

    let args_with_path = json!({
        "action": "document-symbol",
        "path": "src/lib.rs"
    });
    assert_eq!(get_file_path_arg(&args_with_path), Some("src/lib.rs"));
}
