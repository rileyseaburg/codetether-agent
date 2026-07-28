use crate::browser::Rgba;

use super::BrowserPage;

#[test]
fn screenshot_dimensions_follow_viewport() {
    let mut page = BrowserPage::from_html("mem://shot", "<main>Shot</main>");
    page.set_viewport_size(12, 8).unwrap();

    let image = page.screenshot().unwrap();

    assert_eq!((image.width, image.height), (12, 8));
}

#[test]
fn screenshot_ppm_header_uses_viewport() {
    let mut page = BrowserPage::from_html("mem://ppm", "<main>Shot</main>");
    page.set_viewport_size(4, 3).unwrap();

    let ppm = page.screenshot_ppm().unwrap();

    assert!(ppm.starts_with(b"P6\n4 3\n255\n"));
}

#[test]
fn screenshot_reflects_dom_mutation() {
    let html = "<main id='box' style='background: red; width: 4px; height: 4px'></main>";
    let mut page = BrowserPage::from_html("mem://mutate", html);

    page.eval_js(
        "document.querySelector('#box').setAttribute('style', \
         'background: blue; width: 4px; height: 4px');",
    )
    .unwrap();
    let image = page.screenshot().unwrap();

    assert_eq!(
        image.pixel(0, 0),
        Some(Rgba {
            r: 0,
            g: 0,
            b: 255,
            a: 255
        })
    );
}
