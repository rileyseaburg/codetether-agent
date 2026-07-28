use super::*;

#[test]
fn dialog_methods_reflect_state_return_value_and_events() {
    let result = eval_with_dom(
        "<dialog id='d'></dialog>",
        "let d=document.getElementById('d'); let seen=''; d.addEventListener('close',function(e){seen=seen+'L'+this.open+':'+this.returnValue;}); d.onclose=function(e){seen=seen+'H'+e.type;}; d.oncancel=function(e){seen=seen+'K'+e.type; e.preventDefault();}; let canceled=d.dispatchEvent({type:'cancel'}); d.showModal(); let shown=d.hasAttribute('open'); d.close('done'); let fresh=document.getElementById('d'); canceled + ':' + shown + ':' + fresh.hasAttribute('open') + ':' + fresh.returnValue + ':' + seen;",
    )
    .unwrap();
    assert_eq!(
        result.value,
        JsValue::String("false:true:false:done:KcancelLfalse:doneHclose".into())
    );
}

#[test]
fn popover_methods_toggle_reflected_open_state() {
    let result = eval_with_dom(
        "<div id='p' popover></div>",
        "let p=document.getElementById('p'); let before=p.popoverOpen+':'+p.hasAttribute('popover-open'); p.popoverOpen=true; let assigned=p.hasAttribute('popover-open'); p.hidePopover(); let hidden=p.hasAttribute('popover-open'); p.showPopover(); let shown=p.hasAttribute('popover-open'); let toggled=p.togglePopover(); let forced=p.togglePopover(true); let fresh=document.getElementById('p'); before+'|'+assigned+':'+hidden+':'+shown+':'+toggled+':'+forced+':'+fresh.popoverOpen+':'+fresh.hasAttribute('popover-open');",
    )
    .unwrap();
    assert_eq!(
        result.value,
        JsValue::String("false:false|true:false:true:false:true:true:true".into())
    );
}
