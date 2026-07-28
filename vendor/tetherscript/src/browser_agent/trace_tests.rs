use super::{BrowserPage, Locator};
use crate::browser_session::ScrollState;

#[test]
fn snapshot_includes_url_html_scroll_and_focus() {
    let mut page = BrowserPage::from_html("mem://trace", "<main id='app'>Hi</main>");
    page.session.scroll = ScrollState { x: 3, y: 7 };
    page.session.focus = Some("#app".into());

    let snapshot = page.capture_snapshot();

    assert_eq!(snapshot.url, "mem://trace");
    assert!(snapshot.html.contains("id='app'"));
    assert_eq!(snapshot.scroll, ScrollState { x: 3, y: 7 });
    assert_eq!(snapshot.focused_selector.as_deref(), Some("#app"));
}

#[test]
fn trace_entries_keep_deterministic_order() {
    let mut page = BrowserPage::from_html("mem://trace", "<button id='a'>A</button>");
    let first_before = page.capture_snapshot();
    page.click(&Locator::css("#a")).unwrap();
    let first_after = page.capture_snapshot();
    page.append_trace_entry("click#a", first_before, first_after);

    let second_before = page.capture_snapshot();
    page.session.scroll = ScrollState { x: 0, y: 12 };
    let second_after = page.capture_snapshot();
    page.append_trace_entry("scroll", second_before, second_after);

    let labels: Vec<&str> = page
        .trace_entries()
        .iter()
        .map(|e| e.label.as_str())
        .collect();
    assert_eq!(labels, vec!["click#a", "scroll"]);
    assert_eq!(page.trace_entries()[1].after.scroll.y, 12);
}
