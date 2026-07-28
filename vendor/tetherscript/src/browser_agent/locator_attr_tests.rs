use super::query::locate;
use super::{BrowserPage, Locator};

#[test]
fn placeholder_title_and_alt_locators_work() {
    let page = BrowserPage::from_html(
        "mem://attrs",
        "<input placeholder='Search docs'><img alt='tetherscript logo'><button title='Close'></button>",
    );

    assert_eq!(
        locate(&page.session.document, &Locator::placeholder("Search")).len(),
        1
    );
    assert_eq!(
        locate(&page.session.document, &Locator::alt_text("logo")).len(),
        1
    );
    assert_eq!(
        locate(&page.session.document, &Locator::title("Close")).len(),
        1
    );
}

#[test]
fn text_locator_supports_contains_and_exact_modes() {
    let page = BrowserPage::from_html(
        "mem://text",
        "<button>Save draft</button><button>Save</button>",
    );

    assert_eq!(
        locate(&page.session.document, &Locator::text("Save")).len(),
        2
    );
    assert_eq!(
        locate(&page.session.document, &Locator::text_exact("Save")).len(),
        1
    );
}
