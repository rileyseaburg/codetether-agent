use crate::{browser_js::eval_with_dom, js::JsValue};

#[test]
fn replace_children_replaces_element_children_in_order() {
    let result = eval_with_dom(
        "<main id='app'><p>A</p><p>B</p></main>",
        "let app=document.getElementById('app');\
         let em=document.createElement('em');em.textContent='Y';\
         let ret=app.replaceChildren('X',em,'Z');\
         let fresh=document.getElementById('app');\
         String(ret)+':'+fresh.textContent+':'+fresh.innerHTML;",
    )
    .unwrap();
    let expected = JsValue::String("undefined:XYZ:X<em>Y</em>Z".into());
    assert_eq!(result.value, expected);
}

#[test]
fn replace_children_supports_fragments_and_stale_handles() {
    let result = eval_with_dom(
        "<main id='app'><p id='old'>A</p></main>",
        "let app=document.getElementById('app');let old=document.getElementById('old');\
         let text=document.createTextNode('T');\
         let frag=document.createDocumentFragment();\
         frag.appendChild(document.createTextNode('F'));\
         frag.replaceChildren('R',document.createElement('b'));\
         app.replaceChildren(frag);let ret=old.replaceChildren('safe');let textRet=text.replaceChildren('x');\
         let fresh=document.getElementById('app');\
         fresh.textContent+':'+fresh.childNodes.length+':'+String(ret)+':'+String(textRet);",
    )
    .unwrap();
    let expected = JsValue::String("R:2:undefined:undefined".into());
    assert_eq!(result.value, expected);
}

#[test]
fn replace_children_queues_record_and_lifecycle_hooks() {
    let result = eval_with_dom(
        "<main id='app'><x-log id='old'></x-log></main>",
        "customElements.define('x-log',{connectedCallback:function(){console.log('c:'+this.id)},\
         disconnectedCallback:function(){console.log('d:'+this.id)}});\
         let app=document.getElementById('app');\
         let obs=MutationObserver(function(r){console.log('m:'+r[0].addedNodes.length+':'+r[0].removedNodes.length)});\
         obs.observe(app,{childList:true});\
         let next=document.createElement('x-log');next.setAttribute('id','new');\
         app.replaceChildren(next);'ok';",
    )
    .unwrap();
    assert_eq!(result.value, JsValue::String("ok".into()));
    assert_eq!(result.console, vec!["c:old", "d:old", "c:new", "m:1:1"]);
}
