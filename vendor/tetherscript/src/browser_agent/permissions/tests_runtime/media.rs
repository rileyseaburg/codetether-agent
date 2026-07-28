use crate::browser_agent::permissions::BrowserPermission;
use crate::browser_agent::BrowserPage;

#[test]
fn enumerate_devices_hides_labels_without_media_grants() {
    let mut page = BrowserPage::from_html("https://app.test", "");
    page.deny_permission("https://app.test", BrowserPermission::Camera);
    page.deny_permission("https://app.test", BrowserPermission::Microphone);
    let script = "let out='';navigator.mediaDevices.enumerateDevices().then(function(d){out=d.length+':'+d[0].kind+':'+d[0].label+':'+d[1].kind+':'+d[1].label;});out;";
    assert_eq!(
        page.eval_js(script).unwrap().display(),
        "2:videoinput::audioinput:"
    );
}

#[test]
fn enumerate_devices_labels_follow_media_grants() {
    let mut page = BrowserPage::from_html("https://app.test", "");
    page.grant_permission("https://app.test", BrowserPermission::Camera);
    page.grant_permission("https://app.test", BrowserPermission::Microphone);
    let script = "let out='';navigator.mediaDevices.enumerateDevices().then(function(d){out=d[0].label+'|'+d[1].label;});out;";
    assert_eq!(
        page.eval_js(script).unwrap().display(),
        "Agent Camera|Agent Microphone"
    );
}

#[test]
fn get_user_media_behavior_is_preserved() {
    let mut page = BrowserPage::from_html("https://app.test", "");
    page.grant_permission("https://app.test", BrowserPermission::Camera);
    page.grant_permission("https://app.test", BrowserPermission::Microphone);
    let script = "let out='';navigator.mediaDevices.getUserMedia({video:true,audio:true}).then(function(s){out=s.active+':'+s.constraints.video+':'+s.constraints.audio;});out;";
    assert_eq!(page.eval_js(script).unwrap().display(), "true:true:true");
}

#[test]
fn get_display_media_returns_deterministic_unsupported_denial() {
    let mut page = BrowserPage::from_html("https://app.test", "");
    let script = "let out='';navigator.mediaDevices.getDisplayMedia({video:true}).catch(function(e){out=e.name+':'+e.message;});out;";
    assert_eq!(
        page.eval_js(script).unwrap().display(),
        "NotAllowedError:display-capture unsupported"
    );
}
