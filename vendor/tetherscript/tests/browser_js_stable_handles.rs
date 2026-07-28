use tetherscript::browser_js::eval_with_dom;
use tetherscript::js::JsValue;

fn eval(script: &str) -> JsValue {
    eval_with_dom(
        "<main><button id='a'></button><button id='b'></button></main>",
        script,
    )
    .unwrap()
    .value
}

#[test]
fn shifted_element_handle_methods_use_current_node() {
    let value = eval(
        "let a=document.getElementById('a');let b=document.getElementById('b');\
         a.remove();b.setAttribute('data-live','1');\
         b.appendChild(document.createTextNode('ok'));\
         b.addEventListener('click',function(){this.setAttribute('data-hit','yes');});\
         b.click();let fresh=document.getElementById('b');\
         fresh.getAttribute('data-live')+':'\
         +fresh.getAttribute('data-hit')+':'+fresh.textContent;",
    );

    assert_eq!(value, JsValue::String("1:yes:ok".into()));
}

#[test]
fn shifted_element_keeps_existing_event_registrations() {
    let value = eval(
        "let a=document.getElementById('a');let b=document.getElementById('b');\
         b.addEventListener('click',function(){this.setAttribute('data-listener','yes');});\
         b.onclick=function(){this.setAttribute('data-handler','yes');};\
         a.remove();b.click();let fresh=document.getElementById('b');\
         fresh.getAttribute('data-listener')+':'\
         +fresh.getAttribute('data-handler');",
    );

    assert_eq!(value, JsValue::String("yes:yes".into()));
}

#[test]
fn inserted_detached_element_keeps_handle_and_event_registrations() {
    let value = eval(
        "let app=document.querySelector('main');let made=document.createElement('button');\
         made.setAttribute('id','made');\
         made.addEventListener('click',function(){this.setAttribute('data-hit','yes');});\
         app.appendChild(made);made.click();let fresh=document.getElementById('made');\
         fresh.getAttribute('data-hit')+':'+made.getAttribute('data-hit');",
    );

    assert_eq!(value, JsValue::String("yes:yes".into()));
}
