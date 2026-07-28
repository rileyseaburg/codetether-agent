use crate::browser_js::eval_with_dom;
use crate::js::JsValue;

fn eval(html: &str, script: &str) -> JsValue {
    eval_with_dom(html, script).unwrap().value
}

#[test]
fn form_elements_exposes_length_index_and_item() {
    let html = "<form id='f'><div><input id='i'><button id='b'></button>\
        <select id='s'></select><textarea id='t'></textarea></div></form>";
    let script = "let f=document.getElementById('f');let e=f.elements;\
        f.length+':'+e.length+':'+e[0].id+e.item(1).id+e.item(2).id\
        +e.item(3).id+':'+e.item(4);";
    assert_eq!(eval(html, script), JsValue::String("4:4:ibst:null".into()));
}

#[test]
fn form_elements_named_item_and_direct_lookup_match_id_or_name() {
    let html = "<form id='f'><input id='byId' name='first'>\
        <input id='other' name='byName'></form>";
    let script = "let e=document.getElementById('f').elements;\
        e.namedItem('byId').id+':'+e.namedItem('byName').id+':'\
        +e.byId.name+':'+e.byName.id+':'+e.namedItem('missing');";
    assert_eq!(
        eval(html, script),
        JsValue::String("byId:other:first:other:null".into())
    );
}

#[test]
fn form_elements_refresh_after_append_child() {
    let html = "<form id='f'><input id='a'></form>";
    let script = "let f=document.getElementById('f');let e=f.elements;\
        let added=document.createElement('input');added.setAttribute('id','b');\
        added.setAttribute('name','late');f.appendChild(added);\
        f.length+':'+e.length+':'+f.elements.length+':'+e[1].id+':'\
        +e.item(1).id+':'+e.namedItem('late').id+':'+e.late.id;";
    assert_eq!(eval(html, script), JsValue::String("2:2:2:b:b:b:b".into()));
}
