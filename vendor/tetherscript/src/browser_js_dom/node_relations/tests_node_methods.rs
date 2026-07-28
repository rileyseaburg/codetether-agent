use crate::browser_js::eval_with_dom;
use crate::js::JsValue;

fn eval(script: &str) -> JsValue {
    eval_with_dom(
        "<main id='app'><p id='a'>one</p><p id='b'>two</p></main>",
        script,
    )
    .unwrap()
    .value
}

#[test]
fn is_same_node_matches_only_same_resolved_handle() {
    let value = eval(
        "let a=document.getElementById('a');\
         let again=document.getElementById('a');\
         let b=document.getElementById('b');\
         let clone=a.cloneNode(true);\
         [a.isSameNode(again),a.isSameNode(b),a.isSameNode(clone),\
          a.isSameNode(null),a.isSameNode(undefined)].join(':');",
    );
    assert_eq!(value, JsValue::String("true:false:false:false:false".into()));
}

#[test]
fn is_equal_node_compares_structure() {
    let value = eval(
        "let a=document.getElementById('a');\
         let clone=a.cloneNode(true);\
         let shallow=a.cloneNode(false);\
         let b=document.getElementById('b');\
         [a.isEqualNode(clone),a.isEqualNode(shallow),\
          a.isEqualNode(b),a.isEqualNode(null)].join(':');",
    );
    assert_eq!(value, JsValue::String("true:false:false:false".into()));
}

#[test]
fn normalize_merges_text_and_detaches_removed_text_handles() {
    let value = eval(
        "let app=document.getElementById('app');\
         let d=document.createElement('div');d.setAttribute('id','d');d=app.appendChild(d);\
         d.appendChild(document.createTextNode('a'));\
         let empty=d.appendChild(document.createTextNode(''));\
         let right=d.appendChild(document.createTextNode('b'));\
         d.normalize();let fresh=document.getElementById('d');\
         [fresh.childNodes.length,fresh.textContent,\
          right.isSameNode(fresh.firstChild),empty.textContent].join(':');",
    );
    assert_eq!(value, JsValue::String("1:ab:false:".into()));
}
