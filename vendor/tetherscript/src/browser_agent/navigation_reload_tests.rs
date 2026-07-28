use super::{BrowserPage, NavigationKind, NavigationStatus, PageLoadState};

#[test]
fn reload_returns_waitable_metadata_without_new_history_entry() {
    let mut page = BrowserPage::from_html("mem://reload", "<p>old</p>");
    let count = page.history_entries().len();
    let before = page.navigation().id;

    let result = page.reload();

    assert_eq!(result.status, NavigationStatus::Committed);
    assert_eq!(result.kind, NavigationKind::Reload);
    assert_eq!(result.navigation.id, before + 1);
    assert_eq!(page.history_entries().len(), count);
    assert_eq!(
        result.wait_for_load_state(PageLoadState::Load).unwrap().url,
        "mem://reload"
    );
}

#[test]
fn go_zero_uses_reload_semantics() {
    let mut page = BrowserPage::from_html("mem://same", "<p>same</p>");

    let result = page.go(0);

    assert_eq!(result.kind, NavigationKind::Reload);
    assert_eq!(page.history_entries().len(), 2);
}
