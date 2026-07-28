use super::*;
use crate::browser_agent::BrowserPage;

#[path = "tests_runtime/clipboard.rs"]
mod clipboard;
#[path = "tests_runtime/media.rs"]
mod media;

#[test]
fn geolocation_bridge_returns_position_or_denial() {
    let mut page = BrowserPage::from_html("https://app.test", "<p id='out'></p>");
    page.grant_permission("https://app.test", BrowserPermission::Geolocation);
    page.set_geolocation(GeolocationPosition::new(12.0, 34.0, 5.0).unwrap());
    let script = "navigator.geolocation.getCurrentPosition(function(p){document.getElementById('out').textContent='lat:'+p.coords.latitude;},function(e){document.getElementById('out').textContent='err:'+e.code;});";
    page.eval_js(script).unwrap();
    assert!(page.session.html.contains("lat:12"));
    page.deny_permission("https://app.test", BrowserPermission::Geolocation);
    let denied = "navigator.geolocation.getCurrentPosition(function(){},function(e){document.getElementById('out').textContent='err:'+e.code;});";
    page.eval_js(denied).unwrap();
    assert!(page.session.html.contains("err:1"));
}

#[test]
fn permission_query_and_clipboard_bridge_use_configured_state() {
    let mut page = BrowserPage::from_html("https://app.test", "<p id='out'></p>");
    page.grant_permission("https://app.test", BrowserPermission::ClipboardWrite);
    page.grant_permission("https://app.test", BrowserPermission::ClipboardRead);
    let script = "navigator.clipboard.writeText('secret');navigator.permissions.query({name:'clipboard-read'}).then(function(p){document.getElementById('out').textContent=p.state;});";
    page.eval_js(script).unwrap();
    assert_eq!(page.read_clipboard(), "secret");
    assert!(page.session.html.contains("granted"));
}
