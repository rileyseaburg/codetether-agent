use crate::browser_agent::BrowserPage;

#[test]
fn snapshot_focus_order_skips_hidden_inert_and_disabled_controls() {
    let page = BrowserPage::from_html(
        "mem://a11y",
        "<button id='hide' aria-hidden='true'></button><button id='late'></button>\
         <button id='first' tabindex='1'></button><section inert><button id='skip'></button></section>\
         <input id='secret' type='hidden'><a id='link' href='/x'>Link</a>",
    );

    let snapshot = page.accessibility_snapshot();

    assert_eq!(snapshot.focus_order, vec!["#first", "#late", "#link"]);
    assert_eq!(snapshot.roots[0].selector, "#late");
    assert_eq!(snapshot.roots[1].focus_index, Some(0));
    assert_eq!(snapshot.roots[2].focus_index, Some(2));
}
