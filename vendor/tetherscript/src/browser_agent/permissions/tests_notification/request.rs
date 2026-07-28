use super::*;

#[test]
fn notification_request_permission_calls_callback_and_thenable() {
    let script = "let seen='';window.Notification.requestPermission(function(s){seen='cb:'+s;}).then(function(s){seen=seen+':then:'+s;});seen;";
    assert_eq!(
        value(page(PermissionState::Granted), script),
        "cb:granted:then:granted"
    );
}
