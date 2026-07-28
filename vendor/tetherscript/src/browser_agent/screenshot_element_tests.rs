use crate::browser::Rgba;

use super::{BrowserPage, Locator};

#[test]
fn element_screenshot_crops_to_resolved_bounds() {
    let html = "<div style='height:2px'></div><main id='box' \
                style='background: green; width:3px; height:2px'></main>";
    let page = BrowserPage::from_html("mem://element", html);

    let shot = page.element_screenshot(&Locator::css("#box")).unwrap();

    assert_eq!((shot.bounds.x, shot.bounds.y), (0, 2));
    assert_eq!((shot.image.width, shot.image.height), (3, 2));
    assert_eq!(
        shot.image.pixel(0, 0),
        Some(Rgba {
            r: 0,
            g: 128,
            b: 0,
            a: 255
        })
    );
}
