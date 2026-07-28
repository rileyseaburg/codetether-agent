use tetherscript::browser_js::eval_with_dom;
use tetherscript::js::JsValue;

#[test]
fn tree_walker_visits_text_nodes_and_updates_current() {
    let result = eval_with_dom(
        "<main id='app'><p>A</p><span>B</span></main>",
        "let root=document.getElementById('app'); let w=document.createTreeWalker(root, NodeFilter.SHOW_TEXT, null); let a=w.nextNode(); let b=w.nextNode(); let end=w.nextNode(); a.textContent + b.textContent + ':' + end + ':' + w.currentNode.textContent + ':' + w.previousNode().textContent + ':' + w.root.id;",
    )
    .unwrap();
    assert_eq!(result.value, JsValue::String("AB:null:B:A:app".into()));
}

#[test]
fn tree_walker_filter_reject_prunes_element_descendants() {
    let result = eval_with_dom(
        "<main id='app'><section id='keep'><b id='inside'></b></section><section id='drop'><i id='lost'></i></section><p id='tail'></p></main>",
        "let filter={acceptNode:function(n){ if(n.id === 'drop') { return NodeFilter.FILTER_REJECT; } return NodeFilter.FILTER_ACCEPT; }}; let w=document.createTreeWalker(document.getElementById('app'), NodeFilter.SHOW_ELEMENT, filter); let ids=''; let n=w.nextNode(); while(n){ ids=ids+n.id; n=w.nextNode(); } ids;",
    )
    .unwrap();
    assert_eq!(result.value, JsValue::String("keepinsidetail".into()));
}

#[test]
fn node_iterator_returns_dom_wrappers_and_steps_backward() {
    let result = eval_with_dom(
        "<main id='app'><p id='p'></p></main>",
        "let it=document.createNodeIterator(document.getElementById('app'), NodeFilter.SHOW_ELEMENT, null); let first=it.nextNode(); let second=it.nextNode(); let back=it.previousNode(); NodeFilter.SHOW_TEXT + ':' + NodeFilter.SHOW_ALL + ':' + NodeFilter.FILTER_SKIP + ':' + first.getAttribute('id') + ':' + second.nodeName + ':' + back.getAttribute('id') + ':' + it.currentNode.id + ':' + it.root.id;",
    )
    .unwrap();
    assert_eq!(
        result.value,
        JsValue::String("4:4294967295:3:app:p:p:p:app".into())
    );
}
