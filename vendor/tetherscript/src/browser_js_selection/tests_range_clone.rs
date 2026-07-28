use super::super::{eval_with_dom, JsValue};

#[test]
fn clone_range_preserves_original_boundaries() {
    let result = eval_with_dom(
        "<p id='p'>Alpha</p>",
        "let p=document.getElementById('p'); let t=p.firstChild; \
         let r=document.createRange(); r.setStart(t,1); r.setEnd(t,4); \
         let c=r.cloneRange(); r.collapse(true); \
         c.toString() + ':' + r.toString() + ':' + c.collapsed + ':' + r.collapsed;",
    )
    .unwrap();
    assert_eq!(result.value, JsValue::String("lph::false:true".into()));
}

#[test]
fn cloned_range_can_move_independently() {
    let result = eval_with_dom(
        "<p id='p'>Alpha</p>",
        "let t=document.getElementById('p').firstChild; \
         let r=document.createRange(); r.setStart(t,0); r.setEnd(t,2); \
         let c=r.cloneRange(); c.setStart(t,2); c.setEnd(t,5); \
         r.toString() + ':' + c.toString();",
    )
    .unwrap();
    assert_eq!(result.value, JsValue::String("Al:pha".into()));
}
