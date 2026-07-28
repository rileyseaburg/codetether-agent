use super::super::*;

#[test]
fn device_media_events_read_init_fields() {
    let result = eval_with_dom(
        "",
        "let a={x:1};let g={y:2};let r={alpha:3};\
         let q=MediaQueryListEvent('change',{matches:true,media:'screen'});\
         let o=DeviceOrientationEvent('deviceorientation',{alpha:1,beta:2,\
         gamma:3,absolute:true,bubbles:true});\
         let m=DeviceMotionEvent('devicemotion',{acceleration:a,\
         accelerationIncludingGravity:g,rotationRate:r,interval:16});\
         q.matches+':'+q.media+':'+o.alpha+','+o.beta+','+o.gamma+',' +\
         o.absolute+','+o.bubbles+'|'+m.acceleration.x+',' +\
         m.accelerationIncludingGravity.y+','+m.rotationRate.alpha+',' +\
         m.interval;",
    )
    .unwrap();
    assert_eq!(
        result.value.display(),
        "true:screen:1,2,3,true,true|1,2,3,16"
    );
}

#[test]
fn device_orientation_dispatch_preserves_event_shape() {
    let result = eval_with_dom(
        "<div id='sensor'></div>",
        "let el=document.getElementById('sensor');let seen='';\
         el.addEventListener('deviceorientation',function(e){\
         seen=e.type+':'+e.target.id+':'+e.alpha+':'+e.cancelable;});\
         let ok=el.dispatchEvent(DeviceOrientationEvent('deviceorientation',\
         {alpha:90,cancelable:true}));seen+':'+ok;",
    )
    .unwrap();
    assert_eq!(
        result.value.display(),
        "deviceorientation:sensor:90:true:true"
    );
}
