use crate::browser_agent::permissions::BrowserPermission;
use crate::browser_agent::BrowserPage;

fn granted_page() -> BrowserPage {
    let mut page = BrowserPage::from_html("https://app.test", "<p id='out'></p>");
    page.grant_permission("https://app.test", BrowserPermission::ClipboardRead);
    page.grant_permission("https://app.test", BrowserPermission::ClipboardWrite);
    page
}

#[test]
fn clipboard_items_round_trip_plain_text_item() {
    let mut page = granted_page();
    let script = "let out='';let item=new ClipboardItem({'text/plain':'hello'});navigator.clipboard.write([item]).then(function(){navigator.clipboard.read().then(function(items){items[0].getType('text/plain').then(function(blob){blob.text().then(function(text){out=items.length+':'+items[0].types[0]+':'+blob.type+':'+text;});});});});out;";
    assert_eq!(
        page.eval_js(script).unwrap().display(),
        "1:text/plain:text/plain:hello"
    );
    assert_eq!(page.read_clipboard(), "hello");
}

#[test]
fn clipboard_text_methods_keep_item_compatibility() {
    let mut page = granted_page();
    let script = "let seen='';navigator.clipboard.writeText('alpha');navigator.clipboard.read().then(function(items){items[0].getType('text/plain').then(function(blob){blob.text().then(function(t){seen=t;});});});navigator.clipboard.write([new ClipboardItem({'text/plain':'beta'})]);let text='';navigator.clipboard.readText().then(function(t){text=t;});seen+'|'+text;";
    assert_eq!(page.eval_js(script).unwrap().display(), "alpha|beta");
    assert_eq!(page.read_clipboard(), "beta");
}

#[test]
fn clipboard_object_methods_preserve_denied_errors() {
    let mut page = BrowserPage::from_html("https://app.test", "");
    page.deny_permission("https://app.test", BrowserPermission::ClipboardRead);
    page.deny_permission("https://app.test", BrowserPermission::ClipboardWrite);
    page.write_clipboard("keep");
    let script = "let out='';navigator.clipboard.write([new ClipboardItem({'text/plain':'drop'})]).catch(function(e){out='w:'+e.name+':'+e.message;});navigator.clipboard.read().catch(function(e){out=out+'|r:'+e.name+':'+e.message;});out;";
    assert_eq!(
        page.eval_js(script).unwrap().display(),
        "w:NotAllowedError:clipboard-write denied|r:NotAllowedError:clipboard-read denied"
    );
    assert_eq!(page.read_clipboard(), "keep");
}
