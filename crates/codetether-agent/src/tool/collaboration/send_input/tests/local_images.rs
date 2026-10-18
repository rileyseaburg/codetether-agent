//! Local-image paths use the trusted parent workspace, not the process cwd.
use super::super::{args::Args, input};
use serde_json::json;

#[tokio::test]
async fn resolves_relative_and_absolute_images_from_runtime_context() {
    let parent = tempfile::tempdir().unwrap();
    let relative = std::path::Path::new("child-input-test-image.png");
    let absolute = parent.path().join(relative);
    tokio::fs::write(&absolute, [0_u8]).await.unwrap();
    for path in [relative, absolute.as_path()] {
        let args: Args = serde_json::from_value(json!({
            "target":"child", "__ct_parent_workspace":parent.path(),
            "items":[{"type":"local_image", "path":path}]
        }))
        .unwrap();
        let root = args.context.workspace().unwrap();
        assert_eq!(root, parent.path());
        assert_ne!(root, std::env::current_dir().unwrap());
        let prepared = input::prepare_in_workspace(args.message, args.items, Some(root))
            .await
            .unwrap();
        assert_eq!(prepared.images.len(), 1);
        assert_eq!(prepared.images[0].data_url, "data:image/png;base64,AA==");
        assert_eq!(
            prepared.message,
            format!("[Image attached: {}]", path.display())
        );
    }
}

#[tokio::test]
async fn missing_and_unsupported_local_images_are_errors() {
    let parent = tempfile::tempdir().unwrap();
    for path in ["missing.png", "unsupported.txt"] {
        let args: Args = serde_json::from_value(json!({
            "target":"child", "items":[{"type":"text", "text":"inspect"},
                {"type":"local_image", "path":path}]
        }))
        .unwrap();
        assert!(args.context.workspace().is_none());
        assert!(
            input::prepare_in_workspace(args.message, args.items, Some(parent.path()))
                .await
                .is_err()
        );
    }
}
