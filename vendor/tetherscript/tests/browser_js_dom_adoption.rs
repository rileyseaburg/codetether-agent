use tetherscript::browser_js::eval_with_dom;
use tetherscript::js::JsValue;

fn eval(script: &str) -> JsValue {
    eval_with_dom(
        "<main id='app'><p id='a'>A</p><p id='b'>B</p><p id='c'>C</p></main>",
        script,
    )
    .unwrap()
    .value
}

#[test]
fn append_child_moves_existing_node_and_keeps_listener() {
    let value = eval(
        "let app=document.getElementById('app');let a=document.getElementById('a');\
         a.addEventListener('click',function(){this.setAttribute('data-hit','yes');});\
         let returned=app.appendChild(a);a.click();\
         (returned===a)+':'+app.textContent+':'+a.getAttribute('data-hit')+':'\
         +document.querySelectorAll('#a').length;",
    );

    assert_eq!(value, JsValue::String("true:BCA:yes:1".into()));
}

#[test]
fn insert_before_moves_existing_node_without_cloning() {
    let value = eval(
        "let app=document.getElementById('app');let c=document.getElementById('c');\
         let a=document.getElementById('a');let returned=app.insertBefore(c,a);\
         (returned===c)+':'+app.textContent+':'+document.querySelectorAll('#c').length;",
    );

    assert_eq!(value, JsValue::String("true:CAB:1".into()));
}

#[test]
fn fragment_insert_empties_fragment_and_preserves_child_handles() {
    let value = eval(
        "let app=document.getElementById('app');let frag=document.createDocumentFragment();\
         let x=document.createElement('button');x.setAttribute('id','x');x.textContent='X';\
         x.addEventListener('click',function(){this.setAttribute('data-hit','yes');});\
         frag.appendChild(x);let returned=app.insertBefore(frag,app.children[1]);\
         x.click();(returned===frag)+':'+frag.childNodes.length+':'+app.textContent+':'\
         +x.getAttribute('data-hit')+':'+document.getElementById('x').textContent;",
    );

    assert_eq!(value, JsValue::String("true:0:AXBC:yes:X".into()));
}
