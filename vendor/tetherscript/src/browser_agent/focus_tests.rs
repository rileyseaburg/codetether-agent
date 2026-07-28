use super::{BrowserPage, Locator};

#[test]
fn focus_order_uses_dom_order_and_skips_disabled() {
    let page = BrowserPage::from_html(
        "mem://focus",
        "<input id='one'><button id='skip' disabled>Skip</button><a id='two'>Two</a>",
    );

    let selectors = page
        .focus_order()
        .into_iter()
        .map(|target| target.selector)
        .collect::<Vec<_>>();

    assert_eq!(selectors, vec!["#one", "#two"]);
}

#[test]
fn focus_order_honors_positive_tabindex_first() {
    let page = BrowserPage::from_html(
        "mem://focus",
        "<button id='late'></button><button id='first' tabindex='1'></button>",
    );

    assert_eq!(page.focus_order()[0].selector, "#first");
}

#[test]
fn tab_key_advances_focus_and_persists_active_element() {
    let mut page = BrowserPage::from_html(
        "mem://focus",
        "<input id='one'><input id='two'><input id='three' disabled>",
    );
    page.eval_js(
        "document.getElementById('two').addEventListener('focus', function(){ this.setAttribute('data-focused','yes'); });",
    )
    .unwrap();

    page.press(&Locator::css("#one"), "Tab").unwrap();

    assert_eq!(page.session.focus.as_deref(), Some("#two"));
    assert!(page.session.html.contains("data-focused=\"yes\""));
}

#[test]
fn focus_previous_moves_backward() {
    let mut page =
        BrowserPage::from_html("mem://focus", "<input id='one'><button id='two'></button>");

    page.focus_next().unwrap();
    page.focus_next().unwrap();
    let target = page.focus_previous().unwrap().unwrap();

    assert_eq!(target.selector, "#one");
    assert_eq!(page.session.focus.as_deref(), Some("#one"));
}
