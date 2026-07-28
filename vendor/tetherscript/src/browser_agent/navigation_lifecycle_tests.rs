use super::{BrowserPage, PageEventKind};

fn nav_actions(page: &BrowserPage, start: usize) -> Vec<String> {
    page.event_log()[start..]
        .iter()
        .filter(|event| event.kind == PageEventKind::Navigation)
        .map(|event| event.action.clone())
        .collect()
}

#[test]
fn document_navigation_logs_lifecycle_metadata() {
    let mut page = BrowserPage::from_html("mem://one", "<p>one</p>");
    let start = page.event_log().len();

    page.goto_html("mem://two", "<p>two</p>");

    assert_eq!(
        nav_actions(&page, start),
        ["beforeunload", "unload", "document_replace"]
    );
    assert!(page.event_log()[start]
        .message
        .contains("action=goto_html kind=DocumentReplacement from=mem://one to=mem://two"));
}

#[test]
fn reload_logs_unload_pair_and_reload_metadata() {
    let mut page = BrowserPage::from_html("mem://reload", "<p>old</p>");
    let start = page.event_log().len();

    page.reload();

    assert_eq!(
        nav_actions(&page, start),
        ["beforeunload", "unload", "reload"]
    );
    assert!(page.event_log()[start + 2]
        .message
        .contains("action=reload kind=Reload from=mem://reload to=mem://reload"));
}
