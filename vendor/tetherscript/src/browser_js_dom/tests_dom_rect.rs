use crate::browser_js::eval_with_dom;
use crate::js::JsValue;

#[test]
fn dom_rect_to_json_returns_stable_snapshot() {
    let result = eval_with_dom(
        "<style>#box{width:12px;height:4px}</style><div id='box'>Hi</div>",
        "let r=document.getElementById('box').getBoundingClientRect();\
         let j=r.toJSON();[j.x,j.y,j.width,j.height,j.left,j.top,j.right,j.bottom].join(':');",
    )
    .unwrap();

    assert_eq!(result.value, JsValue::String("0:1:12:4:0:1:12:5".into()));
}
