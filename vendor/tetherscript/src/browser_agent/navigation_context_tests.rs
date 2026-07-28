use super::{BrowserContext, BrowserPage};

#[test]
fn page_history_is_isolated_between_context_pages() {
    let mut context = BrowserContext::new();
    let first = context.new_page(BrowserPage::from_html("mem://one", "<p>one</p>"));
    let second = context.new_page(BrowserPage::from_html("mem://other", "<p>other</p>"));

    context
        .page_mut(first)
        .unwrap()
        .goto_html("mem://two", "<p>two</p>");

    let first_page = context.page(first).unwrap();
    let second_page = context.page(second).unwrap();

    assert_eq!(first_page.session.url, "mem://two");
    assert_eq!(second_page.session.url, "mem://other");
    assert_eq!(first_page.history_entries().len(), 3);
    assert_eq!(second_page.history_entries().len(), 2);
    assert_eq!(second_page.history_index(), 1);
}
