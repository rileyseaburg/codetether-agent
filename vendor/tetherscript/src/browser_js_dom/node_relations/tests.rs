use crate::browser_js::eval_with_dom;
use crate::js::JsValue;

fn eval(script: &str) -> JsValue {
    eval_with_dom(
        "<main id='app'><p id='a'><em id='c'></em></p><p id='b'></p></main>",
        script,
    )
    .unwrap()
    .value
}

#[test]
fn is_connected_marks_document_backed_and_detached_nodes() {
    let value = eval("let app=document.getElementById('app'); let made=document.createElement('span'); let added=app.appendChild(document.createElement('em')); app.isConnected + ':' + made.isConnected + ':' + added.isConnected + ':' + document.isConnected;");
    assert_eq!(value, JsValue::String("true:false:true:true".into()));
}

#[test]
fn compare_document_position_orders_document_nodes() {
    let value = eval("let a=document.getElementById('a'); let b=document.getElementById('b'); let c=document.getElementById('c'); a.compareDocumentPosition(a)+':'+a.compareDocumentPosition(b)+':'+b.compareDocumentPosition(a)+':'+a.compareDocumentPosition(c)+':'+c.compareDocumentPosition(a);");
    assert_eq!(value, JsValue::String("0:4:2:20:10".into()));
}

#[test]
fn compare_document_position_marks_disconnected_nodes() {
    let value = eval("let app=document.getElementById('app'); let made=document.createElement('x'); app.compareDocumentPosition(made) === app.DOCUMENT_POSITION_DISCONNECTED + app.DOCUMENT_POSITION_IMPLEMENTATION_SPECIFIC + app.DOCUMENT_POSITION_FOLLOWING;");
    assert_eq!(value, JsValue::Bool(true));
}

#[test]
fn contains_returns_false_for_removed_node_handles() {
    let value = eval("let app=document.getElementById('app'); let b=document.getElementById('b'); let before=app.contains(b); b.remove(); before + ':' + app.contains(b);");
    assert_eq!(value, JsValue::String("true:false".into()));
}

#[test]
fn removed_middle_node_handles_do_not_alias_following_siblings() {
    let value = eval("let app=document.getElementById('app'); let b=document.getElementById('a'); let c=document.getElementById('b'); b.remove(); let before=app.contains(b)+':' +(document.getElementById('b')===null); b.remove(); before+':' +(document.getElementById('b')===null)+':' +c.id;");
    assert_eq!(value, JsValue::String("false:false:false:b".into()));
}
