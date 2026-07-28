use super::{BrowserPage, Locator, NavigationKind};

#[test]
fn hash_anchor_is_same_document_history_entry() {
    let mut page = BrowserPage::from_html(
        "https://example.test/app",
        "<a id='hash' href='#details'>Details</a><main id='app'></main>",
    );
    let document = page.session.document.clone();

    page.click(&Locator::css("#hash")).unwrap();

    assert_eq!(page.session.url, "https://example.test/app#details");
    assert_eq!(page.session.document, document);
    assert_eq!(
        page.history_entries().last().unwrap().kind,
        NavigationKind::SameDocument
    );
    assert!(page
        .session
        .trace
        .iter()
        .any(|event| event.action == "hashchange"));
    assert_eq!(page.go_back().kind, NavigationKind::SameDocument);
}
