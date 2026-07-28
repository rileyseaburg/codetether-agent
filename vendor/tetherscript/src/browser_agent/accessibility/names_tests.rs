use crate::browser_agent::BrowserPage;

#[test]
fn snapshot_uses_labels_alt_title_and_filters_hidden_subtrees() {
    let page = BrowserPage::from_html(
        "mem://a11y",
        "<main><label for='email'>Email</label><input id='email'>\
         <img title='Logo'><button aria-hidden='true'>Hide</button>\
         <section inert><button>Skip</button></section></main>",
    );

    let snapshot = page.accessibility_snapshot();
    let main = &snapshot.roots[0];

    assert_eq!(main.children[1].name, "Email");
    assert_eq!(main.children[2].name, "Logo");
    assert!(!format!("{snapshot:?}").contains("Hide"));
    assert!(!format!("{snapshot:?}").contains("Skip"));
}
