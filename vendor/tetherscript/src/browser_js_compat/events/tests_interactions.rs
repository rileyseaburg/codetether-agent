use super::*;

#[test]
fn interaction_event_constructors_default_fields() {
    let result = eval_with_dom(
        "",
        "let d=DragEvent('drag');let c=CompositionEvent('compositionstart');\
         let t=TouchEvent('touchstart');\
         typeof DragEvent+':'+typeof CompositionEvent+':'+typeof TouchEvent+':' +\
         (d.dataTransfer===null)+':'+d.clientX+','+d.button+','+d.altKey +\
         ':'+c.data+':'+t.touches.length+','+t.targetTouches.length+',' +\
         t.changedTouches.length+','+t.altKey+','+t.ctrlKey+',' +\
         t.metaKey+','+t.shiftKey;",
    )
    .unwrap();
    assert_eq!(
        result.value.display(),
        "function:function:function:true:0,0,false::0,0,0,false,false,false,false"
    );
}

#[test]
fn interaction_event_constructors_read_init_fields() {
    let result = eval_with_dom(
        "",
        "let dt={kind:'dt'};let touches=[{id:1}];let changed=[{id:2}];\
         let d=DragEvent('drag',{clientX:9,button:1,dataTransfer:dt,ctrlKey:true});\
         let c=CompositionEvent('compositionupdate',{data:'ime'});\
         let t=TouchEvent('touchmove',{touches:touches,changedTouches:changed,\
         altKey:true,shiftKey:true});\
         d.clientX+':'+d.button+':'+d.dataTransfer.kind+':'+d.ctrlKey+':' +\
         c.data+':'+t.touches[0].id+','+t.targetTouches.length+',' +\
         t.changedTouches[0].id+','+t.altKey+','+t.shiftKey;",
    )
    .unwrap();
    assert_eq!(result.value.display(), "9:1:dt:true:ime:1,0,2,true,true");
}

#[test]
fn drag_event_dispatch_preserves_fields() {
    let result = eval_with_dom(
        "<button id='drop'></button>",
        "let b=document.getElementById('drop');let seen='';\
         b.addEventListener('drop',function(e){\
         seen=e.type+':'+e.target.id+':'+e.dataTransfer.text+':' +\
         e.clientX+':'+e.bubbles;});\
         let ok=b.dispatchEvent(DragEvent('drop',{bubbles:true,\
         dataTransfer:{text:'payload'},clientX:12}));seen+':'+ok;",
    )
    .unwrap();
    assert_eq!(result.value.display(), "drop:drop:payload:12:true:true");
}
