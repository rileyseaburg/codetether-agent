use super::*;

#[path = "browser_js_webgl_tests.rs"]
mod webgl_tests;

#[test]
fn canvas_2d_fill_rect_records_commands_and_pixels() {
    let result = eval_with_dom(
        "<canvas id='c' width='2' height='2'></canvas>",
        "let ctx=document.getElementById('c').getContext('2d'); \
         ctx.fillStyle='#0f0'; ctx.fillRect(0,0,1,1); \
         ctx.getImageData(0,0,1,1).data.join(',') + ':' + ctx.__summary();",
    )
    .unwrap();
    assert_eq!(result.value.display(), "0,255,0,255:2x2:1:16711945");
    let attrs = match &result.document.children[0] {
        Node::Element(el) => &el.attrs,
        _ => panic!("expected canvas"),
    };
    assert_eq!(
        attrs.get("data-agent-canvas-commands").map(String::as_str),
        Some("fillRect|0|0|1|1|#0f0")
    );
}

#[test]
fn canvas_width_height_properties_follow_attributes() {
    let result = eval_with_dom(
        "<canvas id='c' width='4' height='3'></canvas>",
        "let c=document.getElementById('c'); c.width = 5; c.height;",
    )
    .unwrap();
    assert_eq!(result.value, JsValue::Number(3.0));
    let attrs = match &result.document.children[0] {
        Node::Element(el) => &el.attrs,
        _ => panic!("expected canvas"),
    };
    assert_eq!(attrs.get("width").map(String::as_str), Some("5"));
}
