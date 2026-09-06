//! Parser IDs must not change server-family routing or user override keys.

use std::path::Path;

#[test]
fn jsx_documents_use_react_ids_without_changing_server_families() {
    for (path, document, family) in [
        ("src/MetadataInspector.tsx", "typescriptreact", "typescript"),
        ("src/LayerPicker.jsx", "javascriptreact", "javascript"),
        ("src/types.ts", "typescript", "typescript"),
        ("src/helpers.js", "javascript", "javascript"),
    ] {
        assert_eq!(super::id(Path::new(path)), document, "{path}");
        assert_eq!(crate::lsp::detect_language_from_path(path), Some(family));
        let config = crate::lsp::get_language_server_config(family).unwrap();
        assert_eq!(config.command, "typescript-language-server");
    }
}

#[test]
fn other_document_languages_keep_their_existing_ids() {
    for (path, expected) in [
        ("src/main.rs", "rust"),
        ("src/main.py", "python"),
        ("src/main.go", "go"),
        ("src/main.cpp", "cpp"),
        ("notes.unknown", "plaintext"),
        ("README", "plaintext"),
    ] {
        assert_eq!(super::id(Path::new(path)), expected);
    }
}
