//! Wayland capture must not silently fall back to an XWayland root screenshot.

use super::*;
use std::time::Duration;

#[tokio::test]
async fn wayland_commands_remove_display_and_force_native_toolkits() {
    let original_display = std::env::var_os("DISPLAY");
    let dir = tempfile::tempdir().unwrap();
    let target = dir.path().join("environment.txt");
    let program = script(
        dir.path(),
        "capture environment",
        "printf '%s|%s|%s' \"${DISPLAY-unset}\" \"$GDK_BACKEND\" \"$QT_QPA_PLATFORM\" > \"$2\"",
    );
    command::run(
        Session::Wayland,
        Backend::Gnome,
        &program,
        &target,
        Duration::from_secs(2),
    )
    .await
    .unwrap();
    assert_eq!(
        std::fs::read_to_string(target).unwrap(),
        "unset|wayland|wayland"
    );
    assert_eq!(std::env::var_os("DISPLAY"), original_display);
}
