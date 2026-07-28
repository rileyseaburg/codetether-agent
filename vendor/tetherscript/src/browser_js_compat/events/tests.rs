use super::*;

#[test]
fn event_constructors_initialize_common_and_specific_fields() {
    let result = eval_with_dom(
        "<button id='go'></button>",
        "let b=document.getElementById('go');\
         let e=Event('ready',{bubbles:true,cancelable:true,composed:true});\
         e.preventDefault();\
         let c=CustomEvent('custom',{detail:'payload'});\
         let m=MouseEvent('click',{clientX:7,clientY:8,button:1,buttons:2}); let k=KeyboardEvent('keydown',{key:'A',code:'KeyA',repeat:true});\
         let i=InputEvent('input',{data:'x',inputType:'insertText'}); let s=SubmitEvent('submit',{submitter:b}); let f=FocusEvent('focus',{relatedTarget:b});\
         let p=PointerEvent('pointerdown',{pointerId:5,pointerType:'mouse'}); let w=WheelEvent('wheel',{deltaX:1,deltaY:-2});\
         typeof Event + ':' + e.type + ':' + e.bubbles + ':' + e.cancelable + ':' +\
         e.composed + ':' + e.defaultPrevented + ':' + typeof e.timeStamp + ':' +\
         e.timeStamp + ':' + e.composedPath().length + ':' + c.detail + ':' +\
         typeof c.timeStamp + ':' + c.timeStamp + ':' +\
         m.clientX + ',' + m.clientY + ',' + m.button + ',' +\
         m.buttons + ':' + k.key + ',' + k.code + ',' + k.repeat + ':' +\
         i.data + ',' + i.inputType + ':' + s.submitter.id + ':' +\
         f.relatedTarget.id + ':' + p.pointerId + ',' + p.pointerType + ':' +\
         w.deltaX + ',' + w.deltaY;",
    )
    .unwrap();
    assert_eq!(
        result.value,
        JsValue::String(
            "function:ready:true:true:true:true:number:0:0:payload:number:0:7,8,1,2:A,KeyA,true:x,insertText:go:go:5,mouse:1,-2"
                .into()
        )
    );
}

#[test]
fn constructed_events_dispatch_with_fields_and_cancellation() {
    let result = eval_with_dom(
        "<div id='p'><button id='go'></button></div>",
        "let p=document.getElementById('p'); let b=document.getElementById('go');\
         let seen=''; p.addEventListener('click',function(){seen=seen+'parent';});\
         b.addEventListener('click',function(e){seen=e.type+':'+e.target.id+':' +\
         e.currentTarget.id+':'+e.detail.msg+':'+e.bubbles+':'+e.cancelable+':' +\
         e.composed+':'+typeof e.timeStamp+','+e.timeStamp+':'; e.preventDefault(); e.stopPropagation();});\
         let ok=b.dispatchEvent(CustomEvent('click',{bubbles:true,cancelable:true,composed:true,detail:{msg:'payload'}}));\
         seen + ok;",
    )
    .unwrap();
    assert_eq!(
        result.value.display(),
        "click:go:go:payload:true:true:true:number,0:false"
    );
}
