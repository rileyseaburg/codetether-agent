use super::{LspActionResult, client::LspClient, path_to_uri};

#[tokio::test]
async fn proposed_content_uses_real_rust_analyzer() {
    if std::process::Command::new("rust-analyzer")
        .arg("--version")
        .output()
        .is_err()
    {
        return;
    }
    let dir = tempfile::tempdir().unwrap();
    let src = dir.path().join("src");
    std::fs::create_dir(&src).unwrap();
    std::fs::write(
        dir.path().join("Cargo.toml"),
        "[package]\nname='approval-lsp-test'\nversion='0.1.0'\nedition='2024'\n",
    )
    .unwrap();
    let file = src.join("lib.rs");
    std::fs::write(&file, "pub fn valid() {}\n").unwrap();
    let client = LspClient::for_language("rust", Some(path_to_uri(dir.path())))
        .await
        .unwrap();
    client.initialize().await.unwrap();

    let result = client
        .diagnostics_for_content(&file, "pub fn broken( {\n")
        .await
        .unwrap();

    let LspActionResult::Diagnostics { diagnostics } = result else {
        panic!("wrong result")
    };
    assert!(
        !diagnostics.is_empty(),
        "rust-analyzer returned no diagnostics"
    );
}
