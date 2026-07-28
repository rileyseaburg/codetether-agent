use crate::browser_js::eval_with_dom;
use crate::js::JsValue;

#[path = "tests_html_collection_named.rs"]
mod tests_html_collection_named;

#[test]
fn query_selector_all_exposes_item_and_for_each() {
    let result = eval_with_dom(
        "<ul><li>A</li><li>B</li></ul>",
        "let nodes=document.querySelectorAll('li');let seen='';\
         nodes.forEach(function(n,i,c){seen=seen+i+n.textContent+(c===nodes);});\
         nodes.length+':'+nodes[1].textContent+':'+nodes.item(0).textContent\
         +':'+nodes.item(2)+':'+seen;",
    )
    .unwrap();
    assert_eq!(
        result.value,
        JsValue::String("2:B:A:null:0Atrue1Btrue".into())
    );
}

#[test]
fn children_collection_is_indexed_and_snapshot_based() {
    let result = eval_with_dom(
        "<div id='app'><span>A</span><em>B</em></div>",
        "let app=document.getElementById('app');let kids=app.children;let seen='';\
         kids.forEach(function(n,i,c){seen=seen+i+n.tagName+(c===kids);});\
         let strong=document.createElement('strong');app.appendChild(strong);\
         kids.length+':'+app.children.length+':'+kids[0].textContent\
         +':'+kids.item(1).tagName+':'+kids.item(2)+':'+seen;",
    )
    .unwrap();
    assert_eq!(
        result.value,
        JsValue::String("2:3:A:EM:null:0SPANtrue1EMtrue".into())
    );
}

#[test]
fn child_nodes_collection_includes_text_nodes() {
    let result = eval_with_dom(
        "<div id='app'>A<span>B</span>C</div>",
        "let nodes=document.getElementById('app').childNodes;let seen='';\
         nodes.forEach(function(n,i,c){seen=seen+i+n.nodeType+(c===nodes);});\
         nodes.length+':'+nodes[0].textContent+':'+nodes.item(1).tagName\
         +':'+nodes.item(3)+':'+seen;",
    )
    .unwrap();
    assert_eq!(
        result.value,
        JsValue::String("3:A:SPAN:null:03true11true23true".into())
    );
}
