use crate::browser_js::eval_with_dom;
use crate::js::JsValue;

#[test]
fn range_select_node_and_contents_extract_text() {
    let result = eval_with_dom(
        "<div id='a'>Hello <b id='b'>World</b></div>",
        "let r=document.createRange();let a=document.getElementById('a');\
         r.selectNodeContents(a);let contents=r.toString()+':'\
         +r.startContainer.id+':'+r.endOffset+':'+r.collapsed;\
         let b=document.getElementById('b');r.selectNode(b);\
         contents+'|'+r.toString()+':'+r.startContainer.id+':'\
         +r.startOffset+':'+r.endOffset+':'+r.commonAncestorContainer.id;",
    )
    .unwrap();

    assert_eq!(
        result.value,
        JsValue::String("Hello World:a:2:false|World:a:1:2:a".into())
    );
}

#[test]
fn range_bounds_collapse_and_clone_are_stable() {
    let result = eval_with_dom(
        "<div id='a'>Hello <b>World</b></div>",
        "let r=document.createRange();let t=document.getElementById('a').firstChild;\
         r.setStart(t,1);r.setEnd(t,5);let c=r.cloneRange();\
         r.collapse(true);c.toString()+':'+r.collapsed+':'\
         +r.toString()+':'+c.startOffset+':'+c.endOffset;",
    )
    .unwrap();

    assert_eq!(result.value, JsValue::String("ello:true::1:5".into()));
}
