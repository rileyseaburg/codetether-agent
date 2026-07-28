//! Shared JavaScript event field literals for pointer-like actions.

use crate::browser_agent::action::BoundingBox;

pub(crate) fn mouse(buttons: i64, detail: i64, bounds: BoundingBox) -> String {
    let (x, y) = center(bounds);
    format!(
        "button:0,buttons:{buttons},detail:{detail},clientX:{x},clientY:{y},\
         screenX:{x},screenY:{y},pageX:{x},pageY:{y},x:{x},y:{y},\
         offsetX:0,offsetY:0,movementX:0,movementY:0"
    )
}

pub(crate) fn pointer(buttons: i64, bounds: BoundingBox) -> String {
    let pressure = if buttons == 0 { "0" } else { "0.5" };
    format!(
        "{},pointerId:1,width:1,height:1,pressure:{pressure},\
         tangentialPressure:0,tiltX:0,tiltY:0,twist:0,\
         pointerType:'mouse',isPrimary:true",
        mouse(buttons, 0, bounds)
    )
}

pub(crate) fn wheel(delta_x: i64, delta_y: i64, bounds: BoundingBox) -> String {
    format!(
        "{},deltaX:{delta_x},deltaY:{delta_y},deltaZ:0,deltaMode:0,\
         ctrlKey:false,shiftKey:false,altKey:false,metaKey:false",
        mouse(0, 0, bounds)
    )
}

pub(crate) fn touch(bounds: BoundingBox) -> String {
    let (x, y) = center(bounds);
    format!(
        "identifier:1,target:n,clientX:{x},clientY:{y},screenX:{x},screenY:{y},\
         pageX:{x},pageY:{y},radiusX:1,radiusY:1,rotationAngle:0,force:0.5"
    )
}

fn center(bounds: BoundingBox) -> (i64, i64) {
    (bounds.x + bounds.width / 2, bounds.y + bounds.height / 2)
}
