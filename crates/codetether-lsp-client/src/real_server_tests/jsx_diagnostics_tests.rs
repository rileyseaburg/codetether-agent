//! Real TypeScript server coverage for disk inspection and proposed React edits.

use super::{LspActionResult, LspManager, path_to_uri};

#[path = "jsx_test_support.rs"]
mod support;
use support::{assert_no_errors, available};

#[tokio::test]
async fn jsx_document_diagnostics_use_react_parser_for_disk_and_proposals() {
    if !available() {
        return;
    }
    for extension in ["tsx", "jsx"] {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(
            dir.path().join("tsconfig.json"),
            r#"{"compilerOptions":{"jsx":"preserve","allowJs":true,"noEmit":true}}"#,
        )
        .unwrap();
        let path = dir.path().join(format!("LayerPicker.{extension}"));
        let original = "export const LayerPicker = () => <section><span>Layers</span></section>;\n";
        std::fs::write(&path, original).unwrap();
        let manager = LspManager::new(Some(path_to_uri(dir.path())));
        let client = manager.get_client_for_file(&path).await.unwrap();
        let on_disk = client.diagnostics(&path).await.unwrap();
        assert_no_errors(on_disk);
        let broken = "export const MetadataInspector = () => <section><span></section>;\n";
        let invalid = client.diagnostics_for_content(&path, broken).await.unwrap();
        let LspActionResult::Diagnostics { diagnostics } = invalid else {
            panic!("diagnostics expected")
        };
        assert!(
            diagnostics
                .iter()
                .any(|d| d.severity.as_deref() == Some("error"))
        );
        // The server suppresses repeated empty publications. Change diagnostics
        // before testing repair so the assertion requires an actual publication.
        let proposed =
            "export const MetadataInspector = () => <section><span>Metadata</span></section>;\n";
        let preview = client
            .diagnostics_for_content(&path, proposed)
            .await
            .unwrap();
        assert_no_errors(preview);
        assert_eq!(std::fs::read_to_string(&path).unwrap(), original);
        manager.shutdown_all().await;
    }
}
