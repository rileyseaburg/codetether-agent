//! Real TypeScript diagnostics must answer repeated clean edits and later errors.

use super::{LspActionResult, LspManager, path_to_uri};
use tokio::time::{Duration, timeout};

#[tokio::test]
async fn consecutive_clean_typescript_edits_do_not_wait_for_publications() {
    if which::which("typescript-language-server").is_err() {
        return;
    }
    let root = tempfile::tempdir().unwrap();
    std::fs::write(
        root.path().join("tsconfig.json"),
        r#"{"compilerOptions":{"strict":true}}"#,
    )
    .unwrap();
    let file = root.path().join("example.ts");
    let original = "export const value: string = 'original';";
    std::fs::write(&file, original).unwrap();
    let manager = LspManager::new(Some(path_to_uri(root.path())));
    let client = manager.get_client_for_file(&file).await.unwrap();
    for (content, has_errors) in [
        (original, false),
        ("export const value: string = 'changed';", false),
        ("export const value: string = 42;", true),
        ("export const value: string = 'repaired';", false),
        ("export const value = (", true),
    ] {
        let result = timeout(
            Duration::from_secs(5),
            client.diagnostics_for_content(&file, content),
        )
        .await
        .expect("diagnostics waited for a suppressed publication")
        .unwrap();
        let LspActionResult::Diagnostics { diagnostics } = result else {
            panic!("expected diagnostics");
        };
        assert_eq!(
            diagnostics
                .iter()
                .any(|item| item.severity.as_deref() == Some("error")),
            has_errors,
            "unexpected diagnostics for {content}: {diagnostics:?}"
        );
    }
    assert_eq!(std::fs::read_to_string(file).unwrap(), original);
    manager.shutdown_all().await;
}
