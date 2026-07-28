use crate::browser_js::eval_with_dom;
use crate::js::JsValue;

const BOX_HTML: &str = "<style>#box{width:12px;height:4px}</style><div id='box'>Hi</div>";

#[test]
fn client_rects_first_rect_matches_bounding_rect() {
    let result = eval_with_dom(
        BOX_HTML,
        "let b=document.getElementById('box');let r=b.getBoundingClientRect();\
         let c=b.getClientRects()[0];\
         c.x===r.x&&c.y===r.y&&c.width===r.width&&c.height===r.height\
         &&c.left===r.left&&c.top===r.top&&c.right===r.right&&c.bottom===r.bottom;",
    )
    .unwrap();

    assert_eq!(result.value, JsValue::Bool(true));
}

#[test]
fn client_rects_item_zero_returns_rect() {
    let result = eval_with_dom(
        BOX_HTML,
        "let list=document.getElementById('box').getClientRects();\
         list.length+':'+list.item(0).width+':'+list.item(0).height;",
    )
    .unwrap();

    assert_eq!(result.value, JsValue::String("1:12:4".into()));
}

#[test]
fn client_rects_item_out_of_range_returns_null() {
    let result = eval_with_dom(
        BOX_HTML,
        "document.getElementById('box').getClientRects().item(99)===null;",
    )
    .unwrap();

    assert_eq!(result.value, JsValue::Bool(true));
}

#[test]
fn client_rects_zero_size_returns_empty_list() {
    let result = eval_with_dom(
        "<style>#box{width:0px;height:4px}</style><div id='box'></div>",
        "let list=document.getElementById('box').getClientRects();\
         list.length===0&&list.item(0)===null;",
    )
    .unwrap();

    assert_eq!(result.value, JsValue::Bool(true));
}
