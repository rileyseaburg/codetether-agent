//! IndexedDB model tests.

use super::IndexedDbStore;
use crate::browser_agent::{BrowserContext, BrowserPage};

#[test]
fn store_put_get_delete_round_trips_record() {
    let mut store = IndexedDbStore::new();
    store.put("https://example.test", "app", "users", "1", "Ada");

    assert_eq!(
        store.get("https://example.test", "app", "users", "1"),
        Some("Ada")
    );
    assert!(store.delete("https://example.test", "app", "users", "1"));
    assert_eq!(store.get("https://example.test", "app", "users", "1"), None);
    assert!(store.is_empty());
}

#[test]
fn context_records_are_origin_scoped() {
    let mut context = BrowserContext::new();
    context.indexed_db_put("https://example.test/a", "app", "s", "k", "one");
    context.indexed_db_put("https://other.test/a", "app", "s", "k", "two");

    assert_eq!(
        context.indexed_db_get("https://example.test/b", "app", "s", "k"),
        Some("one".into())
    );
    assert_eq!(
        context.indexed_db_records("https://example.test/c").len(),
        1
    );
}

#[test]
fn context_pages_share_records_but_contexts_do_not() {
    let mut left = BrowserContext::new();
    let first = left.new_page(BrowserPage::from_html("https://example.test/a", ""));
    let second = left.new_page(BrowserPage::from_html("https://example.test/b", ""));
    left.page_mut(first)
        .unwrap()
        .indexed_db_put("app", "s", "k", "one")
        .unwrap();

    assert_eq!(
        left.page_mut(second)
            .unwrap()
            .indexed_db_get("app", "s", "k")
            .unwrap(),
        Some("one".into())
    );
    assert_eq!(
        BrowserContext::new().indexed_db_get("https://example.test", "app", "s", "k"),
        None
    );
}
