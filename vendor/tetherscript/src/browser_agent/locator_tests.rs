use super::query::locate;
use super::{BrowserPage, Locator};

#[test]
fn role_name_matches_aria_label_labelledby_and_text() {
    let page = BrowserPage::from_html(
        "mem://roles",
        "<button aria-label='Save changes'></button><span id='n'>Archive</span>\
         <button aria-labelledby='n'></button><button>Cancel</button>",
    );

    assert_eq!(
        locate(
            &page.session.document,
            &Locator::role_name("button", "Save")
        )
        .len(),
        1
    );
    assert_eq!(
        locate(
            &page.session.document,
            &Locator::role_name("button", "Archive")
        )
        .len(),
        1
    );
    assert_eq!(
        locate(
            &page.session.document,
            &Locator::role_name("button", "Cancel")
        )
        .len(),
        1
    );
}
