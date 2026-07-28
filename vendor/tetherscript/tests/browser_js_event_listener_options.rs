use tetherscript::browser_js::eval_with_dom;
use tetherscript::js::JsValue;

fn eval(script: &str) -> JsValue {
    eval_with_dom("<div id='p'><button id='c'>Go</button></div>", script)
        .unwrap()
        .value
}

#[test]
fn element_remove_event_listener_matches_capture_flag() {
    let value = eval(
        "let p=document.getElementById('p');let c=document.getElementById('c');\
         let seen='';function mark(e){seen=seen+e.eventPhase;}\
         p.addEventListener('click',mark,true);\
         p.addEventListener('click',mark,false);\
         p.removeEventListener('click',mark,false);\
         c.dispatchEvent({type:'click',bubbles:true});seen;",
    );

    assert_eq!(value, JsValue::String("1".into()));
}

#[test]
fn once_listener_removes_only_matching_capture_registration() {
    let value = eval(
        "let p=document.getElementById('p');let c=document.getElementById('c');\
         let seen='';function mark(e){seen=seen+e.eventPhase;}\
         p.addEventListener('click',mark,{capture:true,once:true});\
         p.addEventListener('click',mark,false);\
         c.dispatchEvent({type:'click',bubbles:true});\
         c.dispatchEvent({type:'click',bubbles:true});seen;",
    );

    assert_eq!(value, JsValue::String("133".into()));
}

#[test]
fn window_remove_event_listener_matches_capture_flag_and_once() {
    let value = eval(
        "let seen='';function mark(){seen=seen+'x';}\
         window.addEventListener('agent',mark,true);\
         window.addEventListener('agent',mark,false);\
         window.removeEventListener('agent',mark,false);\
         window.dispatchEvent({type:'agent'});\
         window.dispatchEvent({type:'agent'});\
         window.addEventListener('once',mark,{capture:true,once:true});\
         window.dispatchEvent({type:'once'});\
         window.dispatchEvent({type:'once'});seen;",
    );

    assert_eq!(value, JsValue::String("xxx".into()));
}

#[test]
fn owner_document_exposes_event_target_methods() {
    let value = eval(
        "let root=document.getElementById('p');let seen='';\
         root.ownerDocument.addEventListener('selectionchange',function(e){seen=e.type;});\
         root.ownerDocument.dispatchEvent({type:'selectionchange'});seen;",
    );

    assert_eq!(value, JsValue::String("selectionchange".into()));
}

#[test]
fn owner_document_exposes_node_creation_methods() {
    let value = eval(
        "let doc=document.getElementById('p').ownerDocument;\
         let svg=doc.createElementNS('http://www.w3.org/2000/svg','svg:path');\
         let span=doc.createElement('span');span.appendChild(doc.createTextNode('x'));\
         svg.namespaceURI+'|'+svg.localName+'|'+svg.prefix+'|'+svg.nodeName+'|'+span.textContent;",
    );

    assert_eq!(
        value,
        JsValue::String("http://www.w3.org/2000/svg|path|svg|svg:path|x".into())
    );
}
