//! Capture failures explain missing sessions, utilities, and compositor permissions.

use super::*;

#[tokio::test]
async fn capture_reports_headless_and_missing_utility_errors() {
    let error = snapshot::capture(Session::Headless, &[]).await.unwrap_err();
    assert!(error.to_string().contains("graphical session"));
    let error = snapshot::capture(Session::Wayland, &[]).await.unwrap_err();
    assert!(
        error
            .to_string()
            .contains("No compatible Linux screenshot utility")
    );
}

#[tokio::test]
async fn all_failed_backends_report_their_names_and_permissions_hint() {
    let dir = tempfile::tempdir().unwrap();
    let bad = script(dir.path(), "denied", "exit 7");
    let error = snapshot::capture(Session::Wayland, &[(Backend::Gnome, bad)])
        .await
        .unwrap_err();
    let message = error.to_string();
    assert!(message.contains("gnome-screenshot"));
    assert!(message.contains("7"));
    assert!(message.contains("Wayland"));
    assert!(message.contains("permission"));
}
