use super::super::{eval_with_dom, JsValue};

#[test]
fn range_selects_node_contents_and_exposes_metadata() {
    let result = eval_with_dom(
        "<p id='p'>Hello <b>world</b></p>",
        "let p=document.getElementById('p'); let r=document.createRange(); \
         r.selectNodeContents(p); r.startOffset + ':' + r.endOffset + ':' + \
         r.collapsed + ':' + r.toString();",
    )
    .unwrap();
    assert_eq!(
        result.value,
        JsValue::String("0:2:false:Hello world".into())
    );
}

#[test]
fn selection_add_range_and_get_range_at_round_trip() {
    let result = eval_with_dom(
        "<main><p id='p'>Alpha</p></main>",
        "let r=document.createRange(); r.selectNode(document.getElementById('p')); \
         let s=document.getSelection(); s.addRange(r); \
         s.rangeCount + ':' + s.isCollapsed + ':' + s.getRangeAt(0).toString();",
    )
    .unwrap();
    assert_eq!(result.value, JsValue::String("1:false:Alpha".into()));
}

#[test]
fn focused_text_control_selection_is_visible_to_selection_text() {
    let result = eval_with_dom(
        "<input id='q' value='abcd'>",
        "let q=document.getElementById('q'); q.focus(); q.setSelectionRange(1,3); \
         document.getSelection().toString();",
    )
    .unwrap();
    assert_eq!(result.value, JsValue::String("bc".into()));
}

#[test]
fn contenteditable_type_text_replaces_selected_contents() {
    let result = eval_with_dom(
        "<div id='e' contenteditable='true'>old</div>",
        "let e=document.getElementById('e'); let r=document.createRange(); \
         r.selectNodeContents(e); let s=document.getSelection(); \
         s.removeAllRanges(); s.addRange(r); e.typeText('new'); e.textContent;",
    )
    .unwrap();
    assert_eq!(result.value, JsValue::String("new".into()));
}
