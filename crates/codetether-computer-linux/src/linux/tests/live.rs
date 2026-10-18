//! Opt-in real X11 screenshot test; run under xvfb-run on headless CI hosts.

#[tokio::test]
#[ignore = "requires an X11 desktop and installed screenshot utility; run under xvfb-run"]
async fn live_x11_snapshot_returns_a_real_png_attachment() {
    assert!(std::env::var_os("DISPLAY").is_some());
    let result = super::super::dispatch(
        &serde_json::from_value(serde_json::json!({"action": "snapshot"})).unwrap(),
    )
    .await
    .unwrap();
    assert!(result.success, "{}", result.output);
    let data: serde_json::Value = serde_json::from_str(&result.output).unwrap();
    assert_eq!(data["session_type"], "x11");
    assert_eq!(data["width"], 1280);
    assert_eq!(data["height"], 720);
    let path = std::path::Path::new(data["path"].as_str().unwrap());
    let (png, _, _) = super::image_file::load(path).unwrap();
    assert!(!png.is_empty());
    assert!(result.metadata.contains_key("image_data_url"));
    if let Ok(evidence) = std::env::var("CODETETHER_LINUX_CAPTURE_EVIDENCE") {
        std::fs::write(evidence, &result.output).unwrap();
    }
    println!("{}", result.output);
}
