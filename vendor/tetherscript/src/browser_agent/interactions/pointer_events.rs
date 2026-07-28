//! Pointer event snippets with local capture bookkeeping.

use crate::browser_agent::action::BoundingBox;
use crate::browser_agent::interact::pointer_event_fields as fields;
use crate::browser_agent::keyboard_escape::node;

pub(crate) fn helpers() -> &'static str {
    "let __cap=null;function __pe(n,t,b){let e={type:t,button:0,buttons:b,\
     detail:0,clientX:0,clientY:0,pageX:0,pageY:0,screenX:0,screenY:0,\
     pointerId:1,pointerType:'mouse',isPrimary:true,width:1,height:1,\
     pressure:b,tangentialPressure:0,tiltX:0,tiltY:0,twist:0,\
     setPointerCapture:function(){__cap=n;},\
     releasePointerCapture:function(){if(__cap==n){__cap=null;}},\
     hasPointerCapture:function(){return __cap==n;}};return n.dispatchEvent(e);}\
     function __pt(f){return __cap||f;}"
}

pub(crate) fn dispatch(path: &[usize], event: &str, buttons: i64, bounds: BoundingBox) -> String {
    format!(
        "{}.dispatchEvent({{type:'{event}',{}}});",
        node(path),
        fields::pointer(buttons, bounds)
    )
}

pub(crate) fn mouse(path: &[usize], event: &str, buttons: i64, bounds: BoundingBox) -> String {
    format!(
        "{}.dispatchEvent({{type:'{event}',{}}});",
        node(path),
        fields::mouse(buttons, 0, bounds)
    )
}
