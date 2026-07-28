use super::*;

#[test]
fn storage_event_defaults_are_nullable() {
    let result = eval_with_dom(
        "",
        "let e=StorageEvent('storage'); let c=ClipboardEvent('copy');\
         e.type+':'+(e.key===null)+':'+(e.oldValue===null)+':' +\
         (e.newValue===null)+':'+e.url+':'+(e.storageArea===null)+':' +\
         (c.clipboardData===null)+':'+typeof e.preventDefault+':' +\
         e.defaultPrevented;",
    )
    .unwrap();
    assert_eq!(
        result.value,
        JsValue::String("storage:true:true:true::true:true:function:false".into())
    );
}

#[test]
fn storage_and_clipboard_events_read_init_fields() {
    let result = eval_with_dom(
        "",
        "let area={name:'local'}; let data={text:'copied'};\
         let s=StorageEvent('storage',{key:'k',oldValue:'a',newValue:'b',\
         url:'https://example.test/',storageArea:area,bubbles:true});\
         let c=ClipboardEvent('copy',{clipboardData:data});\
         s.key+':'+s.oldValue+':'+s.newValue+':'+s.url+':' +\
         s.storageArea.name+':'+s.bubbles+':'+c.clipboardData.text;",
    )
    .unwrap();
    assert_eq!(
        result.value,
        JsValue::String("k:a:b:https://example.test/:local:true:copied".into())
    );
}

#[test]
fn clipboard_event_dispatch_preserves_fields() {
    let result = eval_with_dom(
        "<input id='clip'>",
        "let input=document.getElementById('clip'); let seen='';\
         input.addEventListener('paste',function(e){\
         seen=e.type+':'+e.target.id+':'+e.clipboardData.text;});\
         let ok=input.dispatchEvent(ClipboardEvent('paste',{clipboardData:{text:'p'}}));\
         seen+':'+ok;",
    )
    .unwrap();
    assert_eq!(result.value, JsValue::String("paste:clip:p:true".into()));
}
