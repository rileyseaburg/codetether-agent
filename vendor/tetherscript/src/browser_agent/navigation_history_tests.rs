use super::{BrowserPage, NavigationKind, NavigationStatus, PageLoadState};

#[test]
fn page_history_goes_back_forward_and_by_delta() {
    let mut page = BrowserPage::from_html("mem://one", "<p>one</p>");
    page.goto_html("mem://two", "<p>two</p>");
    page.goto_html("mem://three", "<p>three</p>");

    assert_eq!(page.history_index(), 3);
    assert_eq!(page.history_entries().len(), 4);

    let back = page.go_back();
    assert_eq!(back.status, NavigationStatus::Committed);
    assert_eq!(back.navigation.url, "mem://two");
    assert_eq!(back.kind, NavigationKind::DocumentReplacement);

    let first = page.go(-1);
    assert_eq!(first.navigation.url, "mem://one");
    assert_eq!(page.history_index(), 1);

    let forward = page.go_forward();
    assert_eq!(forward.navigation.url, "mem://two");
    assert_eq!(
        forward
            .wait_for_load_state(PageLoadState::Load)
            .unwrap()
            .url,
        "mem://two"
    );
}

#[test]
fn out_of_range_history_traversal_reports_no_entry() {
    let mut page = BrowserPage::from_html("mem://only", "<p>only</p>");

    let result = page.go_forward();

    assert_eq!(result.status, NavigationStatus::NoEntry);
    assert_eq!(result.navigation.url, "mem://only");
    assert!(result.wait_for_load_state(PageLoadState::Load).is_err());
}
