use super::*;

#[test]
fn notification_constructor_exposes_fields_and_denial_state() {
    let fields = "let n=new Notification('Hi',{body:'Body',tag:'t',data:{id:7},silent:true,renotify:true,requireInteraction:true});[n.title,n.body,n.tag,n.data.id,n.silent,n.renotify,n.requireInteraction,n.permission].join('|');";
    let denied = "let n=new Notification('No');n.permission+':'+n.permissionDenied+':'+n.error.name+':'+n.error.message;";
    assert_eq!(
        value(page(PermissionState::Granted), fields),
        "Hi|Body|t|7|true|true|true|granted"
    );
    assert_eq!(
        value(page(PermissionState::Denied), denied),
        "denied:true:NotAllowedError:notifications denied"
    );
}
