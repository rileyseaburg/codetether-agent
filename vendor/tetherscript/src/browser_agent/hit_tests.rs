use super::{BrowserPage, Locator};

#[test]
fn hit_testing_rejects_covered_click_target() {
    let html = "<button id='target' style='width:20px;height:4px'>Save</button><div id='cover' style='position:absolute;left:0;top:0;width:20px;height:4px'></div>";
    let mut page = BrowserPage::from_html("mem://hit", html);

    let err = page.click(&Locator::css("#target")).unwrap_err();

    assert!(err.contains("receives_pointer"));
    assert!(err.contains("div#cover"));
}

#[test]
fn hit_testing_skips_visibility_hidden_cover() {
    let html = "<button id='target' style='width:20px;height:4px'>Save</button>\
        <div id='cover' style='visibility:hidden;position:absolute;left:0;top:0;\
        width:20px;height:4px'></div>";
    let mut page = BrowserPage::from_html("mem://hit-hidden", html);

    page.click(&Locator::css("#target")).unwrap();

    assert!(page.session.html.contains("Save"));
}
