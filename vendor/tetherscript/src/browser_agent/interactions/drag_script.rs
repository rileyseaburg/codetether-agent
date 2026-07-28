//! Drag-and-drop JavaScript sequence assembly.

use crate::browser_agent::action::BoundingBox;
use crate::browser_agent::interact::pointer_event_fields as fields;
use crate::browser_agent::keyboard_escape::node;

use super::{data_transfer, pointer_events};

pub(crate) fn drag(
    source: &[usize],
    target: &[usize],
    source_bounds: BoundingBox,
    target_bounds: BoundingBox,
) -> String {
    format!(
        "let s={};let t={};{}{}__pe(s,'pointerover',0);__pe(s,'pointerdown',1);\
         s.dispatchEvent({{type:'mousedown',{}}});\
         let started=s.dispatchEvent({{type:'dragstart',dataTransfer:dt,{}}});\
         if(started){{__pe(__pt(t),'pointermove',1);t.dispatchEvent({{type:'dragenter',\
         dataTransfer:dt,{}}});t.dispatchEvent({{type:'dragover',\
         dataTransfer:dt,{}}});let ok=t.dispatchEvent({{type:'drop',\
         dataTransfer:dt,{}}});if(ok){{__dropDefault(t,dt);}}}}\
         __pt(s).dispatchEvent({{type:'mouseup',{}}});__pe(__pt(s),'pointerup',0);\
         s.dispatchEvent({{type:'dragend',dataTransfer:dt,{}}});",
        node(source),
        node(target),
        data_transfer::create("s.innerText"),
        helpers(),
        fields::mouse(1, 1, source_bounds),
        fields::mouse(1, 1, source_bounds),
        fields::mouse(1, 0, target_bounds),
        fields::mouse(1, 0, target_bounds),
        fields::mouse(0, 0, target_bounds),
        fields::mouse(0, 1, source_bounds),
        fields::mouse(0, 0, source_bounds)
    )
}

fn helpers() -> String {
    format!("{}{}", pointer_events::helpers(), drop_default())
}

fn drop_default() -> &'static str {
    "function __dropDefault(n,d){let x=d.getData('text/plain');\
     if((n.tagName=='INPUT')||(n.tagName=='TEXTAREA')){n.value=x;\
     n.dispatchEvent({type:'input'});n.dispatchEvent({type:'change'});}\
     else if(n.getAttribute&&n.getAttribute('contenteditable')=='true'){n.textContent=x;\
     n.dispatchEvent({type:'input'});}}"
}
