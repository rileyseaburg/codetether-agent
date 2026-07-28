use super::*;

#[test]
fn pointer_lock_api_updates_state_events_and_errors() {
    let result = eval_with_dom(
        "<div id='box'></div>",
        "function pl(){if(document.pointerLockElement){return document.pointerLockElement.id;}return 'null';}\
         function on_change(e){seen=seen+'C:'+pl()+':'+e.target.id+'|';}\
         function on_error(e){seen=seen+'X:'+e.type+'|';}\
         let box=document.getElementById('box');\
         let seen='';\
         document.addEventListener('pointerlockchange', on_change);\
         document.addEventListener('pointerlockerror', on_error);\
         let shape=typeof box.requestPointerLock+':'+typeof document.exitPointerLock;\
         box.requestPointerLock();\
         box.requestPointerLock();\
         document.exitPointerLock();\
         box.remove();\
         box.requestPointerLock();\
         shape+';'+seen+';'+document.pointerLockElement;",
    )
    .unwrap();
    assert_eq!(
        result.value,
        JsValue::String("function:function;C:box:box|C:null:box|X:pointerlockerror|;null".into())
    );
}
