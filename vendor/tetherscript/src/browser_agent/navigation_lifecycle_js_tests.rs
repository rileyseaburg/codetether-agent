use super::{BrowserPage, Locator, PageEventKind};

#[test]
fn page_navigation_dispatches_beforeunload_and_unload_to_js() {
    let html = "<script>window.addEventListener('beforeunload',function(){console.log('B');});window.addEventListener('unload',function(){console.log('U');});</script>";
    let mut page = BrowserPage::from_html("https://example.test/a", html);
    page.run_scripts().unwrap();
    let start = page.console_events().len();

    page.goto_html("https://example.test/b", "<p>b</p>");

    let messages = page.console_events()[start..]
        .iter()
        .map(|event| event.message.clone())
        .collect::<Vec<_>>();
    assert_eq!(messages, ["B", "U"]);
}

#[test]
fn hash_navigation_logs_and_dispatches_url_metadata() {
    let html = "<script>window.addEventListener('hashchange',function(e){console.log(e.oldURL+'>'+e.newURL);});</script><a id='x' href='#x'>x</a>";
    let mut page = BrowserPage::from_html("https://example.test/app", html);
    page.run_scripts().unwrap();
    let start = page.event_log().len();

    page.click(&Locator::css("#x")).unwrap();

    let events = &page.event_log()[start..];
    assert!(events
        .iter()
        .any(|event| event.kind == PageEventKind::Navigation
            && event.action == "hashchange"
            && event
                .message
                .contains("from=https://example.test/app to=https://example.test/app#x")));
    assert_eq!(
        page.console_events().last().unwrap().message,
        "https://example.test/app>https://example.test/app#x"
    );
}
