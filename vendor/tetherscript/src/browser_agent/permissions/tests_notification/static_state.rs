use super::*;

#[test]
fn notification_static_permission_tracks_states() {
    assert_eq!(
        value(
            page(PermissionState::Granted),
            "window.Notification.permission;"
        ),
        "granted"
    );
    assert_eq!(
        value(
            page(PermissionState::Prompt),
            "window.Notification.permission;"
        ),
        "prompt"
    );
    assert_eq!(
        value(
            page(PermissionState::Denied),
            "window.Notification.permission;"
        ),
        "denied"
    );
}
