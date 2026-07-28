use tetherscript::browser_js::eval_with_dom;
use tetherscript::js::JsValue;

fn eval(script: &str) -> JsValue {
    eval_with_dom("<div id='p'><button id='c'>Go</button></div>", script)
        .unwrap()
        .value
}

#[test]
fn stop_propagation_preserves_same_capture_target_listeners() {
    let value = eval(
        "let p=document.getElementById('p');let c=document.getElementById('c');\
         let seen='';\
         p.addEventListener('click',function(e){seen=seen+'a';e.stopPropagation();},true);\
         p.addEventListener('click',function(){seen=seen+'b';},true);\
         c.addEventListener('click',function(){seen=seen+'target';});\
         p.addEventListener('click',function(){seen=seen+'bubble';});\
         c.dispatchEvent({type:'click',bubbles:true});seen;",
    );

    assert_eq!(value, JsValue::String("ab".into()));
}

#[test]
fn stop_immediate_propagation_skips_same_capture_target_listeners() {
    let value = eval(
        "let p=document.getElementById('p');let c=document.getElementById('c');\
         let seen='';\
         p.addEventListener('click',function(e){seen=seen+'a';e.stopImmediatePropagation();},true);\
         p.addEventListener('click',function(){seen=seen+'b';},true);\
         c.dispatchEvent({type:'click',bubbles:true});seen;",
    );

    assert_eq!(value, JsValue::String("a".into()));
}

#[test]
fn stop_propagation_preserves_same_bubble_target_listeners() {
    let value = eval(
        "let p=document.getElementById('p');let c=document.getElementById('c');\
         let seen='';\
         c.addEventListener('click',function(){seen=seen+'target';});\
         p.addEventListener('click',function(e){seen=seen+'a';e.stopPropagation();});\
         p.addEventListener('click',function(){seen=seen+'b';});\
         c.dispatchEvent({type:'click',bubbles:true});seen;",
    );

    assert_eq!(value, JsValue::String("targetab".into()));
}
