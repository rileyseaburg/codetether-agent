use crate::browser_agent::{BrowserPage, Locator};

#[test]
fn webgl_context_snapshot_reports_metadata() {
    let mut page = BrowserPage::from_html(
        "mem://webgl",
        "<canvas id='c' width='8' height='4'></canvas>",
    );
    page.eval_js(
        "let gl=document.getElementById('c').getContext('webgl2'); \
         gl.viewport(1,2,3,4); gl.clear(gl.COLOR_BUFFER_BIT);",
    )
    .unwrap();
    let snapshot = page.webgl_context(&Locator::css("#c")).unwrap();
    assert_eq!(
        (snapshot.version, snapshot.width, snapshot.height),
        (2, 8, 4)
    );
    assert_eq!(snapshot.viewport, [1, 2, 3, 4]);
    assert_eq!(snapshot.commands[1].operation, "clear");
    assert!(snapshot
        .supported_extensions
        .iter()
        .any(|name| name == "OES_texture_float"));
}
