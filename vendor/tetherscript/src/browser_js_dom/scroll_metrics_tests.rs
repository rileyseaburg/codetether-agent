use crate::browser_js::eval_with_dom;
use crate::js::JsValue;

#[test]
fn scroll_metrics_styled_element_dimensions_and_position() {
    let result = eval_with_dom(
        "<main style='height:3px'></main><div id='box' style='width:12px;height:4px;border-width:2px'></div>",
        "let b=document.getElementById('box');\
         [b.clientWidth,b.clientHeight,b.scrollWidth,b.scrollHeight,\
          b.offsetLeft,b.offsetTop,b.clientLeft,b.clientTop].join(':');",
    )
    .unwrap();

    assert_eq!(result.value, JsValue::String("12:4:12:4:0:3:2:2".into()));
}

#[test]
fn scroll_metrics_zero_size_element_reports_zero_sizes() {
    let result = eval_with_dom(
        "<div id='box' style='width:0px;height:0px'></div>",
        "let b=document.getElementById('box');\
         [b.clientWidth,b.clientHeight,b.scrollWidth,b.scrollHeight].join(':');",
    )
    .unwrap();

    assert_eq!(result.value, JsValue::String("0:0:0:0".into()));
}

#[test]
fn scroll_metrics_element_scroll_methods_are_noops() {
    let result = eval_with_dom(
        "<div id='box'></div>",
        "let b=document.getElementById('box');\
         typeof b.scrollIntoView==='function'&&b.scrollIntoView()===undefined&&\
         typeof b.scrollTo==='function'&&b.scrollTo(1,2)===undefined&&\
         typeof b.scrollBy==='function'&&b.scrollBy(3,4)===undefined;",
    )
    .unwrap();

    assert_eq!(result.value, JsValue::Bool(true));
}
