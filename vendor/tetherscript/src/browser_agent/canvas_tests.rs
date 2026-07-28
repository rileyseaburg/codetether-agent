use crate::browser_agent::{BrowserPage, Locator};

#[test]
fn canvas_surface_reports_dimensions_and_fill_rect() {
    let html = "<canvas id='c' width='4' height='3'></canvas>";
    let mut page = BrowserPage::from_html("mem://canvas", html);
    page.eval_js(
        "let c=document.getElementById('c'); let ctx=c.getContext('2d'); \
         ctx.fillStyle='#f00'; ctx.fillRect(1,1,2,1);",
    )
    .unwrap();
    let surface = page.canvas_surface(&Locator::css("#c")).unwrap();
    assert_eq!((surface.width, surface.height), (4, 3));
    assert_eq!(surface.commands.len(), 1);
    assert_eq!(surface.commands[0].operation, "fillRect");
    assert_eq!(surface.commands[0].args, vec![1, 1, 2, 1]);
    assert_eq!(surface.commands[0].style.as_deref(), Some("#f00"));
    assert!(surface.checksum.unwrap_or_default() > 0);
}

#[test]
fn canvas_api_exposes_raster_bytes_to_js() {
    let mut page = BrowserPage::from_html("mem://canvas", "<canvas id='c' width='2' height='2'>");
    let value = page
        .eval_js(
            "let ctx=document.getElementById('c').getContext('2d'); \
             ctx.fillStyle='blue'; ctx.fillRect(0,0,1,1); \
             ctx.getImageData(0,0,1,1).data.join(',');",
        )
        .unwrap();
    assert_eq!(value.display(), "0,0,255,255");
}
