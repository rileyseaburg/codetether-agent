use super::render_section;
use tempfile::tempdir;

#[test]
fn renders_language_model_tool_contracts() {
    let tmp = tempdir().expect("tempdir");
    std::fs::create_dir(tmp.path().join(".git")).expect("git marker");
    std::fs::write(
        tmp.path().join("package.json"),
        r#"{
          "contributes": {
            "languageModelTools": [{
              "name": "rustyRefactor_refactor_file",
              "toolReferenceName": "refactor_rust_file",
              "userDescription": "Refactor a Rust file",
              "inputSchema": {"required": ["filePath"]}
            }]
          }
        }"#,
    )
    .expect("package");
    let section = render_section(tmp.path());
    assert!(section.contains("rustyRefactor_refactor_file"));
    assert!(section.contains("refactor_rust_file"));
    assert!(section.contains("Required: filePath"));
}
